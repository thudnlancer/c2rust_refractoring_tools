#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut CONVFMT: *const libc::c_char;
    static mut CONVFMTidx: libc::c_int;
    static mut Nnull_string: *mut NODE;
    static mut source: *mut libc::c_char;
    static mut interpret: Option::<
        unsafe extern "C" fn(*mut INSTRUCTION) -> libc::c_int,
    >;
    static mut make_number: Option::<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option::<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option::<
        unsafe extern "C" fn(*const libc::c_char, libc::c_int, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut do_flags: do_flag_values;
    static awk_namespace: [libc::c_char; 0];
    fn r_unref(tmp: *mut NODE);
    fn null_array(symbol: *mut NODE);
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const libc::c_char,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn update_ERRNO_int(_: libc::c_int);
    fn update_ERRNO_string(string: *const libc::c_char);
    fn unset_ERRNO();
    fn flags2str(_: libc::c_int) -> *const libc::c_char;
    fn is_valid_identifier(name: *const libc::c_char) -> bool;
    fn make_builtin(
        name_space: *const libc::c_char,
        _: *const awk_ext_func_t,
    ) -> awk_bool_t;
    fn get_argument(_: libc::c_int) -> *mut NODE;
    fn get_actual_argument(_: *mut NODE, _: libc::c_int, _: bool) -> *mut NODE;
    fn redirect_string(
        redir_exp_str: *const libc::c_char,
        redir_exp_len: size_t,
        not_string_flag: bool,
        redirtype: libc::c_int,
        errflg: *mut libc::c_int,
        extfd: libc::c_int,
        failure_fatal: bool,
    ) -> *mut redirect;
    fn r_warning(mesg: *const libc::c_char, _: ...);
    fn set_loc(file: *const libc::c_char, line: libc::c_int);
    fn after_beginfile(curfile_0: *mut *mut IOBUF);
    fn r_fatal(mesg: *const libc::c_char, _: ...);
    fn nextfile(curfile_0: *mut *mut IOBUF, skipping: bool) -> libc::c_int;
    fn more_blocks(id: libc::c_int) -> *mut libc::c_void;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn make_typed_regex(re: *const libc::c_char, len: size_t) -> *mut NODE;
    fn make_str_node(
        s: *const libc::c_char,
        len: size_t,
        flags: libc::c_int,
    ) -> *mut NODE;
    fn make_bool_node(value: bool) -> *mut NODE;
    fn estrdup(str: *const libc::c_char, len: size_t) -> *mut libc::c_char;
    fn r_free_wstr(n: *mut NODE);
    fn mpfr_unset(n: *mut NODE);
    fn update_global_values();
    fn is_off_limits_var(var: *const libc::c_char) -> libc::c_int;
    fn install_symbol(name: *const libc::c_char, type_0: NODETYPE) -> *mut NODE;
    fn lookup(name: *const libc::c_char) -> *mut NODE;
    fn err(
        isfatal: bool,
        s: *const libc::c_char,
        emsg: *const libc::c_char,
        argp: ::core::ffi::VaList,
    );
    static mut lintfunc: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
    fn register_two_way_processor(processor: *mut awk_two_way_processor_t);
    fn register_output_wrapper(wrapper: *mut awk_output_wrapper_t);
    fn register_input_parser(input_parser: *mut awk_input_parser_t);
    static mut curfile: *mut IOBUF;
    static mut main_beginfile: *mut INSTRUCTION;
    static mut currule: libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
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
pub type __re_size_t = libc::c_uint;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub has_meta: bool,
    pub maybe_long: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_bool {
    awk_false = 0,
    awk_true,
impl awk_bool {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            awk_bool::awk_false => 0,
            awk_bool::awk_true => 1,
        }
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
    pub name: *const libc::c_char,
    pub fd: libc::c_int,
    pub opaque: *mut libc::c_void,
    pub get_record: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *mut awk_input,
            *mut libc::c_int,
            *mut *mut libc::c_char,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> libc::c_int,
    >,
    pub read_func: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option::<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input_parser {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const libc::c_char,
    pub mode: *const libc::c_char,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option::<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_ferror: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
    pub gawk_fclose: Option::<
        unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> libc::c_int,
    >,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const libc::c_char,
    pub can_take_file: Option::<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *mut awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const libc::c_char,
    pub can_take_two_way: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> awk_bool_t,
    >,
    pub take_control_of: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *mut awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    GAWK_API_MINOR_VERSION = 2,
    GAWK_API_MAJOR_VERSION = 3,
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::GAWK_API_MINOR_VERSION => 2,
            C2RustUnnamed::GAWK_API_MAJOR_VERSION => 3,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
impl AWK_NUMBER_TYPE {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE => 0,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR => 1,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ => 2,
        }
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
impl awk_valtype_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
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
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_1::AWK_ELEMENT_DELETE => 1,
            C2RustUnnamed_1::AWK_ELEMENT_DEFAULT => 0,
        }
    }
}

pub type awk_element_t = awk_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_flat_array {
    pub opaque1: *mut libc::c_void,
    pub opaque2: *mut libc::c_void,
    pub count: size_t,
    pub elements: [awk_element_t; 1],
}
pub type awk_flat_array_t = awk_flat_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const libc::c_char,
    pub function: Option::<
        unsafe extern "C" fn(
            libc::c_int,
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
    pub major_version: libc::c_int,
    pub minor_version: libc::c_int,
    pub gmp_major_version: libc::c_int,
    pub gmp_minor_version: libc::c_int,
    pub mpfr_major_version: libc::c_int,
    pub mpfr_minor_version: libc::c_int,
    pub do_flags: [libc::c_int; 6],
    pub api_add_ext_func: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *mut awk_ext_func_t,
        ) -> awk_bool_t,
    >,
    pub api_register_input_parser: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t) -> (),
    >,
    pub api_register_output_wrapper: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_output_wrapper_t) -> (),
    >,
    pub api_register_two_way_processor: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_two_way_processor_t) -> (),
    >,
    pub api_awk_atexit: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
    pub api_register_ext_version: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_fatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_warning: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_lintwarn: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_nonfatal: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
    >,
    pub api_update_ERRNO_int: Option::<
        unsafe extern "C" fn(awk_ext_id_t, libc::c_int) -> (),
    >,
    pub api_update_ERRNO_string: Option::<
        unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
    >,
    pub api_unset_ERRNO: Option::<unsafe extern "C" fn(awk_ext_id_t) -> ()>,
    pub api_get_argument: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            size_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_argument: Option::<
        unsafe extern "C" fn(awk_ext_id_t, size_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_sym_lookup: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            *const libc::c_char,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_lookup_scalar: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_scalar_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update_scalar: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_scalar_t, *mut awk_value_t) -> awk_bool_t,
    >,
    pub api_create_value: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *mut awk_value_t,
            *mut awk_value_cookie_t,
        ) -> awk_bool_t,
    >,
    pub api_release_value: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_value_cookie_t) -> awk_bool_t,
    >,
    pub api_get_element_count: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *mut size_t) -> awk_bool_t,
    >,
    pub api_get_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_array_element: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            *const awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_del_array_element: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *const awk_value_t) -> awk_bool_t,
    >,
    pub api_create_array: Option::<unsafe extern "C" fn(awk_ext_id_t) -> awk_array_t>,
    pub api_clear_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_flatten_array_typed: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut *mut awk_flat_array_t,
            awk_valtype_t,
            awk_valtype_t,
        ) -> awk_bool_t,
    >,
    pub api_release_flattened_array: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut awk_flat_array_t,
        ) -> awk_bool_t,
    >,
    pub api_malloc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub api_calloc: Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub api_realloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub api_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub api_get_mpfr: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_mpz: Option::<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_file: Option::<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            libc::c_int,
            *mut *const awk_input_buf_t,
            *mut *const awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub api_destroy_array: Option::<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
}
pub type gawk_api_t = gawk_api;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nodevals {
    Node_illegal,
    Node_val,
    Node_regex,
    Node_dynregex,
    Node_var,
    Node_var_array,
    Node_var_new,
    Node_elem_new,
    Node_param_list,
    Node_func,
    Node_ext_func,
    Node_builtin_func,
    Node_array_ref,
    Node_array_tree,
    Node_array_leaf,
    Node_dump_array,
    Node_arrayfor,
    Node_frame,
    Node_instruction,
    Node_final,
impl nodevals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            nodevals::Node_illegal => 0,
            nodevals::Node_val => 1,
            nodevals::Node_regex => 2,
            nodevals::Node_dynregex => 3,
            nodevals::Node_var => 4,
            nodevals::Node_var_array => 5,
            nodevals::Node_var_new => 6,
            nodevals::Node_elem_new => 7,
            nodevals::Node_param_list => 8,
            nodevals::Node_func => 9,
            nodevals::Node_ext_func => 10,
            nodevals::Node_builtin_func => 11,
            nodevals::Node_array_ref => 12,
            nodevals::Node_array_tree => 13,
            nodevals::Node_array_leaf => 14,
            nodevals::Node_dump_array => 15,
            nodevals::Node_arrayfor => 16,
            nodevals::Node_frame => 17,
            nodevals::Node_instruction => 18,
            nodevals::Node_final => 19,
        }
    }
}

pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_2,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: libc::c_long,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flagvals {
    REGEX = 524288,
    NUMCONSTSTR = 262144,
    XARRAY = 131072,
    HALFHAT = 65536,
    ARRAYMAXED = 32768,
    NULL_FIELD = 16384,
    NO_EXT_SET = 8192,
    MPZN = 4096,
    MPFN = 2048,
    WSTRCUR = 1024,
    INTIND = 512,
    NUMINT = 256,
    INTLSTR = 128,
    BOOLVAL = 64,
    USER_INPUT = 32,
    NUMBER = 16,
    NUMCUR = 8,
    STRCUR = 4,
    STRING = 2,
    MALLOC = 1,
impl flagvals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            flagvals::REGEX => 524288,
            flagvals::NUMCONSTSTR => 262144,
            flagvals::XARRAY => 131072,
            flagvals::HALFHAT => 65536,
            flagvals::ARRAYMAXED => 32768,
            flagvals::NULL_FIELD => 16384,
            flagvals::NO_EXT_SET => 8192,
            flagvals::MPZN => 4096,
            flagvals::MPFN => 2048,
            flagvals::WSTRCUR => 1024,
            flagvals::INTIND => 512,
            flagvals::NUMINT => 256,
            flagvals::INTLSTR => 128,
            flagvals::BOOLVAL => 64,
            flagvals::USER_INPUT => 32,
            flagvals::NUMBER => 16,
            flagvals::NUMCUR => 8,
            flagvals::STRCUR => 4,
            flagvals::STRING => 2,
            flagvals::MALLOC => 1,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub nodep: C2RustUnnamed_4,
    pub val: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fltnum: libc::c_double,
    pub sp: *mut libc::c_char,
    pub slen: size_t,
    pub idx: libc::c_int,
    pub wsp: *mut wchar_t,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum commenttype {
    FOR_COMMENT = 3,
    BLOCK_COMMENT = 2,
    EOL_COMMENT = 1,
impl commenttype {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            commenttype::FOR_COMMENT => 3,
            commenttype::BLOCK_COMMENT => 2,
            commenttype::EOL_COMMENT => 1,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub l: C2RustUnnamed_11,
    pub r: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
    pub name: *mut libc::c_char,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: libc::c_ulong,
    pub reflags: reflagvals,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
impl reflagvals {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            reflagvals::CONSTANT => 1,
            reflagvals::FS_DFLT => 2,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub extra: *mut exp_node,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xl: libc::c_long,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option::<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2RustUnnamed_8,
    pub x: C2RustUnnamed_7,
    pub comment: *mut exp_instruction,
    pub source_line: libc::c_short,
    pub pool_size: libc::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum opcodeval {
    Op_final = 122,
    Op_parens = 121,
    Op_cond_exp = 120,
    Op_K_function = 119,
    Op_K_else = 118,
    Op_K_if = 117,
    Op_K_switch = 116,
    Op_K_while = 115,
    Op_K_arrayfor = 114,
    Op_K_for = 113,
    Op_K_do = 112,
    Op_list = 111,
    Op_symbol = 110,
    Op_token = 109,
    Op_stop = 108,
    Op_atexit = 107,
    Op_lint_plus = 106,
    Op_lint = 105,
    Op_breakpoint = 104,
    Op_exec_count = 103,
    Op_comment = 102,
    Op_func = 101,
    Op_after_endfile = 100,
    Op_after_beginfile = 99,
    Op_subscript_assign = 98,
    Op_field_assign = 97,
    Op_var_assign = 96,
    Op_var_update = 95,
    Op_arrayfor_final = 94,
    Op_arrayfor_incr = 93,
    Op_arrayfor_init = 92,
    Op_newfile = 91,
    Op_get_record = 90,
    Op_jmp_false = 89,
    Op_jmp_true = 88,
    Op_jmp = 87,
    Op_pop = 86,
    Op_no_op = 85,
    Op_field_spec_lhs = 84,
    Op_subscript_lhs = 83,
    Op_push_lhs = 82,
    Op_push_param = 81,
    Op_push_array = 80,
    Op_push_re = 79,
    Op_push_i = 78,
    Op_push_arg_untyped = 77,
    Op_push_arg = 76,
    Op_push = 75,
    Op_indirect_func_call = 74,
    Op_func_call = 73,
    Op_in_array = 72,
    Op_ext_builtin = 71,
    Op_sub_builtin = 70,
    Op_builtin = 69,
    Op_K_namespace = 68,
    Op_K_nextfile = 67,
    Op_K_getline = 66,
    Op_K_getline_redir = 65,
    Op_K_delete_loop = 64,
    Op_K_delete = 63,
    Op_K_return_from_eval = 62,
    Op_K_return = 61,
    Op_K_exit = 60,
    Op_K_next = 59,
    Op_K_printf = 58,
    Op_K_print_rec = 57,
    Op_K_print = 56,
    Op_K_continue = 55,
    Op_K_break = 54,
    Op_K_default = 53,
    Op_K_case = 52,
    Op_rule = 51,
    Op_nomatch = 50,
    Op_match_rec = 49,
    Op_match = 48,
    Op_geq = 47,
    Op_leq = 46,
    Op_greater = 45,
    Op_less = 44,
    Op_notequal = 43,
    Op_equal = 42,
    Op_or_final = 41,
    Op_or = 40,
    Op_and_final = 39,
    Op_and = 38,
    Op_assign_concat = 37,
    Op_assign_exp = 36,
    Op_assign_minus = 35,
    Op_assign_plus = 34,
    Op_assign_mod = 33,
    Op_assign_quotient = 32,
    Op_assign_times = 31,
    Op_store_field_exp = 30,
    Op_store_field = 29,
    Op_store_sub = 28,
    Op_store_var = 27,
    Op_assign = 26,
    Op_not = 25,
    Op_field_spec = 24,
    Op_unary_plus = 23,
    Op_unary_minus = 22,
    Op_postdecrement = 21,
    Op_postincrement = 20,
    Op_predecrement = 19,
    Op_preincrement = 18,
    Op_sub_array = 17,
    Op_subscript = 16,
    Op_cond_pair = 15,
    Op_line_range = 14,
    Op_concat = 13,
    Op_exp_i = 12,
    Op_exp = 11,
    Op_minus_i = 10,
    Op_minus = 9,
    Op_plus_i = 8,
    Op_plus = 7,
    Op_mod_i = 6,
    Op_mod = 5,
    Op_quotient_i = 4,
    Op_quotient = 3,
    Op_times_i = 2,
    Op_times = 1,
    Op_illegal = 0,
impl opcodeval {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            opcodeval::Op_final => 122,
            opcodeval::Op_parens => 121,
            opcodeval::Op_cond_exp => 120,
            opcodeval::Op_K_function => 119,
            opcodeval::Op_K_else => 118,
            opcodeval::Op_K_if => 117,
            opcodeval::Op_K_switch => 116,
            opcodeval::Op_K_while => 115,
            opcodeval::Op_K_arrayfor => 114,
            opcodeval::Op_K_for => 113,
            opcodeval::Op_K_do => 112,
            opcodeval::Op_list => 111,
            opcodeval::Op_symbol => 110,
            opcodeval::Op_token => 109,
            opcodeval::Op_stop => 108,
            opcodeval::Op_atexit => 107,
            opcodeval::Op_lint_plus => 106,
            opcodeval::Op_lint => 105,
            opcodeval::Op_breakpoint => 104,
            opcodeval::Op_exec_count => 103,
            opcodeval::Op_comment => 102,
            opcodeval::Op_func => 101,
            opcodeval::Op_after_endfile => 100,
            opcodeval::Op_after_beginfile => 99,
            opcodeval::Op_subscript_assign => 98,
            opcodeval::Op_field_assign => 97,
            opcodeval::Op_var_assign => 96,
            opcodeval::Op_var_update => 95,
            opcodeval::Op_arrayfor_final => 94,
            opcodeval::Op_arrayfor_incr => 93,
            opcodeval::Op_arrayfor_init => 92,
            opcodeval::Op_newfile => 91,
            opcodeval::Op_get_record => 90,
            opcodeval::Op_jmp_false => 89,
            opcodeval::Op_jmp_true => 88,
            opcodeval::Op_jmp => 87,
            opcodeval::Op_pop => 86,
            opcodeval::Op_no_op => 85,
            opcodeval::Op_field_spec_lhs => 84,
            opcodeval::Op_subscript_lhs => 83,
            opcodeval::Op_push_lhs => 82,
            opcodeval::Op_push_param => 81,
            opcodeval::Op_push_array => 80,
            opcodeval::Op_push_re => 79,
            opcodeval::Op_push_i => 78,
            opcodeval::Op_push_arg_untyped => 77,
            opcodeval::Op_push_arg => 76,
            opcodeval::Op_push => 75,
            opcodeval::Op_indirect_func_call => 74,
            opcodeval::Op_func_call => 73,
            opcodeval::Op_in_array => 72,
            opcodeval::Op_ext_builtin => 71,
            opcodeval::Op_sub_builtin => 70,
            opcodeval::Op_builtin => 69,
            opcodeval::Op_K_namespace => 68,
            opcodeval::Op_K_nextfile => 67,
            opcodeval::Op_K_getline => 66,
            opcodeval::Op_K_getline_redir => 65,
            opcodeval::Op_K_delete_loop => 64,
            opcodeval::Op_K_delete => 63,
            opcodeval::Op_K_return_from_eval => 62,
            opcodeval::Op_K_return => 61,
            opcodeval::Op_K_exit => 60,
            opcodeval::Op_K_next => 59,
            opcodeval::Op_K_printf => 58,
            opcodeval::Op_K_print_rec => 57,
            opcodeval::Op_K_print => 56,
            opcodeval::Op_K_continue => 55,
            opcodeval::Op_K_break => 54,
            opcodeval::Op_K_default => 53,
            opcodeval::Op_K_case => 52,
            opcodeval::Op_rule => 51,
            opcodeval::Op_nomatch => 50,
            opcodeval::Op_match_rec => 49,
            opcodeval::Op_match => 48,
            opcodeval::Op_geq => 47,
            opcodeval::Op_leq => 46,
            opcodeval::Op_greater => 45,
            opcodeval::Op_less => 44,
            opcodeval::Op_notequal => 43,
            opcodeval::Op_equal => 42,
            opcodeval::Op_or_final => 41,
            opcodeval::Op_or => 40,
            opcodeval::Op_and_final => 39,
            opcodeval::Op_and => 38,
            opcodeval::Op_assign_concat => 37,
            opcodeval::Op_assign_exp => 36,
            opcodeval::Op_assign_minus => 35,
            opcodeval::Op_assign_plus => 34,
            opcodeval::Op_assign_mod => 33,
            opcodeval::Op_assign_quotient => 32,
            opcodeval::Op_assign_times => 31,
            opcodeval::Op_store_field_exp => 30,
            opcodeval::Op_store_field => 29,
            opcodeval::Op_store_sub => 28,
            opcodeval::Op_store_var => 27,
            opcodeval::Op_assign => 26,
            opcodeval::Op_not => 25,
            opcodeval::Op_field_spec => 24,
            opcodeval::Op_unary_plus => 23,
            opcodeval::Op_unary_minus => 22,
            opcodeval::Op_postdecrement => 21,
            opcodeval::Op_postincrement => 20,
            opcodeval::Op_predecrement => 19,
            opcodeval::Op_preincrement => 18,
            opcodeval::Op_sub_array => 17,
            opcodeval::Op_subscript => 16,
            opcodeval::Op_cond_pair => 15,
            opcodeval::Op_line_range => 14,
            opcodeval::Op_concat => 13,
            opcodeval::Op_exp_i => 12,
            opcodeval::Op_exp => 11,
            opcodeval::Op_minus_i => 10,
            opcodeval::Op_minus => 9,
            opcodeval::Op_plus_i => 8,
            opcodeval::Op_plus => 7,
            opcodeval::Op_mod_i => 6,
            opcodeval::Op_mod => 5,
            opcodeval::Op_quotient_i => 4,
            opcodeval::Op_quotient => 3,
            opcodeval::Op_times_i => 2,
            opcodeval::Op_times => 1,
            opcodeval::Op_illegal => 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub xl: libc::c_long,
    pub xn: *mut NODE,
    pub aptr: Option::<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option::<unsafe extern "C" fn(libc::c_int) -> *mut NODE>,
    pub efptr: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: libc::c_long,
    pub ldl: exec_count_t,
    pub name: *mut libc::c_char,
}
pub type exec_count_t = libc::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2RustUnnamed_10,
    pub hi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub li: [libc::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: libc::c_long,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const libc::c_char,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option::<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirval {
    redirect_twoway = 6,
    redirect_input = 5,
    redirect_pipein = 4,
    redirect_pipe = 3,
    redirect_append = 2,
    redirect_output = 1,
    redirect_none = 0,
impl redirval {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            redirval::redirect_twoway => 6,
            redirval::redirect_input => 5,
            redirval::redirect_pipein => 4,
            redirval::redirect_pipe => 3,
            redirval::redirect_append => 2,
            redirval::redirect_output => 1,
            redirval::redirect_none => 0,
        }
    }
}

pub type INSTRUCTION = exp_instruction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut libc::c_char,
    pub off: *mut libc::c_char,
    pub dataend: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: libc::c_int,
    pub flag: iobuf_flags,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum iobuf_flags {
    IOP_IS_TTY = 1,
    IOP_AT_EOF = 2,
    IOP_CLOSED = 4,
    IOP_AT_START = 8,
impl iobuf_flags {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            iobuf_flags::IOP_IS_TTY => 1,
            iobuf_flags::IOP_AT_EOF => 2,
            iobuf_flags::IOP_CLOSED => 4,
            iobuf_flags::IOP_AT_START => 8,
        }
    }
}

pub type IOBUF = iobuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub flag: redirect_flags,
    pub value: *mut libc::c_char,
    pub ifp: *mut FILE,
    pub iop: *mut IOBUF,
    pub pid: libc::c_int,
    pub status: libc::c_int,
    pub prev: *mut redirect,
    pub next: *mut redirect,
    pub mode: *const libc::c_char,
    pub output: awk_output_buf_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redirect_flags {
    RED_TCP = 2048,
    RED_SOCKET = 1024,
    RED_PTY = 512,
    RED_TWOWAY = 256,
    RED_EOF = 128,
    RED_USED = 64,
    RED_FLUSH = 32,
    RED_APPEND = 16,
    RED_WRITE = 8,
    RED_READ = 4,
    RED_PIPE = 2,
    RED_FILE = 1,
    RED_NONE = 0,
impl redirect_flags {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            redirect_flags::RED_TCP => 2048,
            redirect_flags::RED_SOCKET => 1024,
            redirect_flags::RED_PTY => 512,
            redirect_flags::RED_TWOWAY => 256,
            redirect_flags::RED_EOF => 128,
            redirect_flags::RED_USED => 64,
            redirect_flags::RED_FLUSH => 32,
            redirect_flags::RED_APPEND => 16,
            redirect_flags::RED_WRITE => 8,
            redirect_flags::RED_READ => 4,
            redirect_flags::RED_PIPE => 2,
            redirect_flags::RED_FILE => 1,
            redirect_flags::RED_NONE => 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_item {
    pub freep: *mut block_item,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block_header {
    pub freep: *mut block_item,
    pub size: size_t,
    pub name: *const libc::c_char,
    pub highwater: libc::c_long,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum block_id {
    BLOCK_NODE = 0,
    BLOCK_BUCKET,
    BLOCK_MAX,
impl block_id {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            block_id::BLOCK_NODE => 0,
            block_id::BLOCK_BUCKET => 1,
            block_id::BLOCK_MAX => 2,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum do_flag_values {
    DO_MPFR = 32768,
    DO_DEBUG = 16384,
    DO_PROFILE = 8192,
    DO_SANDBOX = 4096,
    DO_TIDY_MEM = 2048,
    DO_DUMP_VARS = 1024,
    DO_PRETTY_PRINT = 512,
    DO_INTERVALS = 256,
    DO_NON_DEC_DATA = 128,
    DO_INTL = 64,
    DO_POSIX = 32,
    DO_TRADITIONAL = 16,
    DO_LINT_OLD = 8,
    DO_LINT_ALL = 4,
    DO_LINT_EXTENSIONS = 2,
    DO_LINT_INVALID = 1,
    DO_FLAG_NONE = 0,
impl do_flag_values {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            do_flag_values::DO_MPFR => 32768,
            do_flag_values::DO_DEBUG => 16384,
            do_flag_values::DO_PROFILE => 8192,
            do_flag_values::DO_SANDBOX => 4096,
            do_flag_values::DO_TIDY_MEM => 2048,
            do_flag_values::DO_DUMP_VARS => 1024,
            do_flag_values::DO_PRETTY_PRINT => 512,
            do_flag_values::DO_INTERVALS => 256,
            do_flag_values::DO_NON_DEC_DATA => 128,
            do_flag_values::DO_INTL => 64,
            do_flag_values::DO_POSIX => 32,
            do_flag_values::DO_TRADITIONAL => 16,
            do_flag_values::DO_LINT_OLD => 8,
            do_flag_values::DO_LINT_ALL => 4,
            do_flag_values::DO_LINT_EXTENSIONS => 2,
            do_flag_values::DO_LINT_INVALID => 1,
            do_flag_values::DO_FLAG_NONE => 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sort_context_t {
    SORTED_IN = 1,
    ASORT,
    ASORTI,
impl sort_context_t {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            sort_context_t::SORTED_IN => 1,
            sort_context_t::ASORT => 2,
            sort_context_t::ASORTI => 3,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub strings: *mut *mut libc::c_char,
    pub i: size_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct version_info {
    pub version: *const libc::c_char,
    pub next: *mut version_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext_exit_handler {
    pub next: *mut ext_exit_handler,
    pub funcp: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub arg0: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as libc::c_int as libc::c_long
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2056 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2088 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2092 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint
        & (NUMCUR as libc::c_int | USER_INPUT as libc::c_int) as libc::c_uint
        == USER_INPUT as libc::c_int as libc::c_uint
    {
        return force_number(n);
    }
    if (*n).flags as libc::c_uint & INTIND as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const libc::c_char,
    mut fmtidx: libc::c_int,
) -> *mut NODE {
    if (*s).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint {
        (*s).type_0 = Node_val;
        (*s)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*s).flags as libc::c_uint & !(NUMBER as libc::c_int) as libc::c_uint);
        return s;
    }
    if (*s).flags as libc::c_uint & STRCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
        && ((*s).sub.val.idx == -(1 as libc::c_int) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.expect("non-null function pointer")(fmtstr, fmtidx, s);
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as libc::c_uint & NUMCUR as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut where_0: *const libc::c_char,
    mut var: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as libc::c_int as libc::c_ulong {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2070 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8
                as *const libc::c_char,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as libc::c_int as size_t, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"./awk.h\0" as *const u8 as *const libc::c_char,
            2074 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            where_0,
            var,
            count as libc::c_long,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as libc::c_uint & MALLOC as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint
    {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn assoc_set(
    mut array: *mut NODE,
    mut sub: *mut NODE,
    mut value: *mut NODE,
) {
    let mut lhs: *mut *mut NODE = ((*(*array).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(array, sub);
    unref(*lhs);
    *lhs = value;
    if ((*(*array).sub.nodep.l.lp).store).is_some() {
        (Some(((*(*array).sub.nodep.l.lp).store).expect("non-null function pointer")))
            .expect("non-null function pointer")(array, sub);
    }
    unref(sub);
}
unsafe extern "C" fn api_get_argument(
    mut id: awk_ext_id_t,
    mut count: size_t,
    mut wanted: awk_valtype_t,
    mut result: *mut awk_value_t,
) -> awk_bool_t {
    let mut current_block: u64;
    let mut arg: *mut NODE = 0 as *mut NODE;
    if result.is_null() {
        return awk_false;
    }
    memset(
        result as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<awk_value_t>() as libc::c_ulong,
    );
    (*result).val_type = AWK_UNDEFINED;
    arg = get_argument(count as libc::c_int);
    if arg.is_null() {
        return awk_false;
    }
    if (*arg).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
        || (*arg).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        if wanted as libc::c_uint == AWK_UNDEFINED as libc::c_int as libc::c_uint {
            return awk_true
        } else if wanted as libc::c_uint == AWK_ARRAY as libc::c_int as libc::c_uint {
            current_block = 555776418994083294;
        } else {
            current_block = 12590570269189879637;
        }
    } else if (*arg).type_0 as libc::c_uint
        == Node_var_array as libc::c_int as libc::c_uint
        || (*arg).type_0 as libc::c_uint == Node_array_ref as libc::c_int as libc::c_uint
    {
        if wanted as libc::c_uint != AWK_ARRAY as libc::c_int as libc::c_uint
            && wanted as libc::c_uint != AWK_UNDEFINED as libc::c_int as libc::c_uint
        {
            return awk_false;
        }
        current_block = 555776418994083294;
    } else {
        current_block = 12590570269189879637;
    }
    match current_block {
        12590570269189879637 => {
            arg = get_actual_argument(arg, count as libc::c_int, 0 as libc::c_int != 0);
            if arg.is_null() {
                return awk_false;
            }
            return node_to_awk_value(arg, result, wanted);
        }
        _ => {
            arg = get_actual_argument(arg, count as libc::c_int, 1 as libc::c_int != 0);
            if arg.is_null() {
                return awk_false;
            }
            return node_to_awk_value(arg, result, wanted);
        }
    };
}
unsafe extern "C" fn api_set_argument(
    mut id: awk_ext_id_t,
    mut count: size_t,
    mut new_array: awk_array_t,
) -> awk_bool_t {
    let mut arg: *mut NODE = 0 as *mut NODE;
    let mut array: *mut NODE = new_array as *mut NODE;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    arg = get_argument(count as libc::c_int);
    if arg.is_null()
        || (*arg).type_0 as libc::c_uint != Node_var_new as libc::c_int as libc::c_uint
            && (*arg).type_0 as libc::c_uint
                != Node_elem_new as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    arg = get_actual_argument(arg, count as libc::c_int, 1 as libc::c_int != 0);
    if arg.is_null() {
        return awk_false;
    }
    (*array).sub.nodep.name = (*arg).sub.nodep.name;
    *arg = *array;
    let ref mut fresh0 = (*(array as *mut block_item)).freep;
    *fresh0 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize].freep = array as *mut block_item;
    return awk_true;
}
#[no_mangle]
pub unsafe extern "C" fn awk_value_to_node(mut retval: *const awk_value_t) -> *mut NODE {
    let mut ext_ret_val: *mut NODE = 0 as *mut NODE;
    let mut v: *mut NODE = 0 as *mut NODE;
    if retval.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"awk_value_to_node: received null retval\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    match (*retval).val_type as libc::c_uint {
        5 => {
            ext_ret_val = (*retval).u.a as *mut NODE;
        }
        0 => {
            ext_ret_val = dupnode(Nnull_string);
        }
        8 => {
            ext_ret_val = make_bool_node(
                (*retval).u.b as libc::c_uint != awk_false as libc::c_int as libc::c_uint,
            );
        }
        1 => {
            match (*retval).u.n.type_0 as libc::c_uint {
                0 => {
                    ext_ret_val = make_number
                        .expect("non-null function pointer")((*retval).u.n.d);
                }
                1 => {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                        183 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"awk_value_to_node: MPFR not supported\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                2 => {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                        195 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"awk_value_to_node: MPFR not supported\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                _ => {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                        199 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"awk_value_to_node: invalid number type `%d'\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*retval).u.n.type_0 as libc::c_uint,
                    );
                }
            }
        }
        2 => {
            ext_ret_val = make_str_node(
                (*retval).u.s.str_0,
                (*retval).u.s.len,
                2 as libc::c_int,
            );
        }
        4 => {
            ext_ret_val = make_str_node(
                (*retval).u.s.str_0,
                (*retval).u.s.len,
                2 as libc::c_int,
            );
            (*ext_ret_val)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                flagvals,
            >(
                (*ext_ret_val).flags as libc::c_uint
                    | USER_INPUT as libc::c_int as libc::c_uint,
            );
        }
        3 => {
            ext_ret_val = make_typed_regex((*retval).u.s.str_0, (*retval).u.s.len);
        }
        6 => {
            v = (*retval).u.scl as *mut NODE;
            if (*v).type_0 as libc::c_uint != Node_var as libc::c_int as libc::c_uint {
                ext_ret_val = 0 as *mut NODE;
            } else {
                ext_ret_val = dupnode((*v).sub.nodep.l.lptr);
            }
        }
        7 => {
            ext_ret_val = dupnode((*retval).u.vc as *mut NODE);
        }
        _ => {
            ext_ret_val = 0 as *mut NODE;
        }
    }
    return ext_ret_val;
}
unsafe extern "C" fn api_fatal(
    mut id: awk_ext_id_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        1 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"fatal: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        format,
        args_0.as_va_list(),
    );
}
unsafe extern "C" fn api_nonfatal(
    mut id: awk_ext_id_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"error: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        format,
        args_0.as_va_list(),
    );
}
unsafe extern "C" fn api_warning(
    mut id: awk_ext_id_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    err(
        0 as libc::c_int != 0,
        dcgettext(
            0 as *const libc::c_char,
            b"warning: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        format,
        args_0.as_va_list(),
    );
}
unsafe extern "C" fn api_lintwarn(
    mut id: awk_ext_id_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    if lintfunc == Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ())
    {
        err(
            1 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"fatal: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            format,
            args_0.as_va_list(),
        );
    } else {
        err(
            0 as libc::c_int != 0,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            format,
            args_0.as_va_list(),
        );
    };
}
unsafe extern "C" fn api_register_input_parser(
    mut id: awk_ext_id_t,
    mut input_parser: *mut awk_input_parser_t,
) {
    if input_parser.is_null() {
        return;
    }
    register_input_parser(input_parser);
}
unsafe extern "C" fn api_register_output_wrapper(
    mut id: awk_ext_id_t,
    mut output_wrapper: *mut awk_output_wrapper_t,
) {
    if output_wrapper.is_null() {
        return;
    }
    register_output_wrapper(output_wrapper);
}
unsafe extern "C" fn api_register_two_way_processor(
    mut id: awk_ext_id_t,
    mut two_way_processor: *mut awk_two_way_processor_t,
) {
    if two_way_processor.is_null() {
        return;
    }
    register_two_way_processor(two_way_processor);
}
unsafe extern "C" fn api_update_ERRNO_int(
    mut id: awk_ext_id_t,
    mut errno_val: libc::c_int,
) {
    update_ERRNO_int(errno_val);
}
unsafe extern "C" fn api_update_ERRNO_string(
    mut id: awk_ext_id_t,
    mut string: *const libc::c_char,
) {
    if string.is_null() {
        return;
    }
    update_ERRNO_string(string);
}
unsafe extern "C" fn api_unset_ERRNO(mut id: awk_ext_id_t) {
    unset_ERRNO();
}
unsafe extern "C" fn api_add_ext_func(
    mut id: awk_ext_id_t,
    mut name_space: *const libc::c_char,
    mut func: *mut awk_ext_func_t,
) -> awk_bool_t {
    if func.is_null() {
        return awk_false;
    }
    if name_space.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            386 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"add_ext_func: received NULL name_space parameter\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return make_builtin(name_space, func);
}
static mut list_head: *mut ext_exit_handler = 0 as *const ext_exit_handler
    as *mut ext_exit_handler;
#[no_mangle]
pub unsafe extern "C" fn run_ext_exit_handlers(mut exitval: libc::c_int) {
    let mut p: *mut ext_exit_handler = 0 as *mut ext_exit_handler;
    let mut next: *mut ext_exit_handler = 0 as *mut ext_exit_handler;
    p = list_head;
    while !p.is_null() {
        next = (*p).next;
        ((*p).funcp).expect("non-null function pointer")((*p).arg0, exitval);
        pma_free(p as *mut libc::c_void);
        p = next;
    }
    list_head = 0 as *mut ext_exit_handler;
}
unsafe extern "C" fn api_awk_atexit(
    mut id: awk_ext_id_t,
    mut funcp: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    mut arg0: *mut libc::c_void,
) {
    let mut p: *mut ext_exit_handler = 0 as *mut ext_exit_handler;
    if funcp.is_none() {
        return;
    }
    p = emalloc_real(
        ::core::mem::size_of::<ext_exit_handler>() as libc::c_ulong,
        b"api_awk_atexit\0" as *const u8 as *const libc::c_char,
        b"p\0" as *const u8 as *const libc::c_char,
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        434 as libc::c_int,
    ) as *mut ext_exit_handler;
    (*p).funcp = funcp;
    (*p).arg0 = arg0;
    (*p).next = list_head;
    list_head = p;
}
static mut scopy: C2RustUnnamed_12 = C2RustUnnamed_12 {
    strings: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    i: 0,
    size: 0,
};
#[no_mangle]
pub unsafe extern "C" fn free_api_string_copies() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < scopy.i {
        pma_free(*(scopy.strings).offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    scopy.i = 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn assign_string(
    mut node: *mut NODE,
    mut val: *mut awk_value_t,
    mut val_type: awk_valtype_t,
) {
    (*val).val_type = val_type;
    if *((*node).sub.val.sp).offset((*node).sub.val.slen as isize) as libc::c_int
        != '\0' as i32
    {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        if scopy.i == scopy.size {
            if scopy.size == 0 as libc::c_int as libc::c_ulong {
                scopy.size = 8 as libc::c_int as size_t;
            } else {
                scopy
                    .size = (scopy.size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            }
            scopy
                .strings = erealloc_real(
                scopy.strings as *mut libc::c_void,
                (scopy.size)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
                b"assign_string\0" as *const u8 as *const libc::c_char,
                b"scopy.strings\0" as *const u8 as *const libc::c_char,
                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                482 as libc::c_int,
            ) as *mut *mut libc::c_char;
        }
        s = emalloc_real(
            ((*node).sub.val.slen).wrapping_add(1 as libc::c_int as libc::c_ulong),
            b"assign_string\0" as *const u8 as *const libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char,
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int,
        ) as *mut libc::c_char;
        memcpy(
            s as *mut libc::c_void,
            (*node).sub.val.sp as *const libc::c_void,
            (*node).sub.val.slen,
        );
        *s.offset((*node).sub.val.slen as isize) = '\0' as i32 as libc::c_char;
        let fresh1 = scopy.i;
        scopy.i = (scopy.i).wrapping_add(1);
        let ref mut fresh2 = *(scopy.strings).offset(fresh1 as isize);
        *fresh2 = s;
        (*val).u.s.str_0 = *fresh2;
    } else {
        (*val).u.s.str_0 = (*node).sub.val.sp;
    }
    (*val).u.s.len = (*node).sub.val.slen;
}
#[inline]
unsafe extern "C" fn assign_number(mut node: *mut NODE, mut val: *mut awk_value_t) {
    (*val).val_type = AWK_NUMBER;
    (*val).u.n.d = (*node).sub.val.fltnum;
    (*val).u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
    (*val).u.n.ptr = 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn assign_regex(mut node: *mut NODE, mut val: *mut awk_value_t) {
    (*val).u.s.str_0 = (*node).sub.val.sp;
    (*val).u.s.len = (*node).sub.val.slen;
    (*val).val_type = AWK_REGEX;
}
#[inline]
unsafe extern "C" fn assign_bool(mut node: *mut NODE, mut val: *mut awk_value_t) {
    (*val).val_type = AWK_BOOL;
    (*val)
        .u
        .b = (if (*node).sub.val.fltnum as libc::c_long
        != 0 as libc::c_int as libc::c_long
    {
        awk_true as libc::c_int
    } else {
        awk_false as libc::c_int
    }) as awk_bool_t;
}
unsafe extern "C" fn node_to_awk_value(
    mut node: *mut NODE,
    mut val: *mut awk_value_t,
    mut wanted: awk_valtype_t,
) -> awk_bool_t {
    let mut ret: awk_bool_t = awk_false;
    if node.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"node_to_awk_value: received null node\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if val.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            565 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"node_to_awk_value: received null val\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    let mut current_block_78: u64;
    match (*node).type_0 as libc::c_uint {
        6 | 7 => {
            (*val).val_type = AWK_UNDEFINED;
            if wanted as libc::c_uint == AWK_UNDEFINED as libc::c_int as libc::c_uint {
                ret = awk_true;
            }
            current_block_78 = 11071260907632769126;
        }
        4 => {
            if wanted as libc::c_uint == AWK_SCALAR as libc::c_int as libc::c_uint {
                (*val).val_type = AWK_SCALAR;
                (*val).u.scl = node as *mut libc::c_void;
                ret = awk_true;
                current_block_78 = 11071260907632769126;
            } else {
                node = (*node).sub.nodep.l.lptr;
                current_block_78 = 8233643070730889256;
            }
        }
        1 => {
            current_block_78 = 8233643070730889256;
        }
        5 => {
            (*val).val_type = AWK_ARRAY;
            if wanted as libc::c_uint == AWK_ARRAY as libc::c_int as libc::c_uint
                || wanted as libc::c_uint == AWK_UNDEFINED as libc::c_int as libc::c_uint
            {
                (*val).u.a = node as awk_array_t;
                ret = awk_true;
            } else {
                ret = awk_false;
            }
            current_block_78 = 11071260907632769126;
        }
        _ => {
            (*val).val_type = AWK_UNDEFINED;
            ret = awk_false;
            current_block_78 = 11071260907632769126;
        }
    }
    match current_block_78 {
        8233643070730889256 => {
            match wanted as libc::c_uint {
                8 => {
                    if (*node).flags as libc::c_uint
                        & BOOLVAL as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        assign_bool(node, val);
                        ret = awk_true;
                    } else {
                        ret = awk_false;
                    }
                }
                1 => {
                    if (*node).flags as libc::c_uint
                        & REGEX as libc::c_int as libc::c_uint
                        != 0 as libc::c_int as libc::c_uint
                    {
                        (*val).val_type = AWK_REGEX;
                    } else {
                        force_number(node);
                        assign_number(node, val);
                        ret = awk_true;
                    }
                }
                4 => {
                    let mut current_block_31: u64;
                    match (*fixtype(node)).flags as libc::c_uint
                        & (STRING as libc::c_int | NUMBER as libc::c_int
                            | USER_INPUT as libc::c_int | REGEX as libc::c_int
                            | BOOLVAL as libc::c_int) as libc::c_uint
                    {
                        80 => {
                            (*val).val_type = AWK_BOOL;
                            current_block_31 = 2873832966593178012;
                        }
                        2 => {
                            (*val).val_type = AWK_STRING;
                            current_block_31 = 2873832966593178012;
                        }
                        16 => {
                            force_string_fmt(node, CONVFMT, CONVFMTidx);
                            current_block_31 = 9804766041727059788;
                        }
                        48 => {
                            current_block_31 = 9804766041727059788;
                        }
                        524288 => {
                            (*val).val_type = AWK_REGEX;
                            current_block_31 = 2873832966593178012;
                        }
                        18 => {
                            if node == Nnull_string {
                                (*val).val_type = AWK_UNDEFINED;
                                current_block_31 = 2873832966593178012;
                            } else {
                                current_block_31 = 12173566750026564479;
                            }
                        }
                        _ => {
                            current_block_31 = 12173566750026564479;
                        }
                    }
                    match current_block_31 {
                        12173566750026564479 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                                633 as libc::c_int,
                            );
                            (Some(
                                (Some(
                                    r_warning
                                        as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                ))
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"node_to_awk_value detected invalid flags combination `%s'; please file a bug report\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                flags2str((*node).flags as libc::c_int),
                            );
                            (*val).val_type = AWK_UNDEFINED;
                        }
                        9804766041727059788 => {
                            assign_string(node, val, AWK_STRNUM);
                            ret = awk_true;
                        }
                        _ => {}
                    }
                }
                2 => {
                    force_string_fmt(node, CONVFMT, CONVFMTidx);
                    assign_string(node, val, AWK_STRING);
                    ret = awk_true;
                }
                3 => {
                    let mut current_block_44: u64;
                    match (*fixtype(node)).flags as libc::c_uint
                        & (STRING as libc::c_int | NUMBER as libc::c_int
                            | USER_INPUT as libc::c_int | REGEX as libc::c_int
                            | BOOLVAL as libc::c_int) as libc::c_uint
                    {
                        2 => {
                            (*val).val_type = AWK_STRING;
                            current_block_44 = 11777552016271000781;
                        }
                        80 => {
                            (*val).val_type = AWK_BOOL;
                            current_block_44 = 11777552016271000781;
                        }
                        16 => {
                            (*val).val_type = AWK_NUMBER;
                            current_block_44 = 11777552016271000781;
                        }
                        48 => {
                            (*val).val_type = AWK_STRNUM;
                            current_block_44 = 11777552016271000781;
                        }
                        524288 => {
                            assign_regex(node, val);
                            ret = awk_true;
                            current_block_44 = 11777552016271000781;
                        }
                        18 => {
                            if node == Nnull_string {
                                (*val).val_type = AWK_UNDEFINED;
                                current_block_44 = 11777552016271000781;
                            } else {
                                current_block_44 = 7905020471275165600;
                            }
                        }
                        _ => {
                            current_block_44 = 7905020471275165600;
                        }
                    }
                    match current_block_44 {
                        7905020471275165600 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                                670 as libc::c_int,
                            );
                            (Some(
                                (Some(
                                    r_warning
                                        as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                ))
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"node_to_awk_value detected invalid flags combination `%s'; please file a bug report\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                flags2str((*node).flags as libc::c_int),
                            );
                            (*val).val_type = AWK_UNDEFINED;
                        }
                        _ => {}
                    }
                }
                6 => {
                    let mut current_block_53: u64;
                    match (*fixtype(node)).flags as libc::c_uint
                        & (STRING as libc::c_int | NUMBER as libc::c_int
                            | USER_INPUT as libc::c_int | REGEX as libc::c_int
                            | BOOLVAL as libc::c_int) as libc::c_uint
                    {
                        80 => {
                            (*val).val_type = AWK_BOOL;
                            current_block_53 = 307447392441238883;
                        }
                        2 => {
                            (*val).val_type = AWK_STRING;
                            current_block_53 = 307447392441238883;
                        }
                        16 => {
                            (*val).val_type = AWK_NUMBER;
                            current_block_53 = 307447392441238883;
                        }
                        48 => {
                            (*val).val_type = AWK_STRNUM;
                            current_block_53 = 307447392441238883;
                        }
                        524288 => {
                            (*val).val_type = AWK_REGEX;
                            current_block_53 = 307447392441238883;
                        }
                        18 => {
                            if node == Nnull_string {
                                (*val).val_type = AWK_UNDEFINED;
                                current_block_53 = 307447392441238883;
                            } else {
                                current_block_53 = 16418587454608096141;
                            }
                        }
                        _ => {
                            current_block_53 = 16418587454608096141;
                        }
                    }
                    match current_block_53 {
                        16418587454608096141 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                                700 as libc::c_int,
                            );
                            (Some(
                                (Some(
                                    r_warning
                                        as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                ))
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"node_to_awk_value detected invalid flags combination `%s'; please file a bug report\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                flags2str((*node).flags as libc::c_int),
                            );
                            (*val).val_type = AWK_UNDEFINED;
                        }
                        _ => {}
                    }
                }
                0 => {
                    let mut current_block_68: u64;
                    match (*fixtype(node)).flags as libc::c_uint
                        & (STRING as libc::c_int | NUMBER as libc::c_int
                            | USER_INPUT as libc::c_int | REGEX as libc::c_int
                            | BOOLVAL as libc::c_int) as libc::c_uint
                    {
                        80 => {
                            assign_bool(node, val);
                            ret = awk_true;
                            current_block_68 = 17167606947040001567;
                        }
                        2 => {
                            assign_string(node, val, AWK_STRING);
                            ret = awk_true;
                            current_block_68 = 17167606947040001567;
                        }
                        16 => {
                            assign_number(node, val);
                            ret = awk_true;
                            current_block_68 = 17167606947040001567;
                        }
                        48 => {
                            assign_string(node, val, AWK_STRNUM);
                            ret = awk_true;
                            current_block_68 = 17167606947040001567;
                        }
                        524288 => {
                            assign_regex(node, val);
                            ret = awk_true;
                            current_block_68 = 17167606947040001567;
                        }
                        18 => {
                            if node == Nnull_string {
                                (*val).val_type = AWK_UNDEFINED;
                                ret = awk_true;
                                current_block_68 = 17167606947040001567;
                            } else {
                                current_block_68 = 16799793128756688569;
                            }
                        }
                        _ => {
                            current_block_68 = 16799793128756688569;
                        }
                    }
                    match current_block_68 {
                        16799793128756688569 => {
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                    libc::c_int,
                                ) -> ())(
                                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                                737 as libc::c_int,
                            );
                            (Some(
                                (Some(
                                    r_warning
                                        as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                                ))
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"node_to_awk_value detected invalid flags combination `%s'; please file a bug report\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                flags2str((*node).flags as libc::c_int),
                            );
                            (*val).val_type = AWK_UNDEFINED;
                        }
                        _ => {}
                    }
                }
                5 | 7 | _ => {}
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn api_sym_lookup(
    mut id: awk_ext_id_t,
    mut name_space: *const libc::c_char,
    mut name: *const libc::c_char,
    mut wanted: awk_valtype_t,
    mut result: *mut awk_value_t,
) -> awk_bool_t {
    let mut node: *mut NODE = 0 as *mut NODE;
    update_global_values();
    if name.is_null() || *name as libc::c_int == '\0' as i32 || result.is_null()
        || !is_valid_identifier(name) || name_space.is_null()
        || *name_space.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && !is_valid_identifier(name_space)
    {
        return awk_false;
    }
    node = ns_lookup(name_space, name, 0 as *mut *mut libc::c_char);
    if node.is_null() {
        return awk_false;
    }
    if is_off_limits_var(name) != 0 {
        (*node)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*node).flags as libc::c_uint | NO_EXT_SET as libc::c_int as libc::c_uint);
    }
    return node_to_awk_value(node, result, wanted);
}
unsafe extern "C" fn api_sym_lookup_scalar(
    mut id: awk_ext_id_t,
    mut cookie: awk_scalar_t,
    mut wanted: awk_valtype_t,
    mut result: *mut awk_value_t,
) -> awk_bool_t {
    let mut node: *mut NODE = cookie as *mut NODE;
    if node.is_null() || result.is_null()
        || (*node).type_0 as libc::c_uint != Node_var as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    update_global_values();
    return node_to_awk_value(node, result, wanted);
}
unsafe extern "C" fn api_sym_update(
    mut id: awk_ext_id_t,
    mut name_space: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *mut awk_value_t,
) -> awk_bool_t {
    let mut node: *mut NODE = 0 as *mut NODE;
    let mut array_node: *mut NODE = 0 as *mut NODE;
    if name.is_null() || *name as libc::c_int == '\0' as i32 || value.is_null()
        || !is_valid_identifier(name) || name_space.is_null()
        || *name_space.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && !is_valid_identifier(name_space)
    {
        return awk_false;
    }
    match (*value).val_type as libc::c_uint {
        1 | 4 | 2 | 3 | 0 | 5 | 6 | 7 => {}
        _ => return awk_false,
    }
    let mut full_name: *mut libc::c_char = 0 as *mut libc::c_char;
    node = ns_lookup(name_space, name, &mut full_name);
    if node.is_null() {
        if (*value).val_type as libc::c_uint == AWK_ARRAY as libc::c_int as libc::c_uint
        {
            array_node = awk_value_to_node(value);
            node = install_symbol(full_name, Node_var_array);
            (*array_node).sub.nodep.name = (*node).sub.nodep.name;
            *node = *array_node;
            let ref mut fresh3 = (*(array_node as *mut block_item)).freep;
            *fresh3 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = array_node as *mut block_item;
            (*value).u.a = node as awk_array_t;
        } else {
            node = install_symbol(full_name, Node_var);
            (*node).sub.nodep.l.lptr = awk_value_to_node(value);
        }
        return awk_true;
    }
    if (*node).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
        != 0 as libc::c_int as libc::c_uint || is_off_limits_var(full_name) != 0
    {
        (*node)
            .flags = ::core::mem::transmute::<
            libc::c_uint,
            flagvals,
        >((*node).flags as libc::c_uint | NO_EXT_SET as libc::c_int as libc::c_uint);
        pma_free(full_name as *mut libc::c_void);
        return awk_false;
    }
    pma_free(full_name as *mut libc::c_void);
    if (*value).val_type as libc::c_uint == AWK_ARRAY as libc::c_int as libc::c_uint {
        if (*node).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
        {
            array_node = awk_value_to_node(value);
            (*array_node).sub.nodep.name = (*node).sub.nodep.name;
            unref((*node).sub.nodep.l.lptr);
            *node = *array_node;
            let ref mut fresh4 = (*(array_node as *mut block_item)).freep;
            *fresh4 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
            nextfree[BLOCK_NODE as libc::c_int as usize]
                .freep = array_node as *mut block_item;
            (*value).u.a = node as awk_array_t;
            return awk_true;
        }
    } else if (*node).type_0 as libc::c_uint == Node_var as libc::c_int as libc::c_uint
        || (*node).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
        || (*node).type_0 as libc::c_uint == Node_elem_new as libc::c_int as libc::c_uint
    {
        unref((*node).sub.nodep.l.lptr);
        (*node).sub.nodep.l.lptr = awk_value_to_node(value);
        if ((*node).type_0 as libc::c_uint == Node_var_new as libc::c_int as libc::c_uint
            || (*node).type_0 as libc::c_uint
                == Node_elem_new as libc::c_int as libc::c_uint)
            && (*value).val_type as libc::c_uint
                != AWK_UNDEFINED as libc::c_int as libc::c_uint
        {
            (*node).type_0 = Node_var;
        }
        return awk_true;
    }
    return awk_false;
}
unsafe extern "C" fn api_sym_update_scalar(
    mut id: awk_ext_id_t,
    mut cookie: awk_scalar_t,
    mut value: *mut awk_value_t,
) -> awk_bool_t {
    let mut node: *mut NODE = cookie as *mut NODE;
    if value.is_null() || node.is_null()
        || (*node).type_0 as libc::c_uint != Node_var as libc::c_int as libc::c_uint
        || (*node).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    match (*value).val_type as libc::c_uint {
        1 => {
            if (*(*node).sub.nodep.l.lptr).valref == 1 as libc::c_int as libc::c_long
                && do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint == 0
            {
                let mut r: *mut NODE = (*node).sub.nodep.l.lptr;
                if (*r).flags as libc::c_uint
                    & (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                    == (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                {
                    pma_free((*r).sub.val.sp as *mut libc::c_void);
                }
                if (*r).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
                    != 0
                {
                    r_free_wstr(r);
                }
                (*r).sub.val.fltnum = (*value).u.n.d;
                (*r)
                    .flags = (MALLOC as libc::c_int | NUMBER as libc::c_int
                    | NUMCUR as libc::c_int) as flagvals;
                (*r).sub.val.sp = 0 as *mut libc::c_char;
                (*r).sub.val.slen = 0 as libc::c_int as size_t;
                return awk_true;
            }
        }
        2 | 4 => {
            if (*(*node).sub.nodep.l.lptr).valref == 1 as libc::c_int as libc::c_long {
                let mut r_0: *mut NODE = (*node).sub.nodep.l.lptr;
                if (*r_0).flags as libc::c_uint
                    & (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                    == (MALLOC as libc::c_int | STRCUR as libc::c_int) as libc::c_uint
                {
                    pma_free((*r_0).sub.val.sp as *mut libc::c_void);
                }
                mpfr_unset(r_0);
                if (*r_0).flags as libc::c_uint & WSTRCUR as libc::c_int as libc::c_uint
                    != 0
                {
                    r_free_wstr(r_0);
                }
                (*r_0).sub.val.fltnum = 0 as libc::c_int as libc::c_double;
                (*r_0)
                    .flags = (MALLOC as libc::c_int | STRING as libc::c_int
                    | STRCUR as libc::c_int) as flagvals;
                if (*value).val_type as libc::c_uint
                    == AWK_STRNUM as libc::c_int as libc::c_uint
                {
                    (*r_0)
                        .flags = ::core::mem::transmute::<
                        libc::c_uint,
                        flagvals,
                    >(
                        (*r_0).flags as libc::c_uint
                            | USER_INPUT as libc::c_int as libc::c_uint,
                    );
                }
                (*r_0).sub.val.idx = -(1 as libc::c_int);
                (*r_0).sub.val.sp = (*value).u.s.str_0;
                (*r_0).sub.val.slen = (*value).u.s.len;
                return awk_true;
            }
        }
        3 | 0 | 6 | 7 => {}
        _ => return awk_false,
    }
    unref((*node).sub.nodep.l.lptr);
    (*node).sub.nodep.l.lptr = awk_value_to_node(value);
    return awk_true;
}
#[inline]
unsafe extern "C" fn valid_subscript_type(mut valtype: awk_valtype_t) -> bool {
    match valtype as libc::c_uint {
        0 | 1 | 4 | 2 | 3 | 6 | 7 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn api_get_array_element(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    index: *const awk_value_t,
    mut wanted: awk_valtype_t,
    mut result: *mut awk_value_t,
) -> awk_bool_t {
    let mut array: *mut NODE = a_cookie as *mut NODE;
    let mut subscript: *mut NODE = 0 as *mut NODE;
    let mut aptr: *mut *mut NODE = 0 as *mut *mut NODE;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint || result.is_null()
        || index.is_null() || !valid_subscript_type((*index).val_type)
    {
        return awk_false;
    }
    subscript = awk_value_to_node(index);
    if (in_array(array, subscript)).is_null() {
        unref(subscript);
        return awk_false;
    }
    aptr = ((*(*array).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(array, subscript);
    if aptr.is_null() {
        unref(subscript);
        return awk_false;
    }
    unref(subscript);
    return node_to_awk_value(*aptr, result, wanted);
}
unsafe extern "C" fn api_set_array_element(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    index: *const awk_value_t,
    value: *const awk_value_t,
) -> awk_bool_t {
    let mut array: *mut NODE = a_cookie as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    let mut elem: *mut NODE = 0 as *mut NODE;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        || (*array).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint || index.is_null() || value.is_null()
        || !valid_subscript_type((*index).val_type)
    {
        return awk_false;
    }
    tmp = awk_value_to_node(index);
    elem = awk_value_to_node(value);
    if (*elem).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        (*elem).sub.nodep.x.extra = array;
        (*elem).sub.nodep.name = estrdup((*index).u.s.str_0, (*index).u.s.len);
    }
    assoc_set(array, tmp, elem);
    return awk_true;
}
unsafe extern "C" fn remove_element(mut array: *mut NODE, mut subscript: *mut NODE) {
    let mut val: *mut NODE = 0 as *mut NODE;
    if array.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            1129 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"remove_element: received null array\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if subscript.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            1132 as libc::c_int,
        );
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"remove_element: received null subscript\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    val = in_array(array, subscript);
    if val.is_null() {
        return;
    }
    if (*val).type_0 as libc::c_uint == Node_var_array as libc::c_int as libc::c_uint {
        ((*(*val).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(val, 0 as *mut exp_node);
        pma_free((*val).sub.nodep.name as *mut libc::c_void);
        let ref mut fresh5 = (*(val as *mut block_item)).freep;
        *fresh5 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
        nextfree[BLOCK_NODE as libc::c_int as usize].freep = val as *mut block_item;
    } else {
        unref(val);
    }
    ((*(*array).sub.nodep.l.lp).remove)
        .expect("non-null function pointer")(array, subscript);
}
unsafe extern "C" fn api_del_array_element(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    index: *const awk_value_t,
) -> awk_bool_t {
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    array = a_cookie as *mut NODE;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        || (*array).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint || index.is_null()
        || !valid_subscript_type((*index).val_type)
    {
        return awk_false;
    }
    sub = awk_value_to_node(index);
    remove_element(array, sub);
    unref(sub);
    return awk_true;
}
unsafe extern "C" fn api_get_element_count(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    mut count: *mut size_t,
) -> awk_bool_t {
    let mut node: *mut NODE = a_cookie as *mut NODE;
    if count.is_null() || node.is_null()
        || (*node).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    *count = (*node).sub.nodep.reflags as size_t;
    return awk_true;
}
unsafe extern "C" fn api_create_array(mut id: awk_ext_id_t) -> awk_array_t {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[BLOCK_NODE as libc::c_int as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[BLOCK_NODE as libc::c_int as usize]
            .freep = (*(n as *mut block_item)).freep;
    } else {
        n = more_blocks(BLOCK_NODE as libc::c_int) as *mut NODE;
    };
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<NODE>() as libc::c_ulong,
    );
    null_array(n);
    return n as awk_array_t;
}
unsafe extern "C" fn api_clear_array(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
) -> awk_bool_t {
    let mut node: *mut NODE = a_cookie as *mut NODE;
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        || (*node).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
    {
        return awk_false;
    }
    ((*(*node).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(node, 0 as *mut exp_node);
    return awk_true;
}
unsafe extern "C" fn api_destroy_array(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
) -> awk_bool_t {
    if api_clear_array(id, a_cookie) as u64 == 0 {
        return awk_false;
    }
    let ref mut fresh6 = (*(a_cookie as *mut NODE as *mut block_item)).freep;
    *fresh6 = nextfree[BLOCK_NODE as libc::c_int as usize].freep;
    nextfree[BLOCK_NODE as libc::c_int as usize]
        .freep = a_cookie as *mut NODE as *mut block_item;
    return awk_true;
}
unsafe extern "C" fn api_flatten_array_typed(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    mut data: *mut *mut awk_flat_array_t,
    mut index_type: awk_valtype_t,
    mut value_type: awk_valtype_t,
) -> awk_bool_t {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut array: *mut NODE = a_cookie as *mut NODE;
    let mut alloc_size: size_t = 0;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint
        || (*array).sub.nodep.reflags as libc::c_uint == 0 as libc::c_int as libc::c_uint
        || data.is_null()
    {
        return awk_false;
    }
    alloc_size = (::core::mem::size_of::<awk_flat_array_t>() as libc::c_ulong)
        .wrapping_add(
            (((*array).sub.nodep.reflags as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<awk_element_t>() as libc::c_ulong),
        );
    *data = ezalloc_real(
        alloc_size,
        b"api_flatten_array_typed\0" as *const u8 as *const libc::c_char,
        b"*data\0" as *const u8 as *const libc::c_char,
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        1258 as libc::c_int,
    ) as *mut awk_flat_array_t;
    list = assoc_list(array, b"@unsorted\0" as *const u8 as *const libc::c_char, ASORTI);
    (**data).opaque1 = array as *mut libc::c_void;
    (**data).opaque2 = list as *mut libc::c_void;
    (**data).count = (*array).sub.nodep.reflags as size_t;
    j = 0 as libc::c_int as size_t;
    i = j;
    while i
        < (2 as libc::c_int as libc::c_uint)
            .wrapping_mul((*array).sub.nodep.reflags as libc::c_uint) as libc::c_ulong
    {
        let mut index: *mut NODE = 0 as *mut NODE;
        let mut value: *mut NODE = 0 as *mut NODE;
        index = *list.offset(i as isize);
        value = *list.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        if node_to_awk_value(
            index,
            &mut (*((**data).elements).as_mut_ptr().offset(j as isize)).index,
            index_type,
        ) as u64 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                1275 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"api_flatten_array_typed: could not convert index %d to %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                i as libc::c_int,
                valtype2str(index_type),
            );
        }
        if node_to_awk_value(
            value,
            &mut (*((**data).elements).as_mut_ptr().offset(j as isize)).value,
            value_type,
        ) as u64 == 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    libc::c_int,
                ) -> ())(
                b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                1280 as libc::c_int,
            );
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const libc::c_char,
                    b"api_flatten_array_typed: could not convert value %d to %s\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                i as libc::c_int,
                valtype2str(value_type),
            );
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        j = j.wrapping_add(1);
        j;
    }
    return awk_true;
}
unsafe extern "C" fn api_release_flattened_array(
    mut id: awk_ext_id_t,
    mut a_cookie: awk_array_t,
    mut data: *mut awk_flat_array_t,
) -> awk_bool_t {
    let mut array: *mut NODE = a_cookie as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if array.is_null()
        || (*array).type_0 as libc::c_uint
            != Node_var_array as libc::c_int as libc::c_uint || data.is_null()
        || array != (*data).opaque1 as *mut NODE
        || (*data).count != (*array).sub.nodep.reflags as libc::c_ulong
        || ((*data).opaque2).is_null()
    {
        return awk_false;
    }
    list = (*data).opaque2 as *mut *mut NODE;
    j = 0 as libc::c_int as size_t;
    i = j;
    k = (2 as libc::c_int as libc::c_uint)
        .wrapping_mul((*array).sub.nodep.reflags as libc::c_uint) as size_t;
    while i < k {
        if (*((*data).elements).as_mut_ptr().offset(j as isize)).flags as libc::c_uint
            & AWK_ELEMENT_DELETE as libc::c_int as libc::c_uint
            != 0 as libc::c_int as libc::c_uint
            && (*array).flags as libc::c_uint & NO_EXT_SET as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
        {
            remove_element(array, *list.offset(i as isize));
        }
        unref(*list.offset(i as isize));
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        j = j.wrapping_add(1);
        j;
    }
    pma_free(list as *mut libc::c_void);
    pma_free(data as *mut libc::c_void);
    return awk_true;
}
unsafe extern "C" fn api_create_value(
    mut id: awk_ext_id_t,
    mut value: *mut awk_value_t,
    mut result: *mut awk_value_cookie_t,
) -> awk_bool_t {
    if value.is_null() || result.is_null() {
        return awk_false;
    }
    match (*value).val_type as libc::c_uint {
        1 | 4 | 2 | 3 => {}
        _ => return awk_false,
    }
    *result = awk_value_to_node(value) as awk_value_cookie_t;
    return (*result != 0 as *mut libc::c_void) as libc::c_int as awk_bool_t;
}
unsafe extern "C" fn api_release_value(
    mut id: awk_ext_id_t,
    mut value: awk_value_cookie_t,
) -> awk_bool_t {
    let mut val: *mut NODE = value as *mut NODE;
    if val.is_null() {
        return awk_false;
    }
    unref(val);
    return awk_true;
}
unsafe extern "C" fn api_get_mpfr(mut id: awk_ext_id_t) -> *mut libc::c_void {
    (set_loc
        as unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
        ) -> ())(
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        1376 as libc::c_int,
    );
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        dcgettext(
            0 as *const libc::c_char,
            b"api_get_mpfr: MPFR not supported\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn api_get_mpz(mut id: awk_ext_id_t) -> *mut libc::c_void {
    (set_loc
        as unsafe extern "C" fn(
            *const libc::c_char,
            libc::c_int,
        ) -> ())(
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        1393 as libc::c_int,
    );
    (Some(
        (Some(r_fatal as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
            .expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(
        dcgettext(
            0 as *const libc::c_char,
            b"api_get_mpfr: MPFR not supported\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn api_get_file(
    mut id: awk_ext_id_t,
    mut name: *const libc::c_char,
    mut namelen: size_t,
    mut filetype: *const libc::c_char,
    mut fd: libc::c_int,
    mut ibufp: *mut *const awk_input_buf_t,
    mut obufp: *mut *const awk_output_buf_t,
) -> awk_bool_t {
    let mut f: *const redirect = 0 as *const redirect;
    let mut flag: libc::c_int = 0;
    let mut redirtype: redirval = redirect_none;
    if name.is_null() || namelen == 0 as libc::c_int as libc::c_ulong {
        if curfile.is_null() {
            let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            let mut save_rule: libc::c_int = 0;
            let mut save_source: *mut libc::c_char = 0 as *mut libc::c_char;
            if nextfile(&mut curfile, 0 as libc::c_int != 0) <= 0 as libc::c_int {
                return awk_false;
            }
            pc = main_beginfile;
            save_rule = currule;
            save_source = source;
            loop {
                if pc.is_null() {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            libc::c_int,
                        ) -> ())(
                        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
                        1424 as libc::c_int,
                    );
                    (Some(
                        (Some(
                            r_fatal
                                as unsafe extern "C" fn(*const libc::c_char, ...) -> (),
                        ))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"cannot find end of BEGINFILE rule\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if (*pc).opcode as libc::c_uint
                    == Op_after_beginfile as libc::c_int as libc::c_uint
                {
                    break;
                }
                pc = (*pc).nexti;
            }
            (*pc).opcode = Op_stop;
            (Some(interpret.expect("non-null function pointer")))
                .expect("non-null function pointer")(main_beginfile);
            (*pc).opcode = Op_after_beginfile;
            after_beginfile(&mut curfile);
            currule = save_rule;
            source = save_source;
        }
        *ibufp = &mut (*curfile).public;
        *obufp = 0 as *const awk_output_buf_t;
        return awk_true;
    }
    redirtype = redirect_none;
    match *filetype.offset(0 as libc::c_int as isize) as libc::c_int {
        60 => {
            if *filetype.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                redirtype = redirect_input;
            }
        }
        62 => {
            match *filetype.offset(1 as libc::c_int as isize) as libc::c_int {
                0 => {
                    redirtype = redirect_output;
                }
                62 => {
                    if *filetype.offset(2 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                    {
                        redirtype = redirect_append;
                    }
                }
                _ => {}
            }
        }
        124 => {
            if *filetype.offset(2 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                match *filetype.offset(1 as libc::c_int as isize) as libc::c_int {
                    62 => {
                        redirtype = redirect_pipe;
                    }
                    60 => {
                        redirtype = redirect_pipein;
                    }
                    38 => {
                        redirtype = redirect_twoway;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    if redirtype as libc::c_uint == redirect_none as libc::c_int as libc::c_uint {
        (set_loc
            as unsafe extern "C" fn(
                *const libc::c_char,
                libc::c_int,
            ) -> ())(
            b"gawkapi.c\0" as *const u8 as *const libc::c_char,
            1478 as libc::c_int,
        );
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const libc::c_char, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open unrecognized file type `%s' for `%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filetype,
            name,
        );
        return awk_false;
    }
    f = redirect_string(
        name,
        namelen,
        0 as libc::c_int != 0,
        redirtype as libc::c_int,
        &mut flag,
        fd,
        0 as libc::c_int != 0,
    );
    if f.is_null() {
        return awk_false;
    }
    *ibufp = if !((*f).iop).is_null() {
        &mut (*(*f).iop).public
    } else {
        0 as *mut awk_input_buf_t
    };
    *obufp = if !((*f).output.fp).is_null() {
        &(*f).output
    } else {
        0 as *const awk_output_buf_t
    };
    return awk_true;
}
static mut vi_head: *mut version_info = 0 as *const version_info as *mut version_info;
unsafe extern "C" fn api_register_ext_version(
    mut id: awk_ext_id_t,
    mut version: *const libc::c_char,
) {
    let mut info: *mut version_info = 0 as *mut version_info;
    if version.is_null() {
        return;
    }
    info = emalloc_real(
        ::core::mem::size_of::<version_info>() as libc::c_ulong,
        b"register_ext_version\0" as *const u8 as *const libc::c_char,
        b"info\0" as *const u8 as *const libc::c_char,
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        1514 as libc::c_int,
    ) as *mut version_info;
    (*info).version = version;
    (*info).next = vi_head;
    vi_head = info;
}
#[no_mangle]
pub static mut api_impl: gawk_api_t = unsafe {
    {
        let mut init = gawk_api {
            major_version: GAWK_API_MAJOR_VERSION as libc::c_int,
            minor_version: GAWK_API_MINOR_VERSION as libc::c_int,
            gmp_major_version: 0 as libc::c_int,
            gmp_minor_version: 0 as libc::c_int,
            mpfr_major_version: 0 as libc::c_int,
            mpfr_minor_version: 0 as libc::c_int,
            do_flags: [0 as libc::c_int, 0, 0, 0, 0, 0],
            api_add_ext_func: Some(
                api_add_ext_func
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *const libc::c_char,
                        *mut awk_ext_func_t,
                    ) -> awk_bool_t,
            ),
            api_register_input_parser: Some(
                api_register_input_parser
                    as unsafe extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t) -> (),
            ),
            api_register_output_wrapper: Some(
                api_register_output_wrapper
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *mut awk_output_wrapper_t,
                    ) -> (),
            ),
            api_register_two_way_processor: Some(
                api_register_two_way_processor
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *mut awk_two_way_processor_t,
                    ) -> (),
            ),
            api_awk_atexit: Some(
                api_awk_atexit
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        Option::<
                            unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
            ),
            api_register_ext_version: Some(
                api_register_ext_version
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
            ),
            api_fatal: Some(
                api_fatal
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
            ),
            api_warning: Some(
                api_warning
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
            ),
            api_lintwarn: Some(
                api_lintwarn
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
            ),
            api_nonfatal: Some(
                api_nonfatal
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char, ...) -> (),
            ),
            api_update_ERRNO_int: Some(
                api_update_ERRNO_int
                    as unsafe extern "C" fn(awk_ext_id_t, libc::c_int) -> (),
            ),
            api_update_ERRNO_string: Some(
                api_update_ERRNO_string
                    as unsafe extern "C" fn(awk_ext_id_t, *const libc::c_char) -> (),
            ),
            api_unset_ERRNO: Some(
                api_unset_ERRNO as unsafe extern "C" fn(awk_ext_id_t) -> (),
            ),
            api_get_argument: Some(
                api_get_argument
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        size_t,
                        awk_valtype_t,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_set_argument: Some(
                api_set_argument
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        size_t,
                        awk_array_t,
                    ) -> awk_bool_t,
            ),
            api_sym_lookup: Some(
                api_sym_lookup
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *const libc::c_char,
                        *const libc::c_char,
                        awk_valtype_t,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_sym_update: Some(
                api_sym_update
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *const libc::c_char,
                        *const libc::c_char,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_sym_lookup_scalar: Some(
                api_sym_lookup_scalar
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_scalar_t,
                        awk_valtype_t,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_sym_update_scalar: Some(
                api_sym_update_scalar
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_scalar_t,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_create_value: Some(
                api_create_value
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *mut awk_value_t,
                        *mut awk_value_cookie_t,
                    ) -> awk_bool_t,
            ),
            api_release_value: Some(
                api_release_value
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_value_cookie_t,
                    ) -> awk_bool_t,
            ),
            api_get_element_count: Some(
                api_get_element_count
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *mut size_t,
                    ) -> awk_bool_t,
            ),
            api_get_array_element: Some(
                api_get_array_element
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *const awk_value_t,
                        awk_valtype_t,
                        *mut awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_set_array_element: Some(
                api_set_array_element
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *const awk_value_t,
                        *const awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_del_array_element: Some(
                api_del_array_element
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *const awk_value_t,
                    ) -> awk_bool_t,
            ),
            api_create_array: Some(
                api_create_array as unsafe extern "C" fn(awk_ext_id_t) -> awk_array_t,
            ),
            api_clear_array: Some(
                api_clear_array
                    as unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
            ),
            api_flatten_array_typed: Some(
                api_flatten_array_typed
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *mut *mut awk_flat_array_t,
                        awk_valtype_t,
                        awk_valtype_t,
                    ) -> awk_bool_t,
            ),
            api_release_flattened_array: Some(
                api_release_flattened_array
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        awk_array_t,
                        *mut awk_flat_array_t,
                    ) -> awk_bool_t,
            ),
            api_malloc: Some(
                pma_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
            ),
            api_calloc: Some(
                pma_calloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
            ),
            api_realloc: Some(
                pma_realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
            api_free: Some(pma_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            api_get_mpfr: Some(
                api_get_mpfr as unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void,
            ),
            api_get_mpz: Some(
                api_get_mpz as unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void,
            ),
            api_get_file: Some(
                api_get_file
                    as unsafe extern "C" fn(
                        awk_ext_id_t,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        libc::c_int,
                        *mut *const awk_input_buf_t,
                        *mut *const awk_output_buf_t,
                    ) -> awk_bool_t,
            ),
            api_destroy_array: Some(
                api_destroy_array
                    as unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
            ),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn init_ext_api() {
    api_impl
        .do_flags[0 as libc::c_int
        as usize] = if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    api_impl
        .do_flags[1 as libc::c_int
        as usize] = if do_flags as libc::c_uint
        & DO_TRADITIONAL as libc::c_int as libc::c_uint != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    api_impl
        .do_flags[2 as libc::c_int
        as usize] = if do_flags as libc::c_uint
        & DO_PROFILE as libc::c_int as libc::c_uint != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    api_impl
        .do_flags[3 as libc::c_int
        as usize] = if do_flags as libc::c_uint
        & DO_SANDBOX as libc::c_int as libc::c_uint != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    api_impl
        .do_flags[4 as libc::c_int
        as usize] = if do_flags as libc::c_uint & DO_DEBUG as libc::c_int as libc::c_uint
        != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    api_impl
        .do_flags[5 as libc::c_int
        as usize] = if do_flags as libc::c_uint & DO_MPFR as libc::c_int as libc::c_uint
        != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn update_ext_api() {
    api_impl
        .do_flags[0 as libc::c_int
        as usize] = if do_flags as libc::c_uint
        & (DO_LINT_INVALID as libc::c_int | DO_LINT_ALL as libc::c_int) as libc::c_uint
        != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_ext_versions() {
    let mut p: *mut version_info = 0 as *mut version_info;
    p = vi_head;
    while !p.is_null() {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*p).version);
        p = (*p).next;
    }
}
unsafe extern "C" fn valtype2str(mut type_0: awk_valtype_t) -> *const libc::c_char {
    static mut buf: [libc::c_char; 100] = [0; 100];
    static mut values: [*const libc::c_char; 8] = [
        b"AWK_UNDEFINED\0" as *const u8 as *const libc::c_char,
        b"AWK_NUMBER\0" as *const u8 as *const libc::c_char,
        b"AWK_STRING\0" as *const u8 as *const libc::c_char,
        b"AWK_REGEX\0" as *const u8 as *const libc::c_char,
        b"AWK_STRNUM\0" as *const u8 as *const libc::c_char,
        b"AWK_ARRAY\0" as *const u8 as *const libc::c_char,
        b"AWK_SCALAR\0" as *const u8 as *const libc::c_char,
        b"AWK_VALUE_COOKIE\0" as *const u8 as *const libc::c_char,
    ];
    if AWK_UNDEFINED as libc::c_int as libc::c_uint <= type_0 as libc::c_uint
        && type_0 as libc::c_uint <= AWK_VALUE_COOKIE as libc::c_int as libc::c_uint
    {
        return values[type_0 as libc::c_int as usize];
    }
    sprintf(
        buf.as_mut_ptr(),
        b"unknown type! (%d)\0" as *const u8 as *const libc::c_char,
        type_0 as libc::c_int,
    );
    return buf.as_mut_ptr();
}
unsafe extern "C" fn ns_lookup(
    mut name_space: *const libc::c_char,
    mut name: *const libc::c_char,
    mut fullname: *mut *mut libc::c_char,
) -> *mut NODE {
    if *name_space.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || strcmp(name_space, awk_namespace.as_ptr()) == 0 as libc::c_int
    {
        if !fullname.is_null() {
            *fullname = estrdup(name, strlen(name));
        }
        return lookup(name);
    }
    let mut len: size_t = (strlen(name_space))
        .wrapping_add(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = emalloc_real(
        len,
        b"ns_lookup\0" as *const u8 as *const libc::c_char,
        b"buf\0" as *const u8 as *const libc::c_char,
        b"gawkapi.c\0" as *const u8 as *const libc::c_char,
        1673 as libc::c_int,
    ) as *mut libc::c_char;
    sprintf(buf, b"%s::%s\0" as *const u8 as *const libc::c_char, name_space, name);
    let mut f: *mut NODE = lookup(buf);
    if !fullname.is_null() {
        *fullname = buf;
    } else {
        pma_free(buf as *mut libc::c_void);
    }
    return f;
}
