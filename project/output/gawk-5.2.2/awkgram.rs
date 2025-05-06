#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fileno(__stream: *mut FILE) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const i8,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn __mbrlen(__s: *const i8, __n: size_t, __ps: *mut mbstate_t) -> size_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut FS_node: *mut NODE;
    static mut FPAT_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static mut do_optimize: bool;
    static mut gawk_mb_cur_max: i32;
    fn r_unref(tmp: *mut NODE);
    fn do_asort(nargs: i32) -> *mut NODE;
    fn do_asorti(nargs: i32) -> *mut NODE;
    static mut hash: Option<
        unsafe extern "C" fn(*const i8, size_t, u64, *mut size_t) -> u64,
    >;
    fn install_symbol(name: *const i8, type_0: NODETYPE) -> *mut NODE;
    fn err(isfatal: bool, s: *const i8, emsg: *const i8, argp: ::core::ffi::VaList);
    fn msg(mesg: *const i8, _: ...);
    fn lookup(name: *const i8) -> *mut NODE;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn check_param_names() -> bool;
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn in_main_context() -> i32;
    fn bcfree(_: *mut INSTRUCTION);
    fn bcalloc(op: OPCODE, size: i32, srcline: i32) -> *mut INSTRUCTION;
    fn opcode2str(type_0: OPCODE) -> *const i8;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn is_all_upper(name: *const i8) -> bool;
    fn is_std_var(var: *const i8) -> i32;
    fn do_index(nargs: i32) -> *mut NODE;
    fn pp_string_fp(
        print_func: Func_print,
        fp_0: *mut FILE,
        str: *const i8,
        namelen: size_t,
        delim: i32,
        breaklines: bool,
    );
    fn r_warning(mesg: *const i8, _: ...);
    fn do_dcngettext(nargs: i32) -> *mut NODE;
    fn do_dcgettext(nargs: i32) -> *mut NODE;
    fn do_close(nargs: i32) -> *mut NODE;
    fn make_regexp(
        s: *const i8,
        len: size_t,
        ignorecase: bool,
        dfa: bool,
        canfatal: bool,
    ) -> *mut Regexp;
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn do_patsplit(nargs: i32) -> *mut NODE;
    fn do_split(nargs: i32) -> *mut NODE;
    fn do_match(nargs: i32) -> *mut NODE;
    fn do_typeof(nargs: i32) -> *mut NODE;
    fn do_isarray(nargs: i32) -> *mut NODE;
    fn do_length(nargs: i32) -> *mut NODE;
    fn do_xor(nargs: i32) -> *mut NODE;
    fn do_toupper(nargs: i32) -> *mut NODE;
    fn do_tolower(nargs: i32) -> *mut NODE;
    fn do_systime(nargs: i32) -> *mut NODE;
    fn do_system(nargs: i32) -> *mut NODE;
    fn do_substr(nargs: i32) -> *mut NODE;
    fn do_strtonum(nargs: i32) -> *mut NODE;
    fn do_strftime(nargs: i32) -> *mut NODE;
    fn do_srand(nargs: i32) -> *mut NODE;
    fn do_sqrt(nargs: i32) -> *mut NODE;
    fn do_sprintf(nargs: i32) -> *mut NODE;
    fn do_sin(nargs: i32) -> *mut NODE;
    fn do_rshift(nargs: i32) -> *mut NODE;
    fn do_rand(nargs: i32) -> *mut NODE;
    fn do_or(nargs: i32) -> *mut NODE;
    fn do_mktime(nargs: i32) -> *mut NODE;
    fn do_mkbool(nargs: i32) -> *mut NODE;
    fn do_lshift(nargs: i32) -> *mut NODE;
    fn do_log(nargs: i32) -> *mut NODE;
    fn do_int(nargs: i32) -> *mut NODE;
    fn do_fflush(nargs: i32) -> *mut NODE;
    fn do_exp(nargs: i32) -> *mut NODE;
    fn do_cos(nargs: i32) -> *mut NODE;
    fn do_compl(nargs: i32) -> *mut NODE;
    fn do_bindtextdomain(nargs: i32) -> *mut NODE;
    fn do_atan2(nargs: i32) -> *mut NODE;
    fn do_and(nargs: i32) -> *mut NODE;
    fn gawk_exit(status: i32);
    fn calc_exp(x1: libc::c_double, x2: libc::c_double) -> libc::c_double;
    static mut func_table: *mut NODE;
    static mut symbol_table: *mut NODE;
    fn make_typed_regex(re: *const i8, len: size_t) -> *mut NODE;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn install_params(func: *mut NODE);
    fn make_params(pnames: *mut *mut i8, pcount: i32) -> *mut NODE;
    fn load_ext(lib_name: *const i8);
    fn files_are_same(path: *mut i8, src: *mut SRCFILE) -> i32;
    fn find_source(
        src: *const i8,
        stb: *mut stat,
        errcode: *mut i32,
        is_extlib: i32,
    ) -> *mut i8;
    fn is_valid_identifier(name: *const i8) -> bool;
    fn remove_params(func: *mut NODE);
    fn set_current_namespace(new_namespace: *const i8);
    fn do_sub(nargs: i32, flags: u32) -> *mut NODE;
    fn flags2str(_: i32) -> *const i8;
    fn variable_list() -> *mut *mut NODE;
    fn print_vars(table: *mut *mut NODE, print_func: Func_print, fp_0: *mut FILE);
    fn function_list(sort: bool) -> *mut *mut NODE;
    fn foreach_func(
        table: *mut *mut NODE,
        _: Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
        _: *mut libc::c_void,
    ) -> i32;
    fn pp_func(pc: *mut INSTRUCTION, _: *mut libc::c_void) -> i32;
    fn get_numbase(str: *const i8, len: size_t, use_locale: bool) -> i32;
    fn nondec2awknum(str: *mut i8, len: size_t, endptr: *mut *mut i8) -> libc::c_double;
    fn srcopen(s: *mut SRCFILE) -> i32;
    fn optimal_bufsize(fd: i32, sbuf: *mut stat) -> size_t;
    fn error(mesg: *const i8, _: ...);
    static mut rule_list: *mut INSTRUCTION;
    static mut max_args: i32;
    static mut args_array: *mut *mut NODE;
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
pub type size_t = u64;
pub type wchar_t = i32;
pub type __int32_t = i32;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type va_list = __builtin_va_list;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
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
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::_ISalnum => 8,
            C2RustUnnamed_0::_ISpunct => 4,
            C2RustUnnamed_0::_IScntrl => 2,
            C2RustUnnamed_0::_ISblank => 1,
            C2RustUnnamed_0::_ISgraph => 32768,
            C2RustUnnamed_0::_ISprint => 16384,
            C2RustUnnamed_0::_ISspace => 8192,
            C2RustUnnamed_0::_ISxdigit => 4096,
            C2RustUnnamed_0::_ISdigit => 2048,
            C2RustUnnamed_0::_ISalpha => 1024,
            C2RustUnnamed_0::_ISlower => 512,
            C2RustUnnamed_0::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            8 => C2RustUnnamed_0::_ISalnum,
            4 => C2RustUnnamed_0::_ISpunct,
            2 => C2RustUnnamed_0::_IScntrl,
            1 => C2RustUnnamed_0::_ISblank,
            32768 => C2RustUnnamed_0::_ISgraph,
            16384 => C2RustUnnamed_0::_ISprint,
            8192 => C2RustUnnamed_0::_ISspace,
            4096 => C2RustUnnamed_0::_ISxdigit,
            2048 => C2RustUnnamed_0::_ISdigit,
            1024 => C2RustUnnamed_0::_ISalpha,
            512 => C2RustUnnamed_0::_ISlower,
            256 => C2RustUnnamed_0::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type mbstate_t = __mbstate_t;
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
pub type __re_size_t = u32;
pub type __re_long_size_t = u64;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
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
pub type regoff_t = i32;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum defrule {
    BEGIN = 1,
    Rule,
    END,
    BEGINFILE,
    ENDFILE,
    MAXRULE,
}
impl defrule {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            defrule::BEGIN => 1,
            defrule::Rule => 2,
            defrule::END => 3,
            defrule::BEGINFILE => 4,
            defrule::ENDFILE => 5,
            defrule::MAXRULE => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> defrule {
        match value {
            1 => defrule::BEGIN,
            2 => defrule::Rule,
            3 => defrule::END,
            4 => defrule::BEGINFILE,
            5 => defrule::ENDFILE,
            6 => defrule::MAXRULE,
            _ => panic!("Invalid value for defrule: {}", value),
        }
    }
}
impl AddAssign<u32> for defrule {
    fn add_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for defrule {
    fn sub_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for defrule {
    fn mul_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for defrule {
    fn div_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for defrule {
    fn rem_assign(&mut self, rhs: u32) {
        *self = defrule::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for defrule {
    type Output = defrule;
    fn add(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for defrule {
    type Output = defrule;
    fn sub(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for defrule {
    type Output = defrule;
    fn mul(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for defrule {
    type Output = defrule;
    fn div(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for defrule {
    type Output = defrule;
    fn rem(self, rhs: u32) -> defrule {
        defrule::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
}
impl nodevals {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> nodevals {
        match value {
            0 => nodevals::Node_illegal,
            1 => nodevals::Node_val,
            2 => nodevals::Node_regex,
            3 => nodevals::Node_dynregex,
            4 => nodevals::Node_var,
            5 => nodevals::Node_var_array,
            6 => nodevals::Node_var_new,
            7 => nodevals::Node_elem_new,
            8 => nodevals::Node_param_list,
            9 => nodevals::Node_func,
            10 => nodevals::Node_ext_func,
            11 => nodevals::Node_builtin_func,
            12 => nodevals::Node_array_ref,
            13 => nodevals::Node_array_tree,
            14 => nodevals::Node_array_leaf,
            15 => nodevals::Node_dump_array,
            16 => nodevals::Node_arrayfor,
            17 => nodevals::Node_frame,
            18 => nodevals::Node_instruction,
            19 => nodevals::Node_final,
            _ => panic!("Invalid value for nodevals: {}", value),
        }
    }
}
impl AddAssign<u32> for nodevals {
    fn add_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for nodevals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for nodevals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for nodevals {
    fn div_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for nodevals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for nodevals {
    type Output = nodevals;
    fn add(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for nodevals {
    type Output = nodevals;
    fn sub(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for nodevals {
    type Output = nodevals;
    fn mul(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for nodevals {
    type Output = nodevals;
    fn div(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for nodevals {
    type Output = nodevals;
    fn rem(self, rhs: u32) -> nodevals {
        nodevals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2RustUnnamed_2,
    pub type_0: NODETYPE,
    pub flags: flagvals,
    pub valref: i64,
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
}
impl flagvals {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> flagvals {
        match value {
            524288 => flagvals::REGEX,
            262144 => flagvals::NUMCONSTSTR,
            131072 => flagvals::XARRAY,
            65536 => flagvals::HALFHAT,
            32768 => flagvals::ARRAYMAXED,
            16384 => flagvals::NULL_FIELD,
            8192 => flagvals::NO_EXT_SET,
            4096 => flagvals::MPZN,
            2048 => flagvals::MPFN,
            1024 => flagvals::WSTRCUR,
            512 => flagvals::INTIND,
            256 => flagvals::NUMINT,
            128 => flagvals::INTLSTR,
            64 => flagvals::BOOLVAL,
            32 => flagvals::USER_INPUT,
            16 => flagvals::NUMBER,
            8 => flagvals::NUMCUR,
            4 => flagvals::STRCUR,
            2 => flagvals::STRING,
            1 => flagvals::MALLOC,
            _ => panic!("Invalid value for flagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for flagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for flagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for flagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for flagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for flagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for flagvals {
    type Output = flagvals;
    fn add(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for flagvals {
    type Output = flagvals;
    fn sub(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for flagvals {
    type Output = flagvals;
    fn mul(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for flagvals {
    type Output = flagvals;
    fn div(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for flagvals {
    type Output = flagvals;
    fn rem(self, rhs: u32) -> flagvals {
        flagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
    pub sp: *mut i8,
    pub slen: size_t,
    pub idx: i32,
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
}
impl commenttype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            commenttype::FOR_COMMENT => 3,
            commenttype::BLOCK_COMMENT => 2,
            commenttype::EOL_COMMENT => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> commenttype {
        match value {
            3 => commenttype::FOR_COMMENT,
            2 => commenttype::BLOCK_COMMENT,
            1 => commenttype::EOL_COMMENT,
            _ => panic!("Invalid value for commenttype: {}", value),
        }
    }
}
impl AddAssign<u32> for commenttype {
    fn add_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for commenttype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for commenttype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for commenttype {
    fn div_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for commenttype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for commenttype {
    type Output = commenttype;
    fn add(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for commenttype {
    type Output = commenttype;
    fn sub(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for commenttype {
    type Output = commenttype;
    fn mul(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for commenttype {
    type Output = commenttype;
    fn div(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for commenttype {
    type Output = commenttype;
    fn rem(self, rhs: u32) -> commenttype {
        commenttype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub l: C2RustUnnamed_11,
    pub r: C2RustUnnamed_6,
    pub x: C2RustUnnamed_5,
    pub name: *mut i8,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: u64,
    pub reflags: reflagvals,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
}
impl reflagvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            reflagvals::CONSTANT => 1,
            reflagvals::FS_DFLT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> reflagvals {
        match value {
            1 => reflagvals::CONSTANT,
            2 => reflagvals::FS_DFLT,
            _ => panic!("Invalid value for reflagvals: {}", value),
        }
    }
}
impl AddAssign<u32> for reflagvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for reflagvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for reflagvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for reflagvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for reflagvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for reflagvals {
    type Output = reflagvals;
    fn add(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for reflagvals {
    type Output = reflagvals;
    fn sub(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for reflagvals {
    type Output = reflagvals;
    fn mul(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for reflagvals {
    type Output = reflagvals;
    fn div(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for reflagvals {
    type Output = reflagvals;
    fn rem(self, rhs: u32) -> reflagvals {
        reflagvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option<unsafe extern "C" fn() -> ()>,
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
}
impl opcodeval {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> opcodeval {
        match value {
            122 => opcodeval::Op_final,
            121 => opcodeval::Op_parens,
            120 => opcodeval::Op_cond_exp,
            119 => opcodeval::Op_K_function,
            118 => opcodeval::Op_K_else,
            117 => opcodeval::Op_K_if,
            116 => opcodeval::Op_K_switch,
            115 => opcodeval::Op_K_while,
            114 => opcodeval::Op_K_arrayfor,
            113 => opcodeval::Op_K_for,
            112 => opcodeval::Op_K_do,
            111 => opcodeval::Op_list,
            110 => opcodeval::Op_symbol,
            109 => opcodeval::Op_token,
            108 => opcodeval::Op_stop,
            107 => opcodeval::Op_atexit,
            106 => opcodeval::Op_lint_plus,
            105 => opcodeval::Op_lint,
            104 => opcodeval::Op_breakpoint,
            103 => opcodeval::Op_exec_count,
            102 => opcodeval::Op_comment,
            101 => opcodeval::Op_func,
            100 => opcodeval::Op_after_endfile,
            99 => opcodeval::Op_after_beginfile,
            98 => opcodeval::Op_subscript_assign,
            97 => opcodeval::Op_field_assign,
            96 => opcodeval::Op_var_assign,
            95 => opcodeval::Op_var_update,
            94 => opcodeval::Op_arrayfor_final,
            93 => opcodeval::Op_arrayfor_incr,
            92 => opcodeval::Op_arrayfor_init,
            91 => opcodeval::Op_newfile,
            90 => opcodeval::Op_get_record,
            89 => opcodeval::Op_jmp_false,
            88 => opcodeval::Op_jmp_true,
            87 => opcodeval::Op_jmp,
            86 => opcodeval::Op_pop,
            85 => opcodeval::Op_no_op,
            84 => opcodeval::Op_field_spec_lhs,
            83 => opcodeval::Op_subscript_lhs,
            82 => opcodeval::Op_push_lhs,
            81 => opcodeval::Op_push_param,
            80 => opcodeval::Op_push_array,
            79 => opcodeval::Op_push_re,
            78 => opcodeval::Op_push_i,
            77 => opcodeval::Op_push_arg_untyped,
            76 => opcodeval::Op_push_arg,
            75 => opcodeval::Op_push,
            74 => opcodeval::Op_indirect_func_call,
            73 => opcodeval::Op_func_call,
            72 => opcodeval::Op_in_array,
            71 => opcodeval::Op_ext_builtin,
            70 => opcodeval::Op_sub_builtin,
            69 => opcodeval::Op_builtin,
            68 => opcodeval::Op_K_namespace,
            67 => opcodeval::Op_K_nextfile,
            66 => opcodeval::Op_K_getline,
            65 => opcodeval::Op_K_getline_redir,
            64 => opcodeval::Op_K_delete_loop,
            63 => opcodeval::Op_K_delete,
            62 => opcodeval::Op_K_return_from_eval,
            61 => opcodeval::Op_K_return,
            60 => opcodeval::Op_K_exit,
            59 => opcodeval::Op_K_next,
            58 => opcodeval::Op_K_printf,
            57 => opcodeval::Op_K_print_rec,
            56 => opcodeval::Op_K_print,
            55 => opcodeval::Op_K_continue,
            54 => opcodeval::Op_K_break,
            53 => opcodeval::Op_K_default,
            52 => opcodeval::Op_K_case,
            51 => opcodeval::Op_rule,
            50 => opcodeval::Op_nomatch,
            49 => opcodeval::Op_match_rec,
            48 => opcodeval::Op_match,
            47 => opcodeval::Op_geq,
            46 => opcodeval::Op_leq,
            45 => opcodeval::Op_greater,
            44 => opcodeval::Op_less,
            43 => opcodeval::Op_notequal,
            42 => opcodeval::Op_equal,
            41 => opcodeval::Op_or_final,
            40 => opcodeval::Op_or,
            39 => opcodeval::Op_and_final,
            38 => opcodeval::Op_and,
            37 => opcodeval::Op_assign_concat,
            36 => opcodeval::Op_assign_exp,
            35 => opcodeval::Op_assign_minus,
            34 => opcodeval::Op_assign_plus,
            33 => opcodeval::Op_assign_mod,
            32 => opcodeval::Op_assign_quotient,
            31 => opcodeval::Op_assign_times,
            30 => opcodeval::Op_store_field_exp,
            29 => opcodeval::Op_store_field,
            28 => opcodeval::Op_store_sub,
            27 => opcodeval::Op_store_var,
            26 => opcodeval::Op_assign,
            25 => opcodeval::Op_not,
            24 => opcodeval::Op_field_spec,
            23 => opcodeval::Op_unary_plus,
            22 => opcodeval::Op_unary_minus,
            21 => opcodeval::Op_postdecrement,
            20 => opcodeval::Op_postincrement,
            19 => opcodeval::Op_predecrement,
            18 => opcodeval::Op_preincrement,
            17 => opcodeval::Op_sub_array,
            16 => opcodeval::Op_subscript,
            15 => opcodeval::Op_cond_pair,
            14 => opcodeval::Op_line_range,
            13 => opcodeval::Op_concat,
            12 => opcodeval::Op_exp_i,
            11 => opcodeval::Op_exp,
            10 => opcodeval::Op_minus_i,
            9 => opcodeval::Op_minus,
            8 => opcodeval::Op_plus_i,
            7 => opcodeval::Op_plus,
            6 => opcodeval::Op_mod_i,
            5 => opcodeval::Op_mod,
            4 => opcodeval::Op_quotient_i,
            3 => opcodeval::Op_quotient,
            2 => opcodeval::Op_times_i,
            1 => opcodeval::Op_times,
            0 => opcodeval::Op_illegal,
            _ => panic!("Invalid value for opcodeval: {}", value),
        }
    }
}
impl AddAssign<u32> for opcodeval {
    fn add_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for opcodeval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for opcodeval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for opcodeval {
    fn div_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for opcodeval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for opcodeval {
    type Output = opcodeval;
    fn add(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for opcodeval {
    type Output = opcodeval;
    fn sub(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for opcodeval {
    type Output = opcodeval;
    fn mul(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for opcodeval {
    type Output = opcodeval;
    fn div(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for opcodeval {
    type Output = opcodeval;
    fn rem(self, rhs: u32) -> opcodeval {
        opcodeval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub xl: i64,
    pub xn: *mut NODE,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
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
    pub fptr: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
    pub efptr: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: i64,
    pub ldl: exec_count_t,
    pub name: *mut i8,
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
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
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
    pub ll: i64,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const i8,
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
pub type afunc_t = Option<
    unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node,
>;
pub type Func_print = Option<unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum lintvals {
    LINT_no_effect = 2,
    LINT_assign_in_cond = 1,
    LINT_illegal = 0,
}
impl lintvals {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            lintvals::LINT_no_effect => 2,
            lintvals::LINT_assign_in_cond => 1,
            lintvals::LINT_illegal => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> lintvals {
        match value {
            2 => lintvals::LINT_no_effect,
            1 => lintvals::LINT_assign_in_cond,
            0 => lintvals::LINT_illegal,
            _ => panic!("Invalid value for lintvals: {}", value),
        }
    }
}
impl AddAssign<u32> for lintvals {
    fn add_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for lintvals {
    fn sub_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for lintvals {
    fn mul_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for lintvals {
    fn div_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for lintvals {
    fn rem_assign(&mut self, rhs: u32) {
        *self = lintvals::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for lintvals {
    type Output = lintvals;
    fn add(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for lintvals {
    type Output = lintvals;
    fn sub(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for lintvals {
    type Output = lintvals;
    fn mul(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for lintvals {
    type Output = lintvals;
    fn div(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for lintvals {
    type Output = lintvals;
    fn rem(self, rhs: u32) -> lintvals {
        lintvals::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type LINTTYPE = lintvals;
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
}
impl redirval {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> redirval {
        match value {
            6 => redirval::redirect_twoway,
            5 => redirval::redirect_input,
            4 => redirval::redirect_pipein,
            3 => redirval::redirect_pipe,
            2 => redirval::redirect_append,
            1 => redirval::redirect_output,
            0 => redirval::redirect_none,
            _ => panic!("Invalid value for redirval: {}", value),
        }
    }
}
impl AddAssign<u32> for redirval {
    fn add_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redirval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redirval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redirval {
    fn div_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redirval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redirval {
    type Output = redirval;
    fn add(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redirval {
    type Output = redirval;
    fn sub(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redirval {
    type Output = redirval;
    fn mul(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redirval {
    type Output = redirval;
    fn div(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redirval {
    type Output = redirval;
    fn rem(self, rhs: u32) -> redirval {
        redirval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type INSTRUCTION = exp_instruction;
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut i8,
    pub fullpath: *mut i8,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: i32,
    pub bufsize: size_t,
    pub buf: *mut i8,
    pub line_offset: *mut i32,
    pub fd: i32,
    pub maxlen: i32,
    pub fini_func: Option<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut i8,
    pub lexend: *mut i8,
    pub lexeme: *mut i8,
    pub lexptr_begin: *mut i8,
    pub lasttok: i32,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum srctype {
    SRC_CMDLINE = 1,
    SRC_STDIN,
    SRC_FILE,
    SRC_INC,
    SRC_EXTLIB,
}
impl srctype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            srctype::SRC_CMDLINE => 1,
            srctype::SRC_STDIN => 2,
            srctype::SRC_FILE => 3,
            srctype::SRC_INC => 4,
            srctype::SRC_EXTLIB => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> srctype {
        match value {
            1 => srctype::SRC_CMDLINE,
            2 => srctype::SRC_STDIN,
            3 => srctype::SRC_FILE,
            4 => srctype::SRC_INC,
            5 => srctype::SRC_EXTLIB,
            _ => panic!("Invalid value for srctype: {}", value),
        }
    }
}
impl AddAssign<u32> for srctype {
    fn add_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for srctype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for srctype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for srctype {
    fn div_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for srctype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for srctype {
    type Output = srctype;
    fn add(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for srctype {
    type Output = srctype;
    fn sub(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for srctype {
    type Output = srctype;
    fn mul(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for srctype {
    type Output = srctype;
    fn div(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for srctype {
    type Output = srctype;
    fn rem(self, rhs: u32) -> srctype {
        srctype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type SRCFILE = srcfile;
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
    pub name: *const i8,
    pub highwater: i64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum block_id {
    BLOCK_NODE = 0,
    BLOCK_BUCKET,
    BLOCK_MAX,
}
impl block_id {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            block_id::BLOCK_NODE => 0,
            block_id::BLOCK_BUCKET => 1,
            block_id::BLOCK_MAX => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> block_id {
        match value {
            0 => block_id::BLOCK_NODE,
            1 => block_id::BLOCK_BUCKET,
            2 => block_id::BLOCK_MAX,
            _ => panic!("Invalid value for block_id: {}", value),
        }
    }
}
impl AddAssign<u32> for block_id {
    fn add_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for block_id {
    fn sub_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for block_id {
    fn mul_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for block_id {
    fn div_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for block_id {
    fn rem_assign(&mut self, rhs: u32) {
        *self = block_id::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for block_id {
    type Output = block_id;
    fn add(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for block_id {
    type Output = block_id;
    fn sub(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for block_id {
    type Output = block_id;
    fn mul(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for block_id {
    type Output = block_id;
    fn div(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for block_id {
    type Output = block_id;
    fn rem(self, rhs: u32) -> block_id {
        block_id::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
}
impl do_flag_values {
    fn to_libc_c_uint(self) -> u32 {
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
    fn from_libc_c_uint(value: u32) -> do_flag_values {
        match value {
            32768 => do_flag_values::DO_MPFR,
            16384 => do_flag_values::DO_DEBUG,
            8192 => do_flag_values::DO_PROFILE,
            4096 => do_flag_values::DO_SANDBOX,
            2048 => do_flag_values::DO_TIDY_MEM,
            1024 => do_flag_values::DO_DUMP_VARS,
            512 => do_flag_values::DO_PRETTY_PRINT,
            256 => do_flag_values::DO_INTERVALS,
            128 => do_flag_values::DO_NON_DEC_DATA,
            64 => do_flag_values::DO_INTL,
            32 => do_flag_values::DO_POSIX,
            16 => do_flag_values::DO_TRADITIONAL,
            8 => do_flag_values::DO_LINT_OLD,
            4 => do_flag_values::DO_LINT_ALL,
            2 => do_flag_values::DO_LINT_EXTENSIONS,
            1 => do_flag_values::DO_LINT_INVALID,
            0 => do_flag_values::DO_FLAG_NONE,
            _ => panic!("Invalid value for do_flag_values: {}", value),
        }
    }
}
impl AddAssign<u32> for do_flag_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for do_flag_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for do_flag_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for do_flag_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for do_flag_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for do_flag_values {
    type Output = do_flag_values;
    fn add(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for do_flag_values {
    type Output = do_flag_values;
    fn sub(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for do_flag_values {
    type Output = do_flag_values;
    fn mul(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for do_flag_values {
    type Output = do_flag_values;
    fn div(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for do_flag_values {
    type Output = do_flag_values;
    fn rem(self, rhs: u32) -> do_flag_values {
        do_flag_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdesc {
    pub name: *mut i8,
    pub used: libc::c_short,
    pub defined: libc::c_short,
    pub extension: libc::c_short,
    pub next: *mut fdesc,
}
pub type yy_state_t = yytype_int16;
pub type yytype_int16 = libc::c_short;
pub type yysymbol_kind_t = i32;
pub const YYSYMBOL_comma: yysymbol_kind_t = 148;
pub const YYSYMBOL_colon: yysymbol_kind_t = 147;
pub const YYSYMBOL_semi: yysymbol_kind_t = 146;
pub const YYSYMBOL_opt_semi: yysymbol_kind_t = 145;
pub const YYSYMBOL_r_paren: yysymbol_kind_t = 144;
pub const YYSYMBOL_r_brace: yysymbol_kind_t = 143;
pub const YYSYMBOL_l_brace: yysymbol_kind_t = 142;
pub const YYSYMBOL_opt_incdec: yysymbol_kind_t = 141;
pub const YYSYMBOL_variable: yysymbol_kind_t = 140;
pub const YYSYMBOL_simple_variable: yysymbol_kind_t = 139;
pub const YYSYMBOL_subscript_list: yysymbol_kind_t = 138;
pub const YYSYMBOL_subscript: yysymbol_kind_t = 137;
pub const YYSYMBOL_bracketed_exp_list: yysymbol_kind_t = 136;
pub const YYSYMBOL_delete_exp_list: yysymbol_kind_t = 135;
pub const YYSYMBOL_delete_subscript: yysymbol_kind_t = 134;
pub const YYSYMBOL_delete_subscript_list: yysymbol_kind_t = 133;
pub const YYSYMBOL_opt_variable: yysymbol_kind_t = 132;
pub const YYSYMBOL_direct_func_call: yysymbol_kind_t = 131;
pub const YYSYMBOL_func_call: yysymbol_kind_t = 130;
pub const YYSYMBOL_non_post_simp_exp: yysymbol_kind_t = 129;
pub const YYSYMBOL_simp_exp_nc: yysymbol_kind_t = 128;
pub const YYSYMBOL_simp_exp: yysymbol_kind_t = 127;
pub const YYSYMBOL_common_exp: yysymbol_kind_t = 126;
pub const YYSYMBOL_a_relop: yysymbol_kind_t = 125;
pub const YYSYMBOL_relop_or_less: yysymbol_kind_t = 124;
pub const YYSYMBOL_assign_operator: yysymbol_kind_t = 123;
pub const YYSYMBOL_exp: yysymbol_kind_t = 122;
pub const YYSYMBOL_opt_fcall_exp: yysymbol_kind_t = 121;
pub const YYSYMBOL_fcall_exp: yysymbol_kind_t = 120;
pub const YYSYMBOL_fcall_expression_list: yysymbol_kind_t = 119;
pub const YYSYMBOL_opt_fcall_expression_list: yysymbol_kind_t = 118;
pub const YYSYMBOL_expression_list: yysymbol_kind_t = 117;
pub const YYSYMBOL_opt_expression_list: yysymbol_kind_t = 116;
pub const YYSYMBOL_opt_exp: yysymbol_kind_t = 115;
pub const YYSYMBOL_param_list: yysymbol_kind_t = 114;
pub const YYSYMBOL_opt_param_list: yysymbol_kind_t = 113;
pub const YYSYMBOL_input_redir: yysymbol_kind_t = 112;
pub const YYSYMBOL_opt_nls: yysymbol_kind_t = 111;
pub const YYSYMBOL_nls: yysymbol_kind_t = 110;
pub const YYSYMBOL_if_statement: yysymbol_kind_t = 109;
pub const YYSYMBOL_108_6: yysymbol_kind_t = 108;
pub const YYSYMBOL_output_redir: yysymbol_kind_t = 107;
pub const YYSYMBOL_print_expression_list: yysymbol_kind_t = 106;
pub const YYSYMBOL_print: yysymbol_kind_t = 105;
pub const YYSYMBOL_case_value: yysymbol_kind_t = 104;
pub const YYSYMBOL_case_statement: yysymbol_kind_t = 103;
pub const YYSYMBOL_case_statements: yysymbol_kind_t = 102;
pub const YYSYMBOL_opt_simple_stmt: yysymbol_kind_t = 101;
pub const YYSYMBOL_100_5: yysymbol_kind_t = 100;
pub const YYSYMBOL_99_4: yysymbol_kind_t = 99;
pub const YYSYMBOL_simple_stmt: yysymbol_kind_t = 98;
pub const YYSYMBOL_97_3: yysymbol_kind_t = 97;
pub const YYSYMBOL_non_compound_stmt: yysymbol_kind_t = 96;
pub const YYSYMBOL_statement: yysymbol_kind_t = 95;
pub const YYSYMBOL_statement_term: yysymbol_kind_t = 94;
pub const YYSYMBOL_statements: yysymbol_kind_t = 93;
pub const YYSYMBOL_a_slash: yysymbol_kind_t = 92;
pub const YYSYMBOL_typed_regexp: yysymbol_kind_t = 91;
pub const YYSYMBOL_90_2: yysymbol_kind_t = 90;
pub const YYSYMBOL_regexp: yysymbol_kind_t = 89;
pub const YYSYMBOL_88_1: yysymbol_kind_t = 88;
pub const YYSYMBOL_function_prologue: yysymbol_kind_t = 87;
pub const YYSYMBOL_lex_builtin: yysymbol_kind_t = 86;
pub const YYSYMBOL_func_name: yysymbol_kind_t = 85;
pub const YYSYMBOL_action: yysymbol_kind_t = 84;
pub const YYSYMBOL_pattern: yysymbol_kind_t = 83;
pub const YYSYMBOL_namespace: yysymbol_kind_t = 82;
pub const YYSYMBOL_library: yysymbol_kind_t = 81;
pub const YYSYMBOL_source: yysymbol_kind_t = 80;
pub const YYSYMBOL_rule: yysymbol_kind_t = 79;
pub const YYSYMBOL_program: yysymbol_kind_t = 78;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 77;
pub const YYSYMBOL_76_: yysymbol_kind_t = 76;
pub const YYSYMBOL_75_: yysymbol_kind_t = 75;
pub const YYSYMBOL_74_: yysymbol_kind_t = 74;
pub const YYSYMBOL_73_: yysymbol_kind_t = 73;
pub const YYSYMBOL_72_: yysymbol_kind_t = 72;
pub const YYSYMBOL_71_: yysymbol_kind_t = 71;
pub const YYSYMBOL_70_: yysymbol_kind_t = 70;
pub const YYSYMBOL_69_: yysymbol_kind_t = 69;
pub const YYSYMBOL_68_: yysymbol_kind_t = 68;
pub const YYSYMBOL_67_: yysymbol_kind_t = 67;
pub const YYSYMBOL_UNARY: yysymbol_kind_t = 66;
pub const YYSYMBOL_65_: yysymbol_kind_t = 65;
pub const YYSYMBOL_64_: yysymbol_kind_t = 64;
pub const YYSYMBOL_63_: yysymbol_kind_t = 63;
pub const YYSYMBOL_62_: yysymbol_kind_t = 62;
pub const YYSYMBOL_61_: yysymbol_kind_t = 61;
pub const YYSYMBOL_60_: yysymbol_kind_t = 60;
pub const YYSYMBOL_59_: yysymbol_kind_t = 59;
pub const YYSYMBOL_58_: yysymbol_kind_t = 58;
pub const YYSYMBOL_57_: yysymbol_kind_t = 57;
pub const YYSYMBOL_56_: yysymbol_kind_t = 56;
pub const YYSYMBOL_55_: yysymbol_kind_t = 55;
pub const YYSYMBOL_SLASH_BEFORE_EQUAL: yysymbol_kind_t = 54;
pub const YYSYMBOL_NEWLINE: yysymbol_kind_t = 53;
pub const YYSYMBOL_LEX_NAMESPACE: yysymbol_kind_t = 52;
pub const YYSYMBOL_LEX_LOAD: yysymbol_kind_t = 51;
pub const YYSYMBOL_LEX_EVAL: yysymbol_kind_t = 50;
pub const YYSYMBOL_LEX_INCLUDE: yysymbol_kind_t = 49;
pub const YYSYMBOL_LEX_EOF: yysymbol_kind_t = 48;
pub const YYSYMBOL_LEX_LENGTH: yysymbol_kind_t = 47;
pub const YYSYMBOL_LEX_BUILTIN: yysymbol_kind_t = 46;
pub const YYSYMBOL_DECREMENT: yysymbol_kind_t = 45;
pub const YYSYMBOL_INCREMENT: yysymbol_kind_t = 44;
pub const YYSYMBOL_LEX_OR: yysymbol_kind_t = 43;
pub const YYSYMBOL_LEX_AND: yysymbol_kind_t = 42;
pub const YYSYMBOL_LEX_IN: yysymbol_kind_t = 41;
pub const YYSYMBOL_LEX_NEXTFILE: yysymbol_kind_t = 40;
pub const YYSYMBOL_LEX_GETLINE: yysymbol_kind_t = 39;
pub const YYSYMBOL_LEX_ENDFILE: yysymbol_kind_t = 38;
pub const YYSYMBOL_LEX_BEGINFILE: yysymbol_kind_t = 37;
pub const YYSYMBOL_LEX_FUNCTION: yysymbol_kind_t = 36;
pub const YYSYMBOL_LEX_EXIT: yysymbol_kind_t = 35;
pub const YYSYMBOL_LEX_NEXT: yysymbol_kind_t = 34;
pub const YYSYMBOL_LEX_PRINTF: yysymbol_kind_t = 33;
pub const YYSYMBOL_LEX_PRINT: yysymbol_kind_t = 32;
pub const YYSYMBOL_LEX_CONTINUE: yysymbol_kind_t = 31;
pub const YYSYMBOL_LEX_BREAK: yysymbol_kind_t = 30;
pub const YYSYMBOL_LEX_FOR: yysymbol_kind_t = 29;
pub const YYSYMBOL_LEX_DO: yysymbol_kind_t = 28;
pub const YYSYMBOL_LEX_WHILE: yysymbol_kind_t = 27;
pub const YYSYMBOL_LEX_DEFAULT: yysymbol_kind_t = 26;
pub const YYSYMBOL_LEX_CASE: yysymbol_kind_t = 25;
pub const YYSYMBOL_LEX_SWITCH: yysymbol_kind_t = 24;
pub const YYSYMBOL_LEX_DELETE: yysymbol_kind_t = 23;
pub const YYSYMBOL_LEX_RETURN: yysymbol_kind_t = 22;
pub const YYSYMBOL_LEX_ELSE: yysymbol_kind_t = 21;
pub const YYSYMBOL_LEX_IF: yysymbol_kind_t = 20;
pub const YYSYMBOL_LEX_END: yysymbol_kind_t = 19;
pub const YYSYMBOL_LEX_BEGIN: yysymbol_kind_t = 18;
pub const YYSYMBOL_SUBSCRIPT: yysymbol_kind_t = 17;
pub const YYSYMBOL_CONCAT_OP: yysymbol_kind_t = 16;
pub const YYSYMBOL_MATCHOP: yysymbol_kind_t = 15;
pub const YYSYMBOL_ASSIGN: yysymbol_kind_t = 14;
pub const YYSYMBOL_ASSIGNOP: yysymbol_kind_t = 13;
pub const YYSYMBOL_IO_IN: yysymbol_kind_t = 12;
pub const YYSYMBOL_IO_OUT: yysymbol_kind_t = 11;
pub const YYSYMBOL_RELOP: yysymbol_kind_t = 10;
pub const YYSYMBOL_TYPED_REGEXP: yysymbol_kind_t = 9;
pub const YYSYMBOL_YSTRING: yysymbol_kind_t = 8;
pub const YYSYMBOL_YNUMBER: yysymbol_kind_t = 7;
pub const YYSYMBOL_FILENAME: yysymbol_kind_t = 6;
pub const YYSYMBOL_REGEXP: yysymbol_kind_t = 5;
pub const YYSYMBOL_NAME: yysymbol_kind_t = 4;
pub const YYSYMBOL_FUNC_CALL: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_uint8 = u8;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = i32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum defref {
    FUNC_DEFINE,
    FUNC_USE,
    FUNC_EXT,
}
impl defref {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            defref::FUNC_DEFINE => 0,
            defref::FUNC_USE => 1,
            defref::FUNC_EXT => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> defref {
        match value {
            0 => defref::FUNC_DEFINE,
            1 => defref::FUNC_USE,
            2 => defref::FUNC_EXT,
            _ => panic!("Invalid value for defref: {}", value),
        }
    }
}
impl AddAssign<u32> for defref {
    fn add_assign(&mut self, rhs: u32) {
        *self = defref::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for defref {
    fn sub_assign(&mut self, rhs: u32) {
        *self = defref::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for defref {
    fn mul_assign(&mut self, rhs: u32) {
        *self = defref::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for defref {
    fn div_assign(&mut self, rhs: u32) {
        *self = defref::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for defref {
    fn rem_assign(&mut self, rhs: u32) {
        *self = defref::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for defref {
    type Output = defref;
    fn add(self, rhs: u32) -> defref {
        defref::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for defref {
    type Output = defref;
    fn sub(self, rhs: u32) -> defref {
        defref::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for defref {
    type Output = defref;
    fn mul(self, rhs: u32) -> defref {
        defref::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for defref {
    type Output = defref;
    fn div(self, rhs: u32) -> defref {
        defref::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for defref {
    type Output = defref;
    fn rem(self, rhs: u32) -> defref {
        defref::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub operator: *const i8,
    pub value: OPCODE,
    pub class: i32,
    pub flags: u32,
    pub ptr: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
    pub ptr2: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_12 {
    DONT_CHECK = 2,
    FUNC_BODY = 1,
    FUNC_HEADER = 0,
}
impl C2RustUnnamed_12 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_12::DONT_CHECK => 2,
            C2RustUnnamed_12::FUNC_BODY => 1,
            C2RustUnnamed_12::FUNC_HEADER => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_12 {
        match value {
            2 => C2RustUnnamed_12::DONT_CHECK,
            1 => C2RustUnnamed_12::FUNC_BODY,
            0 => C2RustUnnamed_12::FUNC_HEADER,
            _ => panic!("Invalid value for C2RustUnnamed_12: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_12 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_12 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_12 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_12 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_12 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn add(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn sub(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn mul(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn div(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_12 {
    type Output = C2RustUnnamed_12;
    fn rem(self, rhs: u32) -> C2RustUnnamed_12 {
        C2RustUnnamed_12::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum defline {
    FIRST_LINE,
    LAST_LINE,
}
impl defline {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            defline::FIRST_LINE => 0,
            defline::LAST_LINE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> defline {
        match value {
            0 => defline::FIRST_LINE,
            1 => defline::LAST_LINE,
            _ => panic!("Invalid value for defline: {}", value),
        }
    }
}
impl AddAssign<u32> for defline {
    fn add_assign(&mut self, rhs: u32) {
        *self = defline::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for defline {
    fn sub_assign(&mut self, rhs: u32) {
        *self = defline::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for defline {
    fn mul_assign(&mut self, rhs: u32) {
        *self = defline::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for defline {
    fn div_assign(&mut self, rhs: u32) {
        *self = defline::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for defline {
    fn rem_assign(&mut self, rhs: u32) {
        *self = defline::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for defline {
    type Output = defline;
    fn add(self, rhs: u32) -> defline {
        defline::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for defline {
    type Output = defline;
    fn sub(self, rhs: u32) -> defline {
        defline::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for defline {
    type Output = defline;
    fn mul(self, rhs: u32) -> defline {
        defline::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for defline {
    type Output = defline;
    fn div(self, rhs: u32) -> defline {
        defline::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for defline {
    type Output = defline;
    fn rem(self, rhs: u32) -> defline {
        defline::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: *mut INSTRUCTION,
}
pub type builtin_func_t = Option<unsafe extern "C" fn(i32) -> *mut NODE>;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn putchar(mut __c: i32) -> i32 {
    return _IO_putc(__c, stdout);
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
#[inline]
unsafe extern "C" fn mbrlen(
    mut __s: *const i8,
    mut __n: size_t,
    mut __ps: *mut mbstate_t,
) -> size_t {
    return if !__ps.is_null() {
        mbrtowc(0 as *mut wchar_t, __s, __n, __ps)
    } else {
        __mbrlen(__s, __n, 0 as *mut mbstate_t)
    };
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut libc::c_void,
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2088 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: erealloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2092 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot reallocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2052 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: emalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2056 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut where_0: *const i8,
    mut var: *const i8,
    mut file: *const i8,
    mut line: i32,
) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if count == 0 as i32 as u64 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2070 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            b"%s:%d: ezalloc called with zero bytes\0" as *const u8 as *const i8,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as i32 as size_t, count);
    if ret.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 2074 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            line,
            where_0,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    if !r.is_null()
        && {
            (*r).valref -= 1;
            (*r).valref <= 0 as i32 as i64
        }
    {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn force_string_fmt(
    mut s: *mut NODE,
    mut fmtstr: *const i8,
    mut fmtidx: i32,
) -> *mut NODE {
    if (*s).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        (*s).type_0 = nodevals::Node_val;
        (*s).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*s).flags as u32 & !(flagvals::NUMBER as i32) as u32);
        return s;
    }
    if (*s).flags as u32 & flagvals::STRCUR as i32 as u32 != 0 as i32 as u32
        && ((*s).sub.val.idx == -(1 as i32) || (*s).sub.val.idx == fmtidx)
    {
        return s;
    }
    return format_val.expect("non-null function pointer")(fmtstr, fmtidx, s);
}
static mut trailing_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut outer_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut interblock_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut pending_comment: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut namespace_chain: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut at_seen: i32 = 0 as i32;
static mut want_source: bool = 0 as i32 != 0;
static mut want_namespace: bool = 0 as i32 != 0;
static mut want_regexp: bool = 0 as i32 != 0;
static mut want_param_names: C2RustUnnamed_12 = C2RustUnnamed_12::DONT_CHECK;
static mut in_function: bool = false;
static mut rule: i32 = 0 as i32;
#[no_mangle]
pub static mut ruletab: [*const i8; 6] = [
    b"?\0" as *const u8 as *const i8,
    b"defrule::BEGIN\0" as *const u8 as *const i8,
    b"defrule::Rule\0" as *const u8 as *const i8,
    b"defrule::END\0" as *const u8 as *const i8,
    b"defrule::BEGINFILE\0" as *const u8 as *const i8,
    b"defrule::ENDFILE\0" as *const u8 as *const i8,
];
static mut in_print: bool = 0 as i32 != 0;
static mut in_parens: i32 = 0 as i32;
static mut sub_counter: i32 = 0 as i32;
static mut lexptr: *mut i8 = 0 as *const i8 as *mut i8;
static mut lexend: *mut i8 = 0 as *const i8 as *mut i8;
static mut lexptr_begin: *mut i8 = 0 as *const i8 as *mut i8;
static mut lexeme: *mut i8 = 0 as *const i8 as *mut i8;
static mut lexeof: bool = false;
static mut thisline: *mut i8 = 0 as *const i8 as *mut i8;
static mut in_braces: i32 = 0 as i32;
static mut lastline: i32 = 0 as i32;
static mut firstline: i32 = 0 as i32;
static mut sourcefile: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut lasttok: i32 = 0 as i32;
static mut eof_warned: bool = 0 as i32 != 0;
static mut break_allowed: i32 = 0;
static mut continue_allowed: i32 = 0;
static mut tokstart: *mut i8 = 0 as *const i8 as *mut i8;
static mut tok: *mut i8 = 0 as *const i8 as *mut i8;
static mut tokend: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut errcount: i32 = 0 as i32;
#[no_mangle]
pub static mut awk_namespace: [i8; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"awk\0")
};
#[no_mangle]
pub static mut current_namespace: *const i8 = unsafe { awk_namespace.as_ptr() };
#[no_mangle]
pub static mut namespace_changed: bool = 0 as i32 != 0;
static mut rule_block: [*mut INSTRUCTION; 6] = [0 as *const INSTRUCTION
    as *mut INSTRUCTION; 6];
static mut ip_rec: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_newfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_atexit: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_end: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_endfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut ip_beginfile: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
#[no_mangle]
pub static mut main_beginfile: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
static mut called_from_eval: bool = 0 as i32 != 0;
static mut yytranslate: [yytype_int8; 311] = [
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    65 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    68 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    69 as i32 as yytype_int8,
    70 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    60 as i32 as yytype_int8,
    57 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    56 as i32 as yytype_int8,
    76 as i32 as yytype_int8,
    58 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    59 as i32 as yytype_int8,
    55 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    72 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    73 as i32 as yytype_int8,
    67 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    74 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    75 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    7 as i32 as yytype_int8,
    8 as i32 as yytype_int8,
    9 as i32 as yytype_int8,
    10 as i32 as yytype_int8,
    11 as i32 as yytype_int8,
    12 as i32 as yytype_int8,
    13 as i32 as yytype_int8,
    14 as i32 as yytype_int8,
    15 as i32 as yytype_int8,
    16 as i32 as yytype_int8,
    17 as i32 as yytype_int8,
    18 as i32 as yytype_int8,
    19 as i32 as yytype_int8,
    20 as i32 as yytype_int8,
    21 as i32 as yytype_int8,
    22 as i32 as yytype_int8,
    23 as i32 as yytype_int8,
    24 as i32 as yytype_int8,
    25 as i32 as yytype_int8,
    26 as i32 as yytype_int8,
    27 as i32 as yytype_int8,
    28 as i32 as yytype_int8,
    29 as i32 as yytype_int8,
    30 as i32 as yytype_int8,
    31 as i32 as yytype_int8,
    32 as i32 as yytype_int8,
    33 as i32 as yytype_int8,
    34 as i32 as yytype_int8,
    35 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    40 as i32 as yytype_int8,
    41 as i32 as yytype_int8,
    42 as i32 as yytype_int8,
    43 as i32 as yytype_int8,
    44 as i32 as yytype_int8,
    45 as i32 as yytype_int8,
    46 as i32 as yytype_int8,
    47 as i32 as yytype_int8,
    48 as i32 as yytype_int8,
    49 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    51 as i32 as yytype_int8,
    52 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    66 as i32 as yytype_int8,
];
static mut yypact: [yytype_int16; 356] = [
    -(276 as i32) as yytype_int16,
    315 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(45 as i32) as yytype_int16,
    -(41 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    133 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    13 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    -(13 as i32) as yytype_int16,
    -(7 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    1074 as i32 as yytype_int16,
    817 as i32 as yytype_int16,
    172 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(23 as i32) as yytype_int16,
    -(10 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    29 as i32 as yytype_int16,
    1112 as i32 as yytype_int16,
    973 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    310 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    137 as i32 as yytype_int16,
    741 as i32 as yytype_int16,
    817 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    64 as i32 as yytype_int16,
    84 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    98 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    741 as i32 as yytype_int16,
    741 as i32 as yytype_int16,
    173 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    125 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    15 as i32 as yytype_int16,
    1016 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    151 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    29 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    198 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1001 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    175 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    209 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    208 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    142 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    73 as i32 as yytype_int16,
    1141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    142 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    1016 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    186 as i32 as yytype_int16,
    845 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    8 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    34 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    59 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    541 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    11 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    287 as i32 as yytype_int16,
    130 as i32 as yytype_int16,
    1131 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    139 as i32 as yytype_int16,
    1141 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1001 as i32 as yytype_int16,
    769 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    45 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    174 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    179 as i32 as yytype_int16,
    183 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    90 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    90 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    182 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    142 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    1046 as i32 as yytype_int16,
    1001 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    251 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    659 as i32 as yytype_int16,
    894 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    90 as i32 as yytype_int16,
    1141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    589 as i32 as yytype_int16,
    541 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1141 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    46 as i32 as yytype_int16,
    1016 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    90 as i32 as yytype_int16,
    -(41 as i32) as yytype_int16,
    189 as i32 as yytype_int16,
    1016 as i32 as yytype_int16,
    1016 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    -(15 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    182 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    817 as i32 as yytype_int16,
    263 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    162 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    659 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(10 as i32) as yytype_int16,
    659 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    693 as i32 as yytype_int16,
    186 as i32 as yytype_int16,
    1046 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    1016 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    1089 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    973 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    260 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    659 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    928 as i32 as yytype_int16,
    659 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    340 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    212 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    928 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    281 as i32 as yytype_int16,
    282 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    222 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    142 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    141 as i32 as yytype_int16,
    659 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    389 as i32 as yytype_int16,
    659 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    465 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 356] = [
    2 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    195 as i32 as yytype_uint8,
    177 as i32 as yytype_uint8,
    178 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    184 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    172 as i32 as yytype_uint8,
    5 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    3 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    167 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    149 as i32 as yytype_uint8,
    173 as i32 as yytype_uint8,
    181 as i32 as yytype_uint8,
    197 as i32 as yytype_uint8,
    174 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    192 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    196 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    30 as i32 as yytype_uint8,
    34 as i32 as yytype_uint8,
    35 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    32 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    185 as i32 as yytype_uint8,
    175 as i32 as yytype_uint8,
    176 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    180 as i32 as yytype_uint8,
    174 as i32 as yytype_uint8,
    179 as i32 as yytype_uint8,
    168 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    201 as i32 as yytype_uint8,
    174 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    182 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    207 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    9 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    157 as i32 as yytype_uint8,
    158 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    40 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    194 as i32 as yytype_uint8,
    193 as i32 as yytype_uint8,
    33 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    156 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    199 as i32 as yytype_uint8,
    200 as i32 as yytype_uint8,
    198 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    204 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    169 as i32 as yytype_uint8,
    15 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    202 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    209 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    184 as i32 as yytype_uint8,
    154 as i32 as yytype_uint8,
    155 as i32 as yytype_uint8,
    151 as i32 as yytype_uint8,
    152 as i32 as yytype_uint8,
    153 as i32 as yytype_uint8,
    150 as i32 as yytype_uint8,
    165 as i32 as yytype_uint8,
    166 as i32 as yytype_uint8,
    162 as i32 as yytype_uint8,
    163 as i32 as yytype_uint8,
    164 as i32 as yytype_uint8,
    161 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    183 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    191 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    170 as i32 as yytype_uint8,
    171 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    10 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    11 as i32 as yytype_uint8,
    20 as i32 as yytype_uint8,
    12 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    66 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    205 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    160 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    159 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    203 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    206 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    186 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    195 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    188 as i32 as yytype_uint8,
    190 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    187 as i32 as yytype_uint8,
    189 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    208 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 72] = [
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    253 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(32 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(77 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(208 as i32) as yytype_int16,
    -(55 as i32) as yytype_int16,
    -(68 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(237 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(275 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    47 as i32 as yytype_int16,
    -(48 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(43 as i32) as yytype_int16,
    157 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(157 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    0 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    268 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    2 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    -(38 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(81 as i32) as yytype_int16,
    -(2 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(27 as i32) as yytype_int16,
    -(213 as i32) as yytype_int16,
    -(66 as i32) as yytype_int16,
    -(276 as i32) as yytype_int16,
    -(22 as i32) as yytype_int16,
    -(30 as i32) as yytype_int16,
    -(29 as i32) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 72] = [
    0 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    146 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    78 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    187 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    84 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    79 as i32 as yytype_int16,
    217 as i32 as yytype_int16,
    218 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    219 as i32 as yytype_int16,
    252 as i32 as yytype_int16,
    264 as i32 as yytype_int16,
    271 as i32 as yytype_int16,
    316 as i32 as yytype_int16,
    325 as i32 as yytype_int16,
    338 as i32 as yytype_int16,
    220 as i32 as yytype_int16,
    274 as i32 as yytype_int16,
    296 as i32 as yytype_int16,
    306 as i32 as yytype_int16,
    221 as i32 as yytype_int16,
    153 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    247 as i32 as yytype_int16,
    275 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    121 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    263 as i32 as yytype_int16,
    222 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    95 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    284 as i32 as yytype_int16,
    285 as i32 as yytype_int16,
    286 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    223 as i32 as yytype_int16,
    224 as i32 as yytype_int16,
    143 as i32 as yytype_int16,
    254 as i32 as yytype_int16,
    82 as i32 as yytype_int16,
    340 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
];
static mut yytable: [yytype_int16; 1201] = [
    34 as i32 as yytype_int16,
    125 as i32 as yytype_int16,
    81 as i32 as yytype_int16,
    81 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    270 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    128 as i32 as yytype_int16,
    195 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    238 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    139 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    127 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    292 as i32 as yytype_int16,
    228 as i32 as yytype_int16,
    230 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    197 as i32 as yytype_int16,
    180 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    277 as i32 as yytype_int16,
    65 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    126 as i32 as yytype_int16,
    164 as i32 as yytype_int16,
    330 as i32 as yytype_int16,
    231 as i32 as yytype_int16,
    280 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    232 as i32 as yytype_int16,
    281 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    342 as i32 as yytype_int16,
    183 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    -(13 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    147 as i32 as yytype_int16,
    189 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    148 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    139 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    298 as i32 as yytype_int16,
    259 as i32 as yytype_int16,
    239 as i32 as yytype_int16,
    262 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    85 as i32 as yytype_int16,
    270 as i32 as yytype_int16,
    -(13 as i32) as yytype_int16,
    140 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    -(16 as i32) as yytype_int16,
    161 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    196 as i32 as yytype_int16,
    270 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    198 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    200 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    -(16 as i32) as yytype_int16,
    235 as i32 as yytype_int16,
    -(19 as i32) as yytype_int16,
    341 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    -(100 as i32) as yytype_int16,
    182 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    169 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    171 as i32 as yytype_int16,
    172 as i32 as yytype_int16,
    173 as i32 as yytype_int16,
    174 as i32 as yytype_int16,
    175 as i32 as yytype_int16,
    176 as i32 as yytype_int16,
    177 as i32 as yytype_int16,
    178 as i32 as yytype_int16,
    179 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    -(19 as i32) as yytype_int16,
    48 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    191 as i32 as yytype_int16,
    351 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    194 as i32 as yytype_int16,
    354 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    -(118 as i32) as yytype_int16,
    87 as i32 as yytype_int16,
    186 as i32 as yytype_int16,
    -(101 as i32) as yytype_int16,
    188 as i32 as yytype_int16,
    -(119 as i32) as yytype_int16,
    112 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    150 as i32 as yytype_int16,
    130 as i32 as yytype_int16,
    244 as i32 as yytype_int16,
    245 as i32 as yytype_int16,
    131 as i32 as yytype_int16,
    151 as i32 as yytype_int16,
    246 as i32 as yytype_int16,
    242 as i32 as yytype_int16,
    249 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    115 as i32 as yytype_int16,
    139 as i32 as yytype_int16,
    251 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    258 as i32 as yytype_int16,
    250 as i32 as yytype_int16,
    136 as i32 as yytype_int16,
    137 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    -(110 as i32) as yytype_int16,
    268 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    256 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    115 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    116 as i32 as yytype_int16,
    272 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    282 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    -(119 as i32) as yytype_int16,
    -(119 as i32) as yytype_int16,
    80 as i32 as yytype_int16,
    289 as i32 as yytype_int16,
    290 as i32 as yytype_int16,
    255 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    260 as i32 as yytype_int16,
    278 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    283 as i32 as yytype_int16,
    276 as i32 as yytype_int16,
    279 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    140 as i32 as yytype_int16,
    248 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    -(110 as i32) as yytype_int16,
    133 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    73 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    257 as i32 as yytype_int16,
    287 as i32 as yytype_int16,
    192 as i32 as yytype_int16,
    305 as i32 as yytype_int16,
    307 as i32 as yytype_int16,
    294 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    -(110 as i32) as yytype_int16,
    309 as i32 as yytype_int16,
    297 as i32 as yytype_int16,
    261 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    311 as i32 as yytype_int16,
    -(110 as i32) as yytype_int16,
    266 as i32 as yytype_int16,
    267 as i32 as yytype_int16,
    300 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    236 as i32 as yytype_int16,
    317 as i32 as yytype_int16,
    304 as i32 as yytype_int16,
    337 as i32 as yytype_int16,
    287 as i32 as yytype_int16,
    240 as i32 as yytype_int16,
    293 as i32 as yytype_int16,
    328 as i32 as yytype_int16,
    126 as i32 as yytype_int16,
    241 as i32 as yytype_int16,
    331 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    265 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    288 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    322 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    318 as i32 as yytype_int16,
    343 as i32 as yytype_int16,
    320 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    321 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    327 as i32 as yytype_int16,
    291 as i32 as yytype_int16,
    329 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    308 as i32 as yytype_int16,
    295 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    348 as i32 as yytype_int16,
    315 as i32 as yytype_int16,
    339 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    302 as i32 as yytype_int16,
    353 as i32 as yytype_int16,
    313 as i32 as yytype_int16,
    83 as i32 as yytype_int16,
    355 as i32 as yytype_int16,
    323 as i32 as yytype_int16,
    324 as i32 as yytype_int16,
    216 as i32 as yytype_int16,
    344 as i32 as yytype_int16,
    345 as i32 as yytype_int16,
    319 as i32 as yytype_int16,
    336 as i32 as yytype_int16,
    347 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    349 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    350 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    352 as i32 as yytype_int16,
    310 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    312 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    227 as i32 as yytype_int16,
    314 as i32 as yytype_int16,
    299 as i32 as yytype_int16,
    346 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    80 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    2 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    -(96 as i32) as yytype_int16,
    100 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    332 as i32 as yytype_int16,
    333 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    106 as i32 as yytype_int16,
    107 as i32 as yytype_int16,
    108 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(22 as i32) as yytype_int16,
    201 as i32 as yytype_int16,
    -(22 as i32) as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    334 as i32 as yytype_int16,
    335 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    205 as i32 as yytype_int16,
    -(78 as i32) as yytype_int16,
    -(78 as i32) as yytype_int16,
    206 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(78 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    -(78 as i32) as yytype_int16,
    77 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    205 as i32 as yytype_int16,
    -(77 as i32) as yytype_int16,
    -(77 as i32) as yytype_int16,
    206 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(77 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    -(77 as i32) as yytype_int16,
    77 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    205 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    206 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(109 as i32) as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    216 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(109 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    273 as i32 as yytype_int16,
    -(109 as i32) as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    -(109 as i32) as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    203 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    205 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    206 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    208 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    118 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    229 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    -(117 as i32) as yytype_int16,
    61 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    193 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    269 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    204 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    211 as i32 as yytype_int16,
    212 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    98 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    140 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    66 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    77 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    226 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    93 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
];
static mut yycheck: [yytype_int16; 1201] = [
    1 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    243 as i32 as yytype_int16,
    88 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    223 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    82 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    22 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    320 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    329 as i32 as yytype_int16,
    121 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    66 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    133 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    320 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    146 as i32 as yytype_int16,
    329 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    100 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    106 as i32 as yytype_int16,
    107 as i32 as yytype_int16,
    108 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    192 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    326 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    100 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    102 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    105 as i32 as yytype_int16,
    106 as i32 as yytype_int16,
    107 as i32 as yytype_int16,
    108 as i32 as yytype_int16,
    109 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    111 as i32 as yytype_int16,
    131 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    139 as i32 as yytype_int16,
    347 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    350 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    73 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    131 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    207 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    219 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    233 as i32 as yytype_int16,
    216 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    242 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    225 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    247 as i32 as yytype_int16,
    146 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    261 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    266 as i32 as yytype_int16,
    267 as i32 as yytype_int16,
    224 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    234 as i32 as yytype_int16,
    254 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    263 as i32 as yytype_int16,
    252 as i32 as yytype_int16,
    258 as i32 as yytype_int16,
    292 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    214 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    242 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    226 as i32 as yytype_int16,
    264 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    294 as i32 as yytype_int16,
    297 as i32 as yytype_int16,
    273 as i32 as yytype_int16,
    253 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    282 as i32 as yytype_int16,
    236 as i32 as yytype_int16,
    237 as i32 as yytype_int16,
    303 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    240 as i32 as yytype_int16,
    241 as i32 as yytype_int16,
    289 as i32 as yytype_int16,
    290 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    310 as i32 as yytype_int16,
    293 as i32 as yytype_int16,
    323 as i32 as yytype_int16,
    285 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    271 as i32 as yytype_int16,
    318 as i32 as yytype_int16,
    252 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    321 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    209 as i32 as yytype_int16,
    210 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    213 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    215 as i32 as yytype_int16,
    311 as i32 as yytype_int16,
    330 as i32 as yytype_int16,
    313 as i32 as yytype_int16,
    219 as i32 as yytype_int16,
    315 as i32 as yytype_int16,
    316 as i32 as yytype_int16,
    317 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    319 as i32 as yytype_int16,
    273 as i32 as yytype_int16,
    300 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    297 as i32 as yytype_int16,
    342 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    301 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    349 as i32 as yytype_int16,
    304 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    352 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    312 as i32 as yytype_int16,
    323 as i32 as yytype_int16,
    340 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    247 as i32 as yytype_int16,
    343 as i32 as yytype_int16,
    318 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    346 as i32 as yytype_int16,
    321 as i32 as yytype_int16,
    348 as i32 as yytype_int16,
    302 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    304 as i32 as yytype_int16,
    306 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    306 as i32 as yytype_int16,
    285 as i32 as yytype_int16,
    338 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    263 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    314 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    0 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    349 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    351 as i32 as yytype_int16,
    352 as i32 as yytype_int16,
    314 as i32 as yytype_int16,
    354 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    67 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    25 as i32 as yytype_int16,
    26 as i32 as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    11 as i32 as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    76 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    20 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    24 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    27 as i32 as yytype_int16,
    28 as i32 as yytype_int16,
    29 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    34 as i32 as yytype_int16,
    35 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    76 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    76 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    1 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    23 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    32 as i32 as yytype_int16,
    33 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    12 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    15 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    39 as i32 as yytype_int16,
    70 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    15 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    46 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    15 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    60 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    63 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    65 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    68 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    76 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    55 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    57 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    55 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    55 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
];
static mut yystos: [yytype_uint8; 356] = [
    0 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    3 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    19 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    3 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    10 as i32 as yytype_uint8,
    15 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    12 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    13 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    9 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    5 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    20 as i32 as yytype_uint8,
    22 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    30 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    32 as i32 as yytype_uint8,
    33 as i32 as yytype_uint8,
    34 as i32 as yytype_uint8,
    35 as i32 as yytype_uint8,
    40 as i32 as yytype_uint8,
    75 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    11 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    69 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 210] = [
    0 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
];
static mut yyr2: [yytype_int8; 210] = [
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    7 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    9 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    8 as i32 as yytype_int8,
    8 as i32 as yytype_int8,
    12 as i32 as yytype_int8,
    11 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    6 as i32 as yytype_int8,
    9 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const i8,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut *mut INSTRUCTION,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const i8;
    }
}
#[no_mangle]
pub static mut yychar: i32 = 0;
#[no_mangle]
pub static mut yylval: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
#[no_mangle]
pub static mut yynerrs: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> i32 {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as i32;
    let mut yyerrstatus: i32 = 0 as i32;
    let mut yystacksize: i64 = 200 as i32 as i64;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [*mut INSTRUCTION; 200] = [0 as *mut INSTRUCTION; 200];
    let mut yyvs: *mut *mut INSTRUCTION = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut *mut INSTRUCTION = yyvs;
    let mut yyn: i32 = 0;
    let mut yyresult: i32 = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut yylen: i32 = 0 as i32;
    yychar = -(2 as i32);
    's_46: loop {
        (0 as i32 != 0 && (0 as i32 <= yystate && yystate < 356 as i32)) as i32;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as i32 as isize)) <= yyssp {
            let mut yysize: i64 = yyssp.offset_from(yyss) as i64 + 1 as i32 as i64;
            if 10000 as i32 as i64 <= yystacksize {
                current_block = 10279487622471394456;
                break;
            }
            yystacksize *= 2 as i32 as i64;
            if (10000 as i32 as i64) < yystacksize {
                yystacksize = 10000 as i32 as i64;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = pma_malloc(
                (yystacksize
                    * (::core::mem::size_of::<yy_state_t>() as u64 as i64
                        + ::core::mem::size_of::<*mut INSTRUCTION>() as u64 as i64)
                    + (::core::mem::size_of::<yyalloc>() as u64 as i64
                        - 1 as i32 as i64)) as u64,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 10279487622471394456;
                break;
            }
            let mut yynewbytes: i64 = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as u64).wrapping_mul(::core::mem::size_of::<yy_state_t>() as u64)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize * ::core::mem::size_of::<yy_state_t>() as u64 as i64
                + (::core::mem::size_of::<yyalloc>() as u64 as i64 - 1 as i32 as i64);
            yyptr = yyptr
                .offset(
                    (yynewbytes / ::core::mem::size_of::<yyalloc>() as u64 as i64)
                        as isize,
                );
            let mut yynewbytes_0: i64 = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut *mut INSTRUCTION as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut INSTRUCTION>() as u64)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<*mut INSTRUCTION>() as u64 as i64
                + (::core::mem::size_of::<yyalloc>() as u64 as i64 - 1 as i32 as i64);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0 / ::core::mem::size_of::<yyalloc>() as u64 as i64)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                pma_free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as i32 as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as i32 as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as i32 as isize)) <= yyssp {
                current_block = 11477185723268747871;
                break;
            }
        }
        if yystate == 2 as i32 {
            yyresult = 0 as i32;
            current_block = 6465433104329454717;
            break;
        } else {
            yyn = yypact[yystate as usize] as i32;
            if yyn == -(276 as i32) {
                current_block = 15362765871823309265;
            } else {
                if yychar == -(2 as i32) {
                    yychar = yylex();
                }
                if yychar <= 0 as i32 {
                    yychar = 0 as i32;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 6174974146017752131;
                } else if yychar == 256 as i32 {
                    yychar = 257 as i32;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 14798202723045015423;
                } else {
                    yytoken = (if 0 as i32 <= yychar && yychar <= 310 as i32 {
                        yytranslate[yychar as usize] as yysymbol_kind_t as i32
                    } else {
                        YYSYMBOL_YYUNDEF as i32
                    }) as yysymbol_kind_t;
                    current_block = 6174974146017752131;
                }
                match current_block {
                    14798202723045015423 => {}
                    _ => {
                        yyn += yytoken as i32;
                        if yyn < 0 as i32 || (1200 as i32) < yyn
                            || yycheck[yyn as usize] as i32 != yytoken as i32
                        {
                            current_block = 15362765871823309265;
                        } else {
                            yyn = yytable[yyn as usize] as i32;
                            if yyn <= 0 as i32 {
                                if yyn == -(119 as i32) {
                                    current_block = 17872939710867982311;
                                } else {
                                    yyn = -yyn;
                                    current_block = 7363847765028622843;
                                }
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                    yyerrstatus;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = -(2 as i32);
                                current_block = 7394502214160662244;
                            }
                        }
                    }
                }
            }
            match current_block {
                15362765871823309265 => {
                    yyn = yydefact[yystate as usize] as i32;
                    if yyn == 0 as i32 {
                        current_block = 17872939710867982311;
                    } else {
                        current_block = 7363847765028622843;
                    }
                }
                _ => {}
            }
            match current_block {
                17872939710867982311 => {
                    yytoken = (if yychar == -(2 as i32) {
                        YYSYMBOL_YYEMPTY as i32
                    } else if 0 as i32 <= yychar && yychar <= 310 as i32 {
                        yytranslate[yychar as usize] as yysymbol_kind_t as i32
                    } else {
                        YYSYMBOL_YYUNDEF as i32
                    }) as yysymbol_kind_t;
                    if yyerrstatus == 0 {
                        yynerrs += 1;
                        yynerrs;
                        yyerror(b"syntax error\0" as *const u8 as *const i8);
                    }
                    if yyerrstatus == 3 as i32 {
                        if yychar <= 0 as i32 {
                            if yychar == 0 as i32 {
                                current_block = 11477185723268747871;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const i8,
                                yytoken,
                                &mut yylval,
                            );
                            yychar = -(2 as i32);
                        }
                    }
                    current_block = 14798202723045015423;
                }
                7363847765028622843 => {
                    yylen = yyr2[yyn as usize] as i32;
                    yyval = *yyvsp.offset((1 as i32 - yylen) as isize);
                    match yyn {
                        2 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        3 => {
                            rule = 0 as i32;
                            yyerrstatus = 0 as i32;
                        }
                        4 => {
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                    outer_comment = *yyvsp.offset(0 as i32 as isize);
                                } else {
                                    interblock_comment = *yyvsp.offset(0 as i32 as isize);
                                }
                            }
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        5 => {
                            next_sourcefile();
                        }
                        6 => {
                            rule = 0 as i32;
                        }
                        7 => {
                            append_rule(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            if !pending_comment.is_null() {
                                interblock_comment = pending_comment;
                                pending_comment = 0 as *mut INSTRUCTION;
                            }
                        }
                        8 => {
                            if rule != defrule::Rule as i32 {
                                msg(
                                    dcgettext(
                                        0 as *const i8,
                                        b"%s blocks must have an action part\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                    ruletab[rule as usize],
                                );
                                errcount += 1;
                                errcount;
                            } else if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                msg(
                                    dcgettext(
                                        0 as *const i8,
                                        b"each rule must have a pattern or an action part\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                errcount += 1;
                                errcount;
                            } else {
                                if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                    list_append(
                                        *yyvsp.offset(-(1 as i32) as isize),
                                        *yyvsp.offset(0 as i32 as isize),
                                    );
                                }
                                append_rule(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                            }
                        }
                        9 => {
                            in_function = 0 as i32 != 0;
                            mk_function(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            want_param_names = C2RustUnnamed_12::DONT_CHECK;
                            if !pending_comment.is_null() {
                                interblock_comment = pending_comment;
                                pending_comment = 0 as *mut INSTRUCTION;
                            }
                            yyerrstatus = 0 as i32;
                        }
                        10 => {
                            want_source = 0 as i32 != 0;
                            at_seen -= 1;
                            at_seen;
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null()
                                && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                let mut s: *mut SRCFILE = *yyvsp
                                    .offset(-(1 as i32) as isize) as *mut SRCFILE;
                                (*s).comment = *yyvsp.offset(0 as i32 as isize);
                            }
                            yyerrstatus = 0 as i32;
                        }
                        11 => {
                            want_source = 0 as i32 != 0;
                            at_seen -= 1;
                            at_seen;
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null()
                                && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                let mut s_0: *mut SRCFILE = *yyvsp
                                    .offset(-(1 as i32) as isize) as *mut SRCFILE;
                                (*s_0).comment = *yyvsp.offset(0 as i32 as isize);
                            }
                            yyerrstatus = 0 as i32;
                        }
                        12 => {
                            want_source = 0 as i32 != 0;
                            want_namespace = 0 as i32 != 0;
                            at_seen -= 1;
                            at_seen;
                            set_namespace(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            yyerrstatus = 0 as i32;
                        }
                        13 => {
                            let mut srcfile: *mut libc::c_void = 0 as *mut libc::c_void;
                            if !include_source(
                                *yyvsp.offset(0 as i32 as isize),
                                &mut srcfile,
                            ) {
                                current_block = 11477185723268747871;
                                break;
                            }
                            pma_free(
                                (**yyvsp.offset(0 as i32 as isize)).d.name
                                    as *mut libc::c_void,
                            );
                            bcfree(*yyvsp.offset(0 as i32 as isize));
                            yyval = srcfile as *mut INSTRUCTION;
                        }
                        14 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        15 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        16 => {
                            let mut srcfile_0: *mut libc::c_void = 0
                                as *mut libc::c_void;
                            if !load_library(
                                *yyvsp.offset(0 as i32 as isize),
                                &mut srcfile_0,
                            ) {
                                current_block = 11477185723268747871;
                                break;
                            }
                            pma_free(
                                (**yyvsp.offset(0 as i32 as isize)).d.name
                                    as *mut libc::c_void,
                            );
                            bcfree(*yyvsp.offset(0 as i32 as isize));
                            yyval = srcfile_0 as *mut INSTRUCTION;
                        }
                        17 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        18 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        19 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        20 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        21 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        22 => {
                            rule = defrule::Rule as i32;
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        23 => {
                            rule = defrule::Rule as i32;
                        }
                        24 => {
                            let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            add_lint(
                                *yyvsp.offset(-(2 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            add_lint(
                                *yyvsp.offset(0 as i32 as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            tp = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
                            list_prepend(
                                *yyvsp.offset(-(2 as i32) as isize),
                                bcalloc(
                                    opcodeval::Op_line_range,
                                    (do_flags as u32
                                        & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0) as i32
                                        + 1 as i32,
                                    0 as i32,
                                ),
                            );
                            (*(**yyvsp.offset(-(2 as i32) as isize)).nexti).x.xl = 0
                                as i32 as i64;
                            let ref mut fresh0 = (*(**yyvsp.offset(-(2 as i32) as isize))
                                .nexti)
                                .d
                                .di;
                            *fresh0 = (**yyvsp.offset(0 as i32 as isize)).nexti;
                            list_append(
                                *yyvsp.offset(-(2 as i32) as isize),
                                bcalloc(opcodeval::Op_cond_pair, 1 as i32, 0 as i32),
                            );
                            let ref mut fresh1 = (*(**yyvsp.offset(-(2 as i32) as isize))
                                .d
                                .di)
                                .x
                                .xi;
                            *fresh1 = (**yyvsp.offset(-(2 as i32) as isize)).nexti;
                            let ref mut fresh2 = (*(**yyvsp.offset(-(2 as i32) as isize))
                                .d
                                .di)
                                .d
                                .di;
                            *fresh2 = tp;
                            list_append(
                                *yyvsp.offset(0 as i32 as isize),
                                bcalloc(opcodeval::Op_cond_pair, 1 as i32, 0 as i32),
                            );
                            let ref mut fresh3 = (*(**yyvsp.offset(0 as i32 as isize))
                                .d
                                .di)
                                .x
                                .xi;
                            *fresh3 = (**yyvsp.offset(-(2 as i32) as isize)).nexti;
                            let ref mut fresh4 = (*(**yyvsp.offset(0 as i32 as isize))
                                .d
                                .di)
                                .d
                                .di;
                            *fresh4 = tp;
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                let ref mut fresh5 = (*((**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .nexti)
                                    .offset(1 as i32 as isize))
                                    .d
                                    .di;
                                *fresh5 = (**yyvsp.offset(-(2 as i32) as isize)).d.di;
                                let ref mut fresh6 = (*((**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .nexti)
                                    .offset(1 as i32 as isize))
                                    .x
                                    .xi;
                                *fresh6 = (**yyvsp.offset(0 as i32 as isize)).d.di;
                            }
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                yyval = list_append(
                                    list_merge(
                                        list_prepend(
                                            *yyvsp.offset(-(2 as i32) as isize),
                                            *yyvsp.offset(-(1 as i32) as isize),
                                        ),
                                        *yyvsp.offset(0 as i32 as isize),
                                    ),
                                    tp,
                                );
                            } else {
                                yyval = list_append(
                                    list_merge(
                                        *yyvsp.offset(-(2 as i32) as isize),
                                        *yyvsp.offset(0 as i32 as isize),
                                    ),
                                    tp,
                                );
                            }
                            rule = defrule::Rule as i32;
                        }
                        25 => {
                            static mut begin_seen: i32 = 0 as i32;
                            if do_flags as u32
                                & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                && {
                                    begin_seen += 1;
                                    begin_seen == 2 as i32
                                }
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"old awk does not support multiple `defrule::BEGIN' or `defrule::END' rules\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            rule = defrule::BEGIN as i32;
                            (**yyvsp.offset(0 as i32 as isize)).x.xl = rule as i64;
                            let ref mut fresh7 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh7 = source;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        26 => {
                            static mut end_seen: i32 = 0 as i32;
                            if do_flags as u32
                                & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                && {
                                    end_seen += 1;
                                    end_seen == 2 as i32
                                }
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"old awk does not support multiple `defrule::BEGIN' or `defrule::END' rules\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            rule = defrule::END as i32;
                            (**yyvsp.offset(0 as i32 as isize)).x.xl = rule as i64;
                            let ref mut fresh8 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh8 = source;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        27 => {
                            rule = defrule::BEGINFILE as i32;
                            (**yyvsp.offset(0 as i32 as isize)).x.xl = rule as i64;
                            let ref mut fresh9 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh9 = source;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        28 => {
                            rule = defrule::ENDFILE as i32;
                            (**yyvsp.offset(0 as i32 as isize)).x.xl = rule as i64;
                            let ref mut fresh10 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh10 = source;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        29 => {
                            let mut ip: *mut INSTRUCTION = make_braced_statements(
                                *yyvsp.offset(-(4 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                            );
                            if !(*yyvsp.offset(-(2 as i32) as isize)).is_null()
                                && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                merge_comments(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                                pending_comment = *yyvsp.offset(-(2 as i32) as isize);
                            } else if !(*yyvsp.offset(-(2 as i32) as isize)).is_null() {
                                pending_comment = *yyvsp.offset(-(2 as i32) as isize);
                            } else if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                pending_comment = *yyvsp.offset(0 as i32 as isize);
                            }
                            yyval = ip;
                        }
                        31 => {
                            let mut name: *const i8 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            let mut qname: *mut i8 = qualify_name(name, strlen(name));
                            if qname != name as *mut i8 {
                                pma_free(name as *mut libc::c_void);
                                let ref mut fresh11 = (**yyvsp.offset(0 as i32 as isize))
                                    .d
                                    .name;
                                *fresh11 = qname;
                            }
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        32 => {
                            yyerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"`%s' is a built-in function, it cannot be redefined\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                tokstart,
                            );
                            current_block = 11477185723268747871;
                            break;
                        }
                        33 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                            at_seen -= 1;
                            at_seen;
                        }
                        36 => {
                            want_param_names = C2RustUnnamed_12::FUNC_HEADER;
                        }
                        37 => {
                            let mut func_comment: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            if !(*yyvsp.offset(-(2 as i32) as isize)).is_null()
                                && !((**yyvsp.offset(-(2 as i32) as isize)).comment)
                                    .is_null()
                            {
                                if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                    merge_comments(
                                        (**yyvsp.offset(-(2 as i32) as isize)).comment,
                                        *yyvsp.offset(0 as i32 as isize),
                                    );
                                }
                                func_comment = (**yyvsp.offset(-(2 as i32) as isize))
                                    .comment;
                            } else if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                func_comment = *yyvsp.offset(0 as i32 as isize);
                            }
                            let ref mut fresh12 = (**yyvsp.offset(-(6 as i32) as isize))
                                .d
                                .name;
                            *fresh12 = source;
                            let ref mut fresh13 = (**yyvsp.offset(-(6 as i32) as isize))
                                .comment;
                            *fresh13 = func_comment;
                            if install_function(
                                (**yyvsp.offset(-(5 as i32) as isize)).d.name,
                                *yyvsp.offset(-(6 as i32) as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                            ) < 0 as i32
                            {
                                current_block = 11477185723268747871;
                                break;
                            }
                            in_function = 1 as i32 != 0;
                            let ref mut fresh14 = (**yyvsp.offset(-(5 as i32) as isize))
                                .d
                                .name;
                            *fresh14 = 0 as *mut i8;
                            bcfree(*yyvsp.offset(-(5 as i32) as isize));
                            yyval = *yyvsp.offset(-(6 as i32) as isize);
                            want_param_names = C2RustUnnamed_12::FUNC_BODY;
                        }
                        38 => {
                            want_regexp = 1 as i32 != 0;
                        }
                        39 => {
                            let mut n: *mut NODE = 0 as *mut NODE;
                            let mut exp: *mut NODE = 0 as *mut NODE;
                            let mut re: *mut i8 = 0 as *mut i8;
                            let mut len: size_t = 0;
                            re = (**yyvsp.offset(0 as i32 as isize)).d.name;
                            let ref mut fresh15 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh15 = 0 as *mut i8;
                            len = strlen(re);
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                if len == 0 as i32 as u64 {
                                    lintwarn_ln(
                                        (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                        dcgettext(
                                            0 as *const i8,
                                            b"regexp constant `//' looks like a C++ comment, but is not\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else if *re.offset(0 as i32 as isize) as i32 == '*' as i32
                                    && *re.offset(len.wrapping_sub(1 as i32 as u64) as isize)
                                        as i32 == '*' as i32
                                {
                                    lintwarn_ln(
                                        (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                        dcgettext(
                                            0 as *const i8,
                                            b"regexp constant `/%s/' looks like a C comment, but is not\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                        re,
                                    );
                                }
                            }
                            exp = make_str_node(re, len, 2 as i32);
                            n = make_regnode(nodevals::Node_regex, exp);
                            if n.is_null() {
                                unref(exp);
                                current_block = 11477185723268747871;
                                break;
                            } else {
                                yyval = *yyvsp.offset(0 as i32 as isize);
                                (*yyval).opcode = opcodeval::Op_match_rec;
                                (*yyval).d.dn = n;
                            }
                        }
                        40 => {
                            let mut re_0: *mut i8 = 0 as *mut i8;
                            let mut len_0: size_t = 0;
                            re_0 = (**yyvsp.offset(0 as i32 as isize)).d.name;
                            let ref mut fresh16 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .name;
                            *fresh16 = 0 as *mut i8;
                            len_0 = strlen(re_0);
                            yyval = *yyvsp.offset(0 as i32 as isize);
                            (*yyval).opcode = opcodeval::Op_push_re;
                            (*yyval).d.dn = make_typed_regex(re_0, len_0);
                        }
                        41 => {
                            bcfree(*yyvsp.offset(0 as i32 as isize));
                        }
                        43 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        44 => {
                            if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = *yyvsp.offset(-(1 as i32) as isize);
                            } else {
                                add_lint(
                                    *yyvsp.offset(0 as i32 as isize),
                                    lintvals::LINT_no_effect,
                                );
                                if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                    yyval = *yyvsp.offset(0 as i32 as isize);
                                } else {
                                    yyval = list_merge(
                                        *yyvsp.offset(-(1 as i32) as isize),
                                        *yyvsp.offset(0 as i32 as isize),
                                    );
                                }
                            }
                            if !trailing_comment.is_null() {
                                yyval = list_append(yyval, trailing_comment);
                                trailing_comment = 0 as *mut INSTRUCTION;
                            }
                            yyerrstatus = 0 as i32;
                        }
                        45 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        46 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        47 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        48 => {
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                let mut ip_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                merge_comments(
                                    *yyvsp.offset(0 as i32 as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                ip_0 = list_create(
                                    bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                                );
                                yyval = list_append(ip_0, *yyvsp.offset(0 as i32 as isize));
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        49 => {
                            trailing_comment = *yyvsp.offset(0 as i32 as isize);
                            yyval = make_braced_statements(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        50 => {
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                            } else {
                                yyval = *yyvsp.offset(0 as i32 as isize);
                            }
                        }
                        51 => {
                            let mut dflt: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut curr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut cexp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut cstmt: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut ip_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut nextc_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut case_values: *mut *const i8 = 0 as *mut *const i8;
                            let mut maxcount: i32 = 128 as i32;
                            let mut case_count: i32 = 0 as i32;
                            let mut i: i32 = 0;
                            tbreak = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
                            cstmt = list_create(tbreak);
                            cexp = list_create(
                                bcalloc(opcodeval::Op_pop, 1 as i32, 0 as i32),
                            );
                            dflt = bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32);
                            (*dflt).d.di = tbreak;
                            if !(*yyvsp.offset(-(2 as i32) as isize)).is_null() {
                                curr = (**yyvsp.offset(-(2 as i32) as isize)).nexti;
                                bcfree(*yyvsp.offset(-(2 as i32) as isize));
                            }
                            while !curr.is_null() {
                                let mut caseexp: *mut INSTRUCTION = (*curr).d.di;
                                let mut casestmt: *mut INSTRUCTION = (*curr).x.xi;
                                nextc_0 = (*curr).nexti;
                                if (*curr).opcode as u32
                                    == opcodeval::Op_K_case as i32 as u32
                                {
                                    if (*caseexp).opcode as u32
                                        == opcodeval::Op_push_i as i32 as u32
                                    {
                                        let mut caseval: *mut i8 = 0 as *mut i8;
                                        caseval = (*force_string_fmt(
                                            (*caseexp).d.dn,
                                            CONVFMT,
                                            CONVFMTidx,
                                        ))
                                            .sub
                                            .val
                                            .sp;
                                        i = 0 as i32;
                                        while i < case_count {
                                            if strcmp(caseval, *case_values.offset(i as isize))
                                                == 0 as i32
                                            {
                                                error_ln(
                                                    (*curr).source_line as i32,
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"duplicate case values in switch body: %s\0" as *const u8
                                                            as *const i8,
                                                        5 as i32,
                                                    ),
                                                    caseval,
                                                );
                                            }
                                            i += 1;
                                            i;
                                        }
                                        if case_values.is_null() {
                                            case_values = emalloc_real(
                                                (::core::mem::size_of::<*mut i8>() as u64)
                                                    .wrapping_mul(maxcount as u64),
                                                b"statement\0" as *const u8 as *const i8,
                                                b"case_values\0" as *const u8 as *const i8,
                                                b"awkgram.y\0" as *const u8 as *const i8,
                                                699 as i32,
                                            ) as *mut *const i8;
                                        } else if case_count >= maxcount {
                                            maxcount += 128 as i32;
                                            case_values = erealloc_real(
                                                case_values as *mut libc::c_void,
                                                (::core::mem::size_of::<*mut i8>() as u64)
                                                    .wrapping_mul(maxcount as u64),
                                                b"statement\0" as *const u8 as *const i8,
                                                b"case_values\0" as *const u8 as *const i8,
                                                b"awkgram.y\0" as *const u8 as *const i8,
                                                702 as i32,
                                            ) as *mut *const i8;
                                        }
                                        let fresh17 = case_count;
                                        case_count = case_count + 1;
                                        let ref mut fresh18 = *case_values.offset(fresh17 as isize);
                                        *fresh18 = caseval;
                                    } else {
                                        (*curr.offset(1 as i32 as isize)).x.xl = 1 as i32 as i64;
                                    }
                                    (*curr).d.di = (*casestmt).nexti;
                                    (*curr).x.xi = (*casestmt).d.di;
                                    list_prepend(cexp, curr);
                                    list_prepend(cexp, caseexp);
                                } else {
                                    if (*dflt).d.di != tbreak {
                                        error_ln(
                                            (*curr).source_line as i32,
                                            dcgettext(
                                                0 as *const i8,
                                                b"duplicate `default' detected in switch body\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        (*dflt).d.di = (*casestmt).nexti;
                                    }
                                    if do_flags as u32
                                        & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                                    {
                                        (*curr).d.di = (*casestmt).nexti;
                                        (*curr).x.xi = (*casestmt).d.di;
                                        list_prepend(cexp, curr);
                                    } else {
                                        bcfree(curr);
                                    }
                                }
                                cstmt = list_merge(casestmt, cstmt);
                                curr = nextc_0;
                            }
                            if !case_values.is_null() {
                                pma_free(case_values as *mut libc::c_void);
                            }
                            ip_1 = *yyvsp.offset(-(6 as i32) as isize);
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                let mut head_comment: *mut INSTRUCTION = 0
                                    as *mut INSTRUCTION;
                                if !(*yyvsp.offset(-(4 as i32) as isize)).is_null()
                                    && !(*yyvsp.offset(-(3 as i32) as isize)).is_null()
                                {
                                    merge_comments(
                                        *yyvsp.offset(-(4 as i32) as isize),
                                        *yyvsp.offset(-(3 as i32) as isize),
                                    );
                                    head_comment = *yyvsp.offset(-(4 as i32) as isize);
                                } else if !(*yyvsp.offset(-(4 as i32) as isize)).is_null() {
                                    head_comment = *yyvsp.offset(-(4 as i32) as isize);
                                } else {
                                    head_comment = *yyvsp.offset(-(3 as i32) as isize);
                                }
                                let ref mut fresh19 = (**yyvsp.offset(-(8 as i32) as isize))
                                    .comment;
                                *fresh19 = head_comment;
                                list_prepend(ip_1, *yyvsp.offset(-(8 as i32) as isize));
                                list_prepend(
                                    ip_1,
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                                let ref mut fresh20 = (**yyvsp.offset(-(8 as i32) as isize))
                                    .x
                                    .xi;
                                *fresh20 = tbreak;
                                let ref mut fresh21 = (*(*yyvsp
                                    .offset(-(8 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .d
                                    .di;
                                *fresh21 = (*cexp).nexti;
                                let ref mut fresh22 = (*(*yyvsp
                                    .offset(-(8 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .x
                                    .xi;
                                *fresh22 = (*cexp).d.di;
                                let ref mut fresh23 = (*(*(*yyvsp
                                    .offset(-(8 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .x
                                    .xi)
                                    .comment;
                                *fresh23 = *yyvsp.offset(0 as i32 as isize);
                            }
                            list_append(cexp, dflt);
                            list_merge(ip_1, cexp);
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                list_append(cstmt, *yyvsp.offset(-(1 as i32) as isize));
                            }
                            yyval = list_merge(ip_1, cstmt);
                            break_allowed -= 1;
                            break_allowed;
                            fix_break_continue(ip_1, tbreak, 0 as *mut INSTRUCTION);
                        }
                        52 => {
                            let mut ip_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            tbreak_0 = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
                            add_lint(
                                *yyvsp.offset(-(3 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            tcont = (**yyvsp.offset(-(3 as i32) as isize)).nexti;
                            ip_2 = list_append(
                                *yyvsp.offset(-(3 as i32) as isize),
                                bcalloc(opcodeval::Op_jmp_false, 1 as i32, 0 as i32),
                            );
                            (*(*ip_2).d.di).d.di = tbreak_0;
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                list_append(
                                    ip_2,
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                                let ref mut fresh24 = (**yyvsp.offset(-(5 as i32) as isize))
                                    .x
                                    .xi;
                                *fresh24 = tbreak_0;
                                let ref mut fresh25 = (**yyvsp.offset(-(5 as i32) as isize))
                                    .d
                                    .di;
                                *fresh25 = tcont;
                                let ref mut fresh26 = (*(*yyvsp
                                    .offset(-(5 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .d
                                    .di;
                                *fresh26 = (*ip_2).d.di;
                                list_prepend(ip_2, *yyvsp.offset(-(5 as i32) as isize));
                            }
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                    let ref mut fresh27 = *yyvsp.offset(0 as i32 as isize);
                                    *fresh27 = list_create(
                                        bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                                    );
                                }
                                (*(**yyvsp.offset(-(1 as i32) as isize)).d.dn)
                                    .sub
                                    .val
                                    .comtype = commenttype::BLOCK_COMMENT;
                                let ref mut fresh28 = *yyvsp.offset(0 as i32 as isize);
                                *fresh28 = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                list_merge(ip_2, *yyvsp.offset(0 as i32 as isize));
                            }
                            list_append(
                                ip_2,
                                bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32),
                            );
                            (*(*ip_2).d.di).d.di = tcont;
                            yyval = list_append(ip_2, tbreak_0);
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                            fix_break_continue(ip_2, tbreak_0, tcont);
                        }
                        53 => {
                            let mut ip_3: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tbreak_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            tbreak_1 = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
                            tcont_0 = (**yyvsp.offset(-(2 as i32) as isize)).nexti;
                            add_lint(
                                *yyvsp.offset(-(2 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            if !(*yyvsp.offset(-(5 as i32) as isize)).is_null() {
                                ip_3 = list_merge(
                                    *yyvsp.offset(-(5 as i32) as isize),
                                    *yyvsp.offset(-(2 as i32) as isize),
                                );
                            } else {
                                ip_3 = list_prepend(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                                );
                            }
                            if !(*yyvsp.offset(-(6 as i32) as isize)).is_null() {
                                list_prepend(ip_3, *yyvsp.offset(-(6 as i32) as isize));
                            }
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                list_prepend(
                                    ip_3,
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                            }
                            list_append(
                                ip_3,
                                bcalloc(opcodeval::Op_jmp_true, 1 as i32, 0 as i32),
                            );
                            (*(*ip_3).d.di).d.di = (*ip_3).nexti;
                            yyval = list_append(ip_3, tbreak_1);
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                            fix_break_continue(ip_3, tbreak_1, tcont_0);
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                let ref mut fresh29 = (**yyvsp.offset(-(7 as i32) as isize))
                                    .x
                                    .xi;
                                *fresh29 = tbreak_1;
                                let ref mut fresh30 = (**yyvsp.offset(-(7 as i32) as isize))
                                    .d
                                    .di;
                                *fresh30 = tcont_0;
                                let ref mut fresh31 = (*(*yyvsp
                                    .offset(-(7 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .d
                                    .di;
                                *fresh31 = tcont_0;
                                yyval = list_prepend(
                                    ip_3,
                                    *yyvsp.offset(-(7 as i32) as isize),
                                );
                                bcfree(*yyvsp.offset(-(4 as i32) as isize));
                                if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                    let ref mut fresh32 = (**yyvsp.offset(-(7 as i32) as isize))
                                        .comment;
                                    *fresh32 = *yyvsp.offset(0 as i32 as isize);
                                }
                            }
                        }
                        54 => {
                            let mut ip_4: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut var_name: *mut i8 = (**yyvsp
                                .offset(-(5 as i32) as isize))
                                .d
                                .name;
                            let mut tbreak_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut tcont_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut current_block_474: u64;
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null()
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                    == opcodeval::Op_K_delete as i32 as u32
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).x.xl
                                    == 1 as i32 as i64
                                && (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_push as i32 as u32
                                && ((*(*(**yyvsp.offset(0 as i32 as isize)).nexti).d.dn)
                                    .type_0 as u32 != nodevals::Node_var as i32 as u32
                                    || ((*(*(**yyvsp.offset(0 as i32 as isize)).nexti).d.dn)
                                        .sub
                                        .nodep
                                        .r
                                        .uptr)
                                        .is_none())
                                && strcmp(
                                    (*(*(**yyvsp.offset(0 as i32 as isize)).nexti).d.dn)
                                        .sub
                                        .nodep
                                        .name,
                                    var_name,
                                ) == 0 as i32
                            {
                                let mut arr: *mut NODE = 0 as *mut NODE;
                                ip_4 = (*(**yyvsp.offset(0 as i32 as isize)).nexti).nexti;
                                if (*(**yyvsp.offset(-(3 as i32) as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_push as i32 as u32
                                    && (**yyvsp.offset(-(3 as i32) as isize)).d.di
                                        == (**yyvsp.offset(-(3 as i32) as isize)).nexti
                                {
                                    arr = (*(**yyvsp.offset(-(3 as i32) as isize)).nexti).d.dn;
                                }
                                if !arr.is_null()
                                    && (*ip_4).opcode as u32
                                        == opcodeval::Op_no_op as i32 as u32
                                    && (*(*ip_4).nexti).opcode as u32
                                        == opcodeval::Op_push_array as i32 as u32
                                    && strcmp(
                                        (*(*(*ip_4).nexti).d.dn).sub.nodep.name,
                                        (*arr).sub.nodep.name,
                                    ) == 0 as i32
                                    && (*(*ip_4).nexti).nexti
                                        == (**yyvsp.offset(0 as i32 as isize)).d.di
                                {
                                    make_assignable((**yyvsp.offset(0 as i32 as isize)).nexti);
                                    (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode = opcodeval::Op_K_delete_loop;
                                    (*(**yyvsp.offset(0 as i32 as isize)).d.di).x.xl = 0 as i32
                                        as i64;
                                    if !(*yyvsp.offset(-(7 as i32) as isize)).is_null() {
                                        bcfree(*yyvsp.offset(-(7 as i32) as isize));
                                    }
                                    pma_free(var_name as *mut libc::c_void);
                                    bcfree(*yyvsp.offset(-(5 as i32) as isize));
                                    bcfree(*yyvsp.offset(-(4 as i32) as isize));
                                    bcfree(*yyvsp.offset(-(3 as i32) as isize));
                                    if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                        merge_comments(
                                            *yyvsp.offset(-(1 as i32) as isize),
                                            0 as *mut INSTRUCTION,
                                        );
                                        let ref mut fresh33 = *yyvsp.offset(0 as i32 as isize);
                                        *fresh33 = list_prepend(
                                            *yyvsp.offset(0 as i32 as isize),
                                            *yyvsp.offset(-(1 as i32) as isize),
                                        );
                                    }
                                    yyval = *yyvsp.offset(0 as i32 as isize);
                                    current_block_474 = 9622788801273990334;
                                } else {
                                    current_block_474 = 12266330736856645888;
                                }
                            } else {
                                tbreak_2 = 0 as *mut INSTRUCTION;
                                tcont_1 = 0 as *mut INSTRUCTION;
                                current_block_474 = 12266330736856645888;
                            }
                            match current_block_474 {
                                12266330736856645888 => {
                                    ip_4 = *yyvsp.offset(-(3 as i32) as isize);
                                    (*(*ip_4).nexti).opcode = opcodeval::Op_push_array;
                                    tbreak_2 = bcalloc(
                                        opcodeval::Op_arrayfor_final,
                                        1 as i32,
                                        0 as i32,
                                    );
                                    (**yyvsp.offset(-(4 as i32) as isize)).opcode = opcodeval::Op_arrayfor_incr;
                                    let ref mut fresh34 = (**yyvsp.offset(-(4 as i32) as isize))
                                        .x
                                        .xn;
                                    *fresh34 = variable(
                                        (**yyvsp.offset(-(5 as i32) as isize)).source_line as i32,
                                        var_name,
                                        nodevals::Node_var,
                                    );
                                    let ref mut fresh35 = (**yyvsp.offset(-(4 as i32) as isize))
                                        .d
                                        .di;
                                    *fresh35 = tbreak_2;
                                    tcont_1 = *yyvsp.offset(-(4 as i32) as isize);
                                    (**yyvsp.offset(-(5 as i32) as isize)).opcode = opcodeval::Op_arrayfor_init;
                                    let ref mut fresh36 = (**yyvsp.offset(-(5 as i32) as isize))
                                        .d
                                        .di;
                                    *fresh36 = tbreak_2;
                                    list_append(ip_4, *yyvsp.offset(-(5 as i32) as isize));
                                    if do_flags as u32
                                        & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                                    {
                                        (**yyvsp.offset(-(7 as i32) as isize)).opcode = opcodeval::Op_K_arrayfor;
                                        let ref mut fresh37 = (**yyvsp.offset(-(7 as i32) as isize))
                                            .d
                                            .di;
                                        *fresh37 = tcont_1;
                                        let ref mut fresh38 = (**yyvsp.offset(-(7 as i32) as isize))
                                            .x
                                            .xi;
                                        *fresh38 = tbreak_2;
                                        list_append(ip_4, *yyvsp.offset(-(7 as i32) as isize));
                                    }
                                    if (*(**yyvsp.offset(-(4 as i32) as isize)).x.xn).type_0
                                        as u32 == nodevals::Node_var as i32 as u32
                                        && ((*(**yyvsp.offset(-(4 as i32) as isize)).x.xn)
                                            .sub
                                            .nodep
                                            .r
                                            .uptr)
                                            .is_some()
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(opcodeval::Op_var_update, 1 as i32, 0 as i32),
                                        );
                                        (*(*ip_4).d.di).x.aptr = (*(**yyvsp
                                            .offset(-(4 as i32) as isize))
                                            .x
                                            .xn)
                                            .sub
                                            .nodep
                                            .r
                                            .uptr;
                                    }
                                    list_append(ip_4, *yyvsp.offset(-(4 as i32) as isize));
                                    if (*(**yyvsp.offset(-(4 as i32) as isize)).x.xn).type_0
                                        as u32 == nodevals::Node_var as i32 as u32
                                        && ((*(**yyvsp.offset(-(4 as i32) as isize)).x.xn)
                                            .sub
                                            .nodep
                                            .x
                                            .aptr)
                                            .is_some()
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(opcodeval::Op_var_assign, 1 as i32, 0 as i32),
                                        );
                                        (*(*ip_4).d.di).x.aptr = (*(**yyvsp
                                            .offset(-(4 as i32) as isize))
                                            .x
                                            .xn)
                                            .sub
                                            .nodep
                                            .x
                                            .aptr;
                                    }
                                    if do_flags as u32
                                        & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                                    {
                                        list_append(
                                            ip_4,
                                            bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                        );
                                        let ref mut fresh39 = (*(*yyvsp
                                            .offset(-(7 as i32) as isize))
                                            .offset(1 as i32 as isize))
                                            .d
                                            .di;
                                        *fresh39 = *yyvsp.offset(-(4 as i32) as isize);
                                        let ref mut fresh40 = (*(*yyvsp
                                            .offset(-(7 as i32) as isize))
                                            .offset(1 as i32 as isize))
                                            .x
                                            .xi;
                                        *fresh40 = (*ip_4).d.di;
                                    }
                                    if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                        merge_comments(
                                            *yyvsp.offset(-(1 as i32) as isize),
                                            0 as *mut INSTRUCTION,
                                        );
                                    }
                                    if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                        if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                            let ref mut fresh41 = *yyvsp.offset(0 as i32 as isize);
                                            *fresh41 = list_prepend(
                                                *yyvsp.offset(0 as i32 as isize),
                                                *yyvsp.offset(-(1 as i32) as isize),
                                            );
                                        }
                                        list_merge(ip_4, *yyvsp.offset(0 as i32 as isize));
                                    } else if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                        list_append(ip_4, *yyvsp.offset(-(1 as i32) as isize));
                                    }
                                    list_append(
                                        ip_4,
                                        bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32),
                                    );
                                    (*(*ip_4).d.di).d.di = *yyvsp.offset(-(4 as i32) as isize);
                                    yyval = list_append(ip_4, tbreak_2);
                                    fix_break_continue(ip_4, tbreak_2, tcont_1);
                                }
                                _ => {}
                            }
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        55 => {
                            if !(*yyvsp.offset(-(7 as i32) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(7 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                let ref mut fresh42 = (**yyvsp
                                    .offset(-(11 as i32) as isize))
                                    .comment;
                                *fresh42 = *yyvsp.offset(-(7 as i32) as isize);
                            }
                            if !(*yyvsp.offset(-(4 as i32) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(4 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                if ((**yyvsp.offset(-(11 as i32) as isize)).comment)
                                    .is_null()
                                {
                                    (*(**yyvsp.offset(-(4 as i32) as isize)).d.dn)
                                        .sub
                                        .val
                                        .comtype = commenttype::FOR_COMMENT;
                                    let ref mut fresh43 = (**yyvsp
                                        .offset(-(11 as i32) as isize))
                                        .comment;
                                    *fresh43 = *yyvsp.offset(-(4 as i32) as isize);
                                } else {
                                    let ref mut fresh44 = (*(**yyvsp
                                        .offset(-(11 as i32) as isize))
                                        .comment)
                                        .comment;
                                    *fresh44 = *yyvsp.offset(-(4 as i32) as isize);
                                }
                            }
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh45 = *yyvsp.offset(0 as i32 as isize);
                                *fresh45 = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                            add_lint(
                                *yyvsp.offset(-(6 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            yyval = mk_for_loop(
                                *yyvsp.offset(-(11 as i32) as isize),
                                *yyvsp.offset(-(9 as i32) as isize),
                                *yyvsp.offset(-(6 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        56 => {
                            if !(*yyvsp.offset(-(6 as i32) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(6 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                let ref mut fresh46 = (**yyvsp
                                    .offset(-(10 as i32) as isize))
                                    .comment;
                                *fresh46 = *yyvsp.offset(-(6 as i32) as isize);
                            }
                            if !(*yyvsp.offset(-(4 as i32) as isize)).is_null() {
                                merge_comments(
                                    *yyvsp.offset(-(4 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                );
                                if ((**yyvsp.offset(-(10 as i32) as isize)).comment)
                                    .is_null()
                                {
                                    (*(**yyvsp.offset(-(4 as i32) as isize)).d.dn)
                                        .sub
                                        .val
                                        .comtype = commenttype::FOR_COMMENT;
                                    let ref mut fresh47 = (**yyvsp
                                        .offset(-(10 as i32) as isize))
                                        .comment;
                                    *fresh47 = *yyvsp.offset(-(4 as i32) as isize);
                                } else {
                                    let ref mut fresh48 = (*(**yyvsp
                                        .offset(-(10 as i32) as isize))
                                        .comment)
                                        .comment;
                                    *fresh48 = *yyvsp.offset(-(4 as i32) as isize);
                                }
                            }
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh49 = *yyvsp.offset(0 as i32 as isize);
                                *fresh49 = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                            yyval = mk_for_loop(
                                *yyvsp.offset(-(10 as i32) as isize),
                                *yyvsp.offset(-(8 as i32) as isize),
                                0 as *mut libc::c_void as *mut INSTRUCTION,
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            break_allowed -= 1;
                            break_allowed;
                            continue_allowed -= 1;
                            continue_allowed;
                        }
                        57 => {
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                            } else {
                                yyval = *yyvsp.offset(0 as i32 as isize);
                            }
                        }
                        58 => {
                            if break_allowed == 0 {
                                error_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`break' is not allowed outside a loop or switch\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let ref mut fresh50 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .di;
                            *fresh50 = 0 as *mut exp_instruction;
                            yyval = list_create(*yyvsp.offset(-(1 as i32) as isize));
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        59 => {
                            if continue_allowed == 0 {
                                error_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`continue' is not allowed outside a loop\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let ref mut fresh51 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .di;
                            *fresh51 = 0 as *mut exp_instruction;
                            yyval = list_create(*yyvsp.offset(-(1 as i32) as isize));
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        60 => {
                            if rule != 0 && rule != defrule::Rule as i32 {
                                error_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`next' used in %s action\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            let ref mut fresh52 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .di;
                            *fresh52 = ip_rec;
                            yyval = list_create(*yyvsp.offset(-(1 as i32) as isize));
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        61 => {
                            if rule == defrule::BEGIN as i32
                                || rule == defrule::END as i32
                                || rule == defrule::ENDFILE as i32
                            {
                                error_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`nextfile' used in %s action\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            let ref mut fresh53 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .di;
                            *fresh53 = ip_newfile;
                            let ref mut fresh54 = (**yyvsp.offset(-(1 as i32) as isize))
                                .x
                                .xi;
                            *fresh54 = ip_endfile;
                            yyval = list_create(*yyvsp.offset(-(1 as i32) as isize));
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        62 => {
                            let ref mut fresh55 = (**yyvsp.offset(-(2 as i32) as isize))
                                .d
                                .di;
                            *fresh55 = ip_end;
                            let ref mut fresh56 = (**yyvsp.offset(-(2 as i32) as isize))
                                .x
                                .xi;
                            *fresh56 = ip_atexit;
                            if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                yyval = list_create(*yyvsp.offset(-(2 as i32) as isize));
                                list_prepend(
                                    yyval,
                                    bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32),
                                );
                                (*(*yyval).nexti).d.dn = dupnode(Nnull_string);
                            } else {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    *yyvsp.offset(-(2 as i32) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        63 => {
                            if !in_function {
                                yyerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"`return' used outside function context\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                        }
                        64 => {
                            if called_from_eval {
                                (**yyvsp.offset(-(3 as i32) as isize)).opcode = opcodeval::Op_K_return_from_eval;
                            }
                            if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                yyval = list_create(*yyvsp.offset(-(3 as i32) as isize));
                                list_prepend(
                                    yyval,
                                    bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32),
                                );
                                (*(*yyval).nexti).d.dn = dupnode(Nnull_string);
                            } else {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    *yyvsp.offset(-(3 as i32) as isize),
                                );
                            }
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    yyval,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        65 => {
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = list_append(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            } else {
                                yyval = *yyvsp.offset(-(1 as i32) as isize);
                            }
                        }
                        66 => {
                            in_print = 1 as i32 != 0;
                            in_parens = 0 as i32;
                        }
                        67 => {
                            let mut current_block_629: u64;
                            if do_optimize as i32 != 0
                                && (**yyvsp.offset(-(3 as i32) as isize)).opcode as u32
                                    == opcodeval::Op_K_print as i32 as u32
                                && ((*yyvsp.offset(-(1 as i32) as isize)).is_null()
                                    || (*(**yyvsp.offset(-(1 as i32) as isize)).d.di).opcode
                                        as u32 == opcodeval::Op_field_spec as i32 as u32
                                        && (*(*(**yyvsp.offset(-(1 as i32) as isize)).nexti).nexti)
                                            .nexti == (**yyvsp.offset(-(1 as i32) as isize)).d.di
                                        && (*(*(**yyvsp.offset(-(1 as i32) as isize)).nexti).nexti)
                                            .opcode as u32 == opcodeval::Op_push_i as i32 as u32
                                        && (*(*(*(**yyvsp.offset(-(1 as i32) as isize)).nexti)
                                            .nexti)
                                            .d
                                            .dn)
                                            .type_0 as u32 == nodevals::Node_val as i32 as u32)
                            {
                                static mut warned: bool = 0 as i32 != 0;
                                if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                    let mut n_0: *mut NODE = (*(*(**yyvsp
                                        .offset(-(1 as i32) as isize))
                                        .nexti)
                                        .nexti)
                                        .d
                                        .dn;
                                    if (*n_0).flags as u32
                                        & (flagvals::STRING as i32 | flagvals::STRCUR as i32) as u32
                                        != 0 as i32 as u32 || !((*n_0).sub.val.fltnum == 0.0f64)
                                    {
                                        current_block_629 = 93922101832262581;
                                    } else {
                                        bcfree((**yyvsp.offset(-(1 as i32) as isize)).d.di);
                                        unref(n_0);
                                        bcfree(
                                            (*(**yyvsp.offset(-(1 as i32) as isize)).nexti).nexti,
                                        );
                                        bcfree((**yyvsp.offset(-(1 as i32) as isize)).nexti);
                                        bcfree(*yyvsp.offset(-(1 as i32) as isize));
                                        current_block_629 = 9828823676310104886;
                                    }
                                } else {
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        && (rule == defrule::BEGIN as i32
                                            || rule == defrule::END as i32) && !warned
                                    {
                                        warned = 1 as i32 != 0;
                                        lintwarn_ln(
                                            (**yyvsp.offset(-(3 as i32) as isize)).source_line as i32,
                                            dcgettext(
                                                0 as *const i8,
                                                b"plain `print' in defrule::BEGIN or defrule::END rule should probably be `print \"\"'\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    }
                                    current_block_629 = 9828823676310104886;
                                }
                                match current_block_629 {
                                    93922101832262581 => {}
                                    _ => {
                                        (**yyvsp.offset(-(3 as i32) as isize)).x.xl = 0 as i32
                                            as i64;
                                        (**yyvsp.offset(-(3 as i32) as isize)).opcode = opcodeval::Op_K_print_rec;
                                        if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as i32) as isize)).d.dl = redirval::redirect_none
                                                as i32 as i64;
                                            yyval = list_create(*yyvsp.offset(-(3 as i32) as isize));
                                        } else {
                                            let mut ip_5: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                            ip_5 = (**yyvsp.offset(0 as i32 as isize)).nexti;
                                            (**yyvsp.offset(-(3 as i32) as isize)).d.dl = (*ip_5).d.dl;
                                            let ref mut fresh57 = (**yyvsp.offset(0 as i32 as isize))
                                                .nexti;
                                            *fresh57 = (*ip_5).nexti;
                                            bcfree(ip_5);
                                            yyval = list_append(
                                                *yyvsp.offset(0 as i32 as isize),
                                                *yyvsp.offset(-(3 as i32) as isize),
                                            );
                                        }
                                        current_block_629 = 279356324420966219;
                                    }
                                }
                            } else {
                                current_block_629 = 93922101832262581;
                            }
                            match current_block_629 {
                                93922101832262581 => {
                                    if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                        if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as i32) as isize)).x.xl = 0 as i32
                                                as i64;
                                            if (**yyvsp.offset(-(3 as i32) as isize)).opcode as u32
                                                == opcodeval::Op_K_print as i32 as u32
                                            {
                                                (**yyvsp.offset(-(3 as i32) as isize)).opcode = opcodeval::Op_K_print_rec;
                                            }
                                            (**yyvsp.offset(-(3 as i32) as isize)).d.dl = redirval::redirect_none
                                                as i32 as i64;
                                            yyval = list_create(*yyvsp.offset(-(3 as i32) as isize));
                                        } else {
                                            let mut t: *mut INSTRUCTION = *yyvsp
                                                .offset(-(1 as i32) as isize);
                                            (**yyvsp.offset(-(3 as i32) as isize)).x.xl = count_expressions(
                                                &mut t,
                                                0 as i32 != 0,
                                            ) as i64;
                                            (**yyvsp.offset(-(3 as i32) as isize)).d.dl = redirval::redirect_none
                                                as i32 as i64;
                                            yyval = list_append(t, *yyvsp.offset(-(3 as i32) as isize));
                                        }
                                    } else {
                                        let mut ip_6: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                                        ip_6 = (**yyvsp.offset(0 as i32 as isize)).nexti;
                                        (**yyvsp.offset(-(3 as i32) as isize)).d.dl = (*ip_6).d.dl;
                                        let ref mut fresh58 = (**yyvsp.offset(0 as i32 as isize))
                                            .nexti;
                                        *fresh58 = (*ip_6).nexti;
                                        bcfree(ip_6);
                                        if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                            (**yyvsp.offset(-(3 as i32) as isize)).x.xl = 0 as i32
                                                as i64;
                                            if (**yyvsp.offset(-(3 as i32) as isize)).opcode as u32
                                                == opcodeval::Op_K_print as i32 as u32
                                            {
                                                (**yyvsp.offset(-(3 as i32) as isize)).opcode = opcodeval::Op_K_print_rec;
                                            }
                                            yyval = list_append(
                                                *yyvsp.offset(0 as i32 as isize),
                                                *yyvsp.offset(-(3 as i32) as isize),
                                            );
                                        } else {
                                            let mut t_0: *mut INSTRUCTION = *yyvsp
                                                .offset(-(1 as i32) as isize);
                                            (**yyvsp.offset(-(3 as i32) as isize)).x.xl = count_expressions(
                                                &mut t_0,
                                                0 as i32 != 0,
                                            ) as i64;
                                            yyval = list_append(
                                                list_merge(*yyvsp.offset(0 as i32 as isize), t_0),
                                                *yyvsp.offset(-(3 as i32) as isize),
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        68 => {
                            sub_counter = 0 as i32;
                        }
                        69 => {
                            let mut arr_0: *mut i8 = (**yyvsp
                                .offset(-(2 as i32) as isize))
                                .d
                                .name;
                            (**yyvsp.offset(-(2 as i32) as isize)).opcode = opcodeval::Op_push_array;
                            let ref mut fresh59 = (**yyvsp.offset(-(2 as i32) as isize))
                                .d
                                .dn;
                            *fresh59 = variable(
                                (**yyvsp.offset(-(2 as i32) as isize)).source_line as i32,
                                arr_0,
                                nodevals::Node_var_new,
                            );
                            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32
                                == 0
                                && do_flags as u32
                                    & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                            {
                                if (**yyvsp.offset(-(2 as i32) as isize)).d.dn
                                    == symbol_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const i8,
                                            i32,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const i8,
                                        1255 as i32,
                                    );
                                    (Some(
                                        (Some(
                                            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                        ))
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            0 as *const i8,
                                            b"`delete' is not allowed with SYMTAB\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else if (**yyvsp.offset(-(2 as i32) as isize)).d.dn
                                    == func_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const i8,
                                            i32,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const i8,
                                        1257 as i32,
                                    );
                                    (Some(
                                        (Some(
                                            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                        ))
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            0 as *const i8,
                                            b"`delete' is not allowed with FUNCTAB\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                }
                            }
                            if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                (**yyvsp.offset(-(3 as i32) as isize)).x.xl = 0 as i32
                                    as i64;
                                yyval = list_append(
                                    list_create(*yyvsp.offset(-(2 as i32) as isize)),
                                    *yyvsp.offset(-(3 as i32) as isize),
                                );
                            } else {
                                (**yyvsp.offset(-(3 as i32) as isize)).x.xl = sub_counter
                                    as i64;
                                yyval = list_append(
                                    list_append(
                                        *yyvsp.offset(0 as i32 as isize),
                                        *yyvsp.offset(-(2 as i32) as isize),
                                    ),
                                    *yyvsp.offset(-(3 as i32) as isize),
                                );
                            }
                        }
                        70 => {
                            static mut warned_0: bool = 0 as i32 != 0;
                            let mut arr_1: *mut i8 = (**yyvsp
                                .offset(-(1 as i32) as isize))
                                .d
                                .name;
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && !warned_0
                            {
                                warned_0 = 1 as i32 != 0;
                                lintwarn_ln(
                                    (**yyvsp.offset(-(3 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`delete(array)' is a non-portable tawk extension\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            if do_flags as u32
                                & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
                            {
                                error_ln(
                                    (**yyvsp.offset(-(3 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"`delete(array)' is a non-portable tawk extension\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let ref mut fresh60 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .dn;
                            *fresh60 = variable(
                                (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                arr_1,
                                nodevals::Node_var_new,
                            );
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_push_array;
                            (**yyvsp.offset(-(3 as i32) as isize)).x.xl = 0 as i32
                                as i64;
                            yyval = list_append(
                                list_create(*yyvsp.offset(-(1 as i32) as isize)),
                                *yyvsp.offset(-(3 as i32) as isize),
                            );
                            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32
                                == 0
                                && do_flags as u32
                                    & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                            {
                                if (**yyvsp.offset(-(1 as i32) as isize)).d.dn
                                    == symbol_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const i8,
                                            i32,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const i8,
                                        1304 as i32,
                                    );
                                    (Some(
                                        (Some(
                                            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                        ))
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            0 as *const i8,
                                            b"`delete' is not allowed with SYMTAB\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                } else if (**yyvsp.offset(-(1 as i32) as isize)).d.dn
                                    == func_table
                                {
                                    (set_loc
                                        as unsafe extern "C" fn(
                                            *const i8,
                                            i32,
                                        ) -> ())(
                                        b"awkgram.y\0" as *const u8 as *const i8,
                                        1306 as i32,
                                    );
                                    (Some(
                                        (Some(
                                            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                        ))
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dcgettext(
                                            0 as *const i8,
                                            b"`delete' is not allowed with FUNCTAB\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                }
                            }
                        }
                        71 => {
                            yyval = optimize_assignment(
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        72 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        73 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        74 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        75 => {
                            if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                            } else {
                                yyval = list_prepend(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        76 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        77 => {
                            let mut casestmt_0: *mut INSTRUCTION = *yyvsp
                                .offset(0 as i32 as isize);
                            if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                casestmt_0 = list_create(
                                    bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                                );
                            }
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                list_prepend(
                                    casestmt_0,
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                            }
                            let ref mut fresh61 = (**yyvsp.offset(-(4 as i32) as isize))
                                .d
                                .di;
                            *fresh61 = *yyvsp.offset(-(3 as i32) as isize);
                            let ref mut fresh62 = (**yyvsp.offset(-(4 as i32) as isize))
                                .x
                                .xi;
                            *fresh62 = casestmt_0;
                            let ref mut fresh63 = (**yyvsp.offset(-(4 as i32) as isize))
                                .comment;
                            *fresh63 = *yyvsp.offset(-(1 as i32) as isize);
                            bcfree(*yyvsp.offset(-(2 as i32) as isize));
                            yyval = *yyvsp.offset(-(4 as i32) as isize);
                        }
                        78 => {
                            let mut casestmt_1: *mut INSTRUCTION = *yyvsp
                                .offset(0 as i32 as isize);
                            if (*yyvsp.offset(0 as i32 as isize)).is_null() {
                                casestmt_1 = list_create(
                                    bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                                );
                            }
                            if do_flags as u32
                                & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            {
                                list_prepend(
                                    casestmt_1,
                                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                                );
                            }
                            bcfree(*yyvsp.offset(-(2 as i32) as isize));
                            let ref mut fresh64 = (**yyvsp.offset(-(3 as i32) as isize))
                                .x
                                .xi;
                            *fresh64 = casestmt_1;
                            let ref mut fresh65 = (**yyvsp.offset(-(3 as i32) as isize))
                                .comment;
                            *fresh65 = *yyvsp.offset(-(1 as i32) as isize);
                            yyval = *yyvsp.offset(-(3 as i32) as isize);
                        }
                        79 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        80 => {
                            let mut n_1: *mut NODE = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .dn;
                            force_number(n_1);
                            negate_num(n_1);
                            bcfree(*yyvsp.offset(-(1 as i32) as isize));
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        81 => {
                            let mut n_2: *mut NODE = (*(**yyvsp
                                .offset(0 as i32 as isize))
                                .d
                                .di)
                                .d
                                .dn;
                            bcfree(*yyvsp.offset(-(1 as i32) as isize));
                            add_sign_to_num(n_2, '+' as i32 as i8);
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        82 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        83 => {
                            if (*(**yyvsp.offset(0 as i32 as isize)).d.dn).type_0 as u32
                                == nodevals::Node_regex as i32 as u32
                            {
                                (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_push_re;
                            } else {
                                (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_push;
                            }
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        84 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_push_re;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        85 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        86 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        88 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        89 => {
                            in_print = 0 as i32 != 0;
                            in_parens = 0 as i32;
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        90 => {
                            in_print = 0 as i32 != 0;
                            in_parens = 0 as i32;
                        }
                        91 => {
                            if (**yyvsp.offset(-(2 as i32) as isize)).d.dl
                                == redirval::redirect_twoway as i32 as i64
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                    == opcodeval::Op_K_getline_redir as i32 as u32
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).d.dl
                                    == redirval::redirect_twoway as i32 as i64
                            {
                                yyerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"multistage two-way pipelines don't work\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && (**yyvsp.offset(-(2 as i32) as isize)).d.dl
                                    == redirval::redirect_output as i32 as i64
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                    == opcodeval::Op_concat as i32 as u32
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const i8,
                                    1433 as i32,
                                );
                                (Some(lintfunc.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        0 as *const i8,
                                        b"concatenation as I/O `>' redirection target is ambiguous\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yyval = list_prepend(
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                            );
                        }
                        92 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh66 = (**yyvsp.offset(-(5 as i32) as isize))
                                    .comment;
                                *fresh66 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            add_lint(
                                *yyvsp.offset(-(3 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            yyval = mk_condition(
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(-(5 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                0 as *mut INSTRUCTION,
                                0 as *mut INSTRUCTION,
                            );
                        }
                        93 => {
                            if !(*yyvsp.offset(-(4 as i32) as isize)).is_null() {
                                let ref mut fresh67 = (**yyvsp.offset(-(8 as i32) as isize))
                                    .comment;
                                *fresh67 = *yyvsp.offset(-(4 as i32) as isize);
                            }
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh68 = (**yyvsp.offset(-(2 as i32) as isize))
                                    .comment;
                                *fresh68 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            add_lint(
                                *yyvsp.offset(-(6 as i32) as isize),
                                lintvals::LINT_assign_in_cond,
                            );
                            yyval = mk_condition(
                                *yyvsp.offset(-(6 as i32) as isize),
                                *yyvsp.offset(-(8 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        94 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        95 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null()
                                && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                if (*(**yyvsp.offset(-(1 as i32) as isize)).d.dn)
                                    .sub
                                    .val
                                    .comtype as u32 == commenttype::EOL_COMMENT as i32 as u32
                                {
                                    let ref mut fresh69 = (**yyvsp.offset(-(1 as i32) as isize))
                                        .comment;
                                    *fresh69 = *yyvsp.offset(0 as i32 as isize);
                                } else {
                                    merge_comments(
                                        *yyvsp.offset(-(1 as i32) as isize),
                                        *yyvsp.offset(0 as i32 as isize),
                                    );
                                }
                                yyval = *yyvsp.offset(-(1 as i32) as isize);
                            } else if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                yyval = *yyvsp.offset(-(1 as i32) as isize);
                            } else if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                yyval = *yyvsp.offset(0 as i32 as isize);
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        96 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        97 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        98 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        99 => {
                            bcfree(*yyvsp.offset(-(1 as i32) as isize));
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        100 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        101 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        102 => {
                            (**yyvsp.offset(0 as i32 as isize)).x.xl = 0 as i32 as i64;
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        103 => {
                            if !(*yyvsp.offset(-(2 as i32) as isize)).is_null()
                                && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                (**yyvsp.offset(0 as i32 as isize)).x.xl = (*(**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .d
                                    .di)
                                    .x
                                    .xl + 1 as i32 as i64;
                                yyval = list_append(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                                yyerrstatus = 0 as i32;
                                if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                    if !((**yyvsp.offset(-(2 as i32) as isize)).comment)
                                        .is_null()
                                    {
                                        merge_comments(
                                            (**yyvsp.offset(-(2 as i32) as isize)).comment,
                                            *yyvsp.offset(-(1 as i32) as isize),
                                        );
                                    } else {
                                        let ref mut fresh70 = (**yyvsp.offset(-(2 as i32) as isize))
                                            .comment;
                                        *fresh70 = *yyvsp.offset(-(1 as i32) as isize);
                                    }
                                }
                            } else {
                                yyval = 0 as *mut INSTRUCTION;
                            }
                        }
                        104 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        105 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        106 => {
                            yyval = *yyvsp.offset(-(2 as i32) as isize);
                        }
                        107 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        108 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        109 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        110 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        111 => {
                            yyval = mk_expression_list(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        112 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh71 = (*(**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh71 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            yyerrstatus = 0 as i32;
                        }
                        113 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        114 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        115 => {
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        116 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh72 = (*(**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh72 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            yyval = *yyvsp.offset(-(2 as i32) as isize);
                        }
                        117 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        118 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        119 => {
                            yyval = mk_expression_list(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        120 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh73 = (*(**yyvsp
                                    .offset(-(2 as i32) as isize))
                                    .d
                                    .di)
                                    .comment;
                                *fresh73 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            yyerrstatus = 0 as i32;
                        }
                        121 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        122 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        123 => {
                            yyval = mk_expression_list(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        124 => {
                            if !(*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                let ref mut fresh74 = (**yyvsp.offset(-(2 as i32) as isize))
                                    .comment;
                                *fresh74 = *yyvsp.offset(-(1 as i32) as isize);
                            }
                            yyval = *yyvsp.offset(-(2 as i32) as isize);
                        }
                        125 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        126 => {
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        127 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        128 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        129 => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                    == opcodeval::Op_match_rec as i32 as u32
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"regular expression on right of assignment\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yyval = mk_assignment(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        130 => {
                            yyval = mk_assignment(
                                *yyvsp.offset(-(2 as i32) as isize),
                                list_create(*yyvsp.offset(0 as i32 as isize)),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        131 => {
                            yyval = mk_boolean(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        132 => {
                            yyval = mk_boolean(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        133 => {
                            if (*(**yyvsp.offset(-(2 as i32) as isize)).d.di).opcode
                                as u32 == opcodeval::Op_match_rec as i32 as u32
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"regular expression on left of `~' or `!~' operator\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let ref mut fresh75 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .dn;
                            *fresh75 = (**yyvsp.offset(0 as i32 as isize)).d.dn;
                            bcfree(*yyvsp.offset(0 as i32 as isize));
                            yyval = list_append(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        134 => {
                            if (*(**yyvsp.offset(-(2 as i32) as isize)).d.di).opcode
                                as u32 == opcodeval::Op_match_rec as i32 as u32
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"regular expression on left of `~' or `!~' operator\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            if (**yyvsp.offset(0 as i32 as isize)).d.di
                                == (**yyvsp.offset(0 as i32 as isize)).nexti
                                && (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_match_rec as i32 as u32
                            {
                                let ref mut fresh76 = (**yyvsp.offset(-(1 as i32) as isize))
                                    .d
                                    .dn;
                                *fresh76 = (*(**yyvsp.offset(0 as i32 as isize)).nexti)
                                    .d
                                    .dn;
                                bcfree((**yyvsp.offset(0 as i32 as isize)).nexti);
                                bcfree(*yyvsp.offset(0 as i32 as isize));
                                yyval = list_append(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            } else {
                                let ref mut fresh77 = (**yyvsp.offset(-(1 as i32) as isize))
                                    .d
                                    .dn;
                                *fresh77 = make_regnode(
                                    nodevals::Node_dynregex,
                                    0 as *mut NODE,
                                );
                                yyval = list_append(
                                    list_merge(
                                        *yyvsp.offset(-(2 as i32) as isize),
                                        *yyvsp.offset(0 as i32 as isize),
                                    ),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                        }
                        135 => {
                            if do_flags as u32
                                & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"old awk does not support the keyword `in' except after `for'\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode = opcodeval::Op_push_array;
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_in_array;
                            (**yyvsp.offset(-(1 as i32) as isize)).x.xl = 1 as i32
                                as i64;
                            yyval = list_append(
                                list_merge(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                ),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        136 => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                    == opcodeval::Op_match_rec as i32 as u32
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"regular expression on right of comparison\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yyval = list_append(
                                list_merge(
                                    *yyvsp.offset(-(2 as i32) as isize),
                                    *yyvsp.offset(0 as i32 as isize),
                                ),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        137 => {
                            yyval = mk_condition(
                                *yyvsp.offset(-(4 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        138 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        139 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        140 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        141 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_assign_quotient;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        142 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        143 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        144 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        145 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        146 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        147 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        148 => {
                            let mut count: i32 = 2 as i32;
                            let mut is_simple_var: bool = 0 as i32 != 0;
                            if (*(**yyvsp.offset(-(1 as i32) as isize)).d.di).opcode
                                as u32 == opcodeval::Op_concat as i32 as u32
                            {
                                is_simple_var = (*(**yyvsp.offset(-(1 as i32) as isize))
                                    .d
                                    .di)
                                    .d
                                    .dl & 2 as i32 as i64 != 0 as i32 as i64;
                                count = ((*(**yyvsp.offset(-(1 as i32) as isize)).d.di).x.xl
                                    + 1 as i32 as i64) as i32;
                                (*(**yyvsp.offset(-(1 as i32) as isize)).d.di).opcode = opcodeval::Op_no_op;
                            } else {
                                is_simple_var = (*(**yyvsp.offset(-(1 as i32) as isize))
                                    .nexti)
                                    .opcode as u32 == opcodeval::Op_push as i32 as u32
                                    && (**yyvsp.offset(-(1 as i32) as isize)).d.di
                                        == (**yyvsp.offset(-(1 as i32) as isize)).nexti;
                            }
                            let mut current_block_948: u64;
                            if do_optimize as i32 != 0
                                && (**yyvsp.offset(-(1 as i32) as isize)).nexti
                                    == (**yyvsp.offset(-(1 as i32) as isize)).d.di
                                && (*(**yyvsp.offset(-(1 as i32) as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_push_i as i32 as u32
                                && (**yyvsp.offset(0 as i32 as isize)).nexti
                                    == (**yyvsp.offset(0 as i32 as isize)).d.di
                                && (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_push_i as i32 as u32
                            {
                                let mut n1: *mut NODE = (*(**yyvsp
                                    .offset(-(1 as i32) as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                let mut n2: *mut NODE = (*(**yyvsp
                                    .offset(0 as i32 as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                let mut nlen: size_t = 0;
                                if (*n1).flags as u32
                                    & (flagvals::NUMBER as i32 | flagvals::NUMINT as i32
                                        | flagvals::INTLSTR as i32) as u32 != 0 as i32 as u32
                                    || (*n2).flags as u32
                                        & (flagvals::NUMBER as i32 | flagvals::NUMINT as i32
                                            | flagvals::INTLSTR as i32) as u32 != 0 as i32 as u32
                                {
                                    current_block_948 = 16952324956488869110;
                                } else {
                                    n1 = force_string_fmt(n1, CONVFMT, CONVFMTidx);
                                    n2 = force_string_fmt(n2, CONVFMT, CONVFMTidx);
                                    nlen = ((*n1).sub.val.slen)
                                        .wrapping_add((*n2).sub.val.slen);
                                    (*n1).sub.val.sp = erealloc_real(
                                        (*n1).sub.val.sp as *mut libc::c_void,
                                        nlen.wrapping_add(1 as i32 as u64),
                                        b"constant fold\0" as *const u8 as *const i8,
                                        b"n1->stptr\0" as *const u8 as *const i8,
                                        b"awkgram.y\0" as *const u8 as *const i8,
                                        1775 as i32,
                                    ) as *mut i8;
                                    memcpy(
                                        ((*n1).sub.val.sp).offset((*n1).sub.val.slen as isize)
                                            as *mut libc::c_void,
                                        (*n2).sub.val.sp as *const libc::c_void,
                                        (*n2).sub.val.slen,
                                    );
                                    (*n1).sub.val.slen = nlen;
                                    *((*n1).sub.val.sp).offset(nlen as isize) = '\0' as i32
                                        as i8;
                                    (*n1).flags = ::core::mem::transmute::<
                                        u32,
                                        flagvals,
                                    >(
                                        (*n1).flags as u32
                                            & !(flagvals::NUMCUR as i32 | flagvals::NUMBER as i32
                                                | flagvals::NUMINT as i32) as u32,
                                    );
                                    (*n1).flags = ::core::mem::transmute::<
                                        u32,
                                        flagvals,
                                    >(
                                        (*n1).flags as u32
                                            | (flagvals::STRING as i32 | flagvals::STRCUR as i32) as u32,
                                    );
                                    unref(n2);
                                    bcfree((**yyvsp.offset(0 as i32 as isize)).nexti);
                                    bcfree(*yyvsp.offset(0 as i32 as isize));
                                    yyval = *yyvsp.offset(-(1 as i32) as isize);
                                    current_block_948 = 2403495654184706625;
                                }
                            } else {
                                current_block_948 = 16952324956488869110;
                            }
                            match current_block_948 {
                                16952324956488869110 => {
                                    yyval = list_append(
                                        list_merge(
                                            *yyvsp.offset(-(1 as i32) as isize),
                                            *yyvsp.offset(0 as i32 as isize),
                                        ),
                                        bcalloc(opcodeval::Op_concat, 1 as i32, 0 as i32),
                                    );
                                    (*(*yyval).d.di).d.dl = (if is_simple_var as i32 != 0 {
                                        2 as i32
                                    } else {
                                        0 as i32
                                    }) as i64;
                                    (*(*yyval).d.di).x.xl = count as i64;
                                    if count > max_args {
                                        max_args = count;
                                    }
                                }
                                _ => {}
                            }
                        }
                        150 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        151 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        152 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        153 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        154 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        155 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        156 => {
                            if (rule == defrule::BEGINFILE as i32
                                || rule == defrule::ENDFILE as i32)
                                && (*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                error_ln(
                                    (**yyvsp.offset(-(2 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"non-redirected `getline' invalid inside `%s' rule\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    ruletab[rule as usize],
                                );
                            }
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && rule == defrule::END as i32
                                && (*yyvsp.offset(0 as i32 as isize)).is_null()
                            {
                                lintwarn_ln(
                                    (**yyvsp.offset(-(2 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"non-redirected `getline' undefined inside defrule::END action\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yyval = mk_getline(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                redirval::redirect_input as i32,
                            );
                        }
                        157 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_postincrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(-(1 as i32) as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        158 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_postdecrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(-(1 as i32) as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        159 => {
                            if do_flags as u32
                                & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                            {
                                warning_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"old awk does not support the keyword `in' except after `for'\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                lintwarn_ln(
                                    (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"old awk does not support multidimensional arrays\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode = opcodeval::Op_push_array;
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_in_array;
                            if (*yyvsp.offset(-(3 as i32) as isize)).is_null() {
                                errcount += 1;
                                errcount;
                                (**yyvsp.offset(-(1 as i32) as isize)).x.xl = 0 as i32
                                    as i64;
                                yyval = list_merge(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            } else {
                                let mut t_1: *mut INSTRUCTION = *yyvsp
                                    .offset(-(3 as i32) as isize);
                                (**yyvsp.offset(-(1 as i32) as isize)).x.xl = count_expressions(
                                    &mut t_1,
                                    0 as i32 != 0,
                                ) as i64;
                                yyval = list_append(
                                    list_merge(t_1, *yyvsp.offset(0 as i32 as isize)),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                        }
                        160 => {
                            yyval = mk_getline(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                                (**yyvsp.offset(-(2 as i32) as isize)).d.dl as i32,
                            );
                            bcfree(*yyvsp.offset(-(2 as i32) as isize));
                        }
                        161 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        162 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        163 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        164 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        165 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        166 => {
                            yyval = mk_binary(
                                *yyvsp.offset(-(2 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        167 => {
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        168 => {
                            if (**yyvsp.offset(0 as i32 as isize)).opcode as u32
                                == opcodeval::Op_match_rec as i32 as u32
                            {
                                (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_nomatch;
                                (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_push_i;
                                let ref mut fresh78 = (**yyvsp.offset(-(1 as i32) as isize))
                                    .d
                                    .dn;
                                *fresh78 = set_profile_text(
                                    make_number.expect("non-null function pointer")(0.0f64),
                                    b"0\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                yyval = list_append(
                                    list_append(
                                        list_create(*yyvsp.offset(-(1 as i32) as isize)),
                                        bcalloc(opcodeval::Op_field_spec, 1 as i32, 0 as i32),
                                    ),
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            } else if do_optimize as i32 != 0
                                && (**yyvsp.offset(0 as i32 as isize)).nexti
                                    == (**yyvsp.offset(0 as i32 as isize)).d.di
                                && (*(**yyvsp.offset(0 as i32 as isize)).nexti).opcode
                                    as u32 == opcodeval::Op_push_i as i32 as u32
                                && (*(*(**yyvsp.offset(0 as i32 as isize)).nexti).d.dn)
                                    .flags as u32
                                    & (flagvals::MPFN as i32 | flagvals::MPZN as i32
                                        | flagvals::INTLSTR as i32) as u32 == 0 as i32 as u32
                            {
                                let mut n_3: *mut NODE = (*(**yyvsp
                                    .offset(0 as i32 as isize))
                                    .nexti)
                                    .d
                                    .dn;
                                if (*n_3).flags as u32 & flagvals::STRING as i32 as u32
                                    != 0 as i32 as u32
                                {
                                    (*n_3).sub.val.fltnum = ((*n_3).sub.val.slen
                                        == 0 as i32 as u64) as i32 as libc::c_double;
                                    (*n_3).flags = ::core::mem::transmute::<
                                        u32,
                                        flagvals,
                                    >(
                                        (*n_3).flags as u32
                                            & !(flagvals::STRCUR as i32 | flagvals::STRING as i32)
                                                as u32,
                                    );
                                    (*n_3).flags = ::core::mem::transmute::<
                                        u32,
                                        flagvals,
                                    >(
                                        (*n_3).flags as u32
                                            | (flagvals::NUMCUR as i32 | flagvals::NUMBER as i32) as u32,
                                    );
                                    pma_free((*n_3).sub.val.sp as *mut libc::c_void);
                                    (*n_3).sub.val.sp = 0 as *mut i8;
                                    (*n_3).sub.val.slen = 0 as i32 as size_t;
                                } else {
                                    (*n_3).sub.val.fltnum = ((*n_3).sub.val.fltnum == 0.0f64)
                                        as i32 as libc::c_double;
                                }
                                bcfree(*yyvsp.offset(-(1 as i32) as isize));
                                yyval = *yyvsp.offset(0 as i32 as isize);
                            } else {
                                (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_not;
                                add_lint(
                                    *yyvsp.offset(0 as i32 as isize),
                                    lintvals::LINT_assign_in_cond,
                                );
                                yyval = list_append(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                        }
                        169 => {
                            yyval = list_append(
                                *yyvsp.offset(-(1 as i32) as isize),
                                bcalloc(opcodeval::Op_parens, 1 as i32, sourceline),
                            );
                        }
                        170 => {
                            yyval = snode(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                            );
                            if yyval.is_null() {
                                current_block = 11477185723268747871;
                                break;
                            }
                        }
                        171 => {
                            yyval = snode(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(-(3 as i32) as isize),
                            );
                            if yyval.is_null() {
                                current_block = 11477185723268747871;
                                break;
                            }
                        }
                        172 => {
                            static mut warned_1: bool = 0 as i32 != 0;
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                && !warned_1
                            {
                                warned_1 = 1 as i32 != 0;
                                lintwarn_ln(
                                    (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"call of `length' without parentheses is not portable\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yyval = snode(
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(0 as i32 as isize),
                            );
                            if yyval.is_null() {
                                current_block = 11477185723268747871;
                                break;
                            }
                        }
                        175 => {
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_preincrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(0 as i32 as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        176 => {
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_predecrement;
                            yyval = mk_assignment(
                                *yyvsp.offset(0 as i32 as isize),
                                0 as *mut INSTRUCTION,
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        177 => {
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        178 => {
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        179 => {
                            if (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                == opcodeval::Op_push_i as i32 as u32
                                && (*(*(**yyvsp.offset(0 as i32 as isize)).d.di).d.dn).flags
                                    as u32 & flagvals::STRING as i32 as u32 == 0 as i32 as u32
                            {
                                let mut n_4: *mut NODE = (*(**yyvsp
                                    .offset(0 as i32 as isize))
                                    .d
                                    .di)
                                    .d
                                    .dn;
                                force_number(n_4);
                                negate_num(n_4);
                                yyval = *yyvsp.offset(0 as i32 as isize);
                                bcfree(*yyvsp.offset(-(1 as i32) as isize));
                            } else {
                                (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_unary_minus;
                                yyval = list_append(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                        }
                        180 => {
                            if (*(**yyvsp.offset(0 as i32 as isize)).d.di).opcode as u32
                                == opcodeval::Op_push_i as i32 as u32
                                && (*(*(**yyvsp.offset(0 as i32 as isize)).d.di).d.dn).flags
                                    as u32 & flagvals::STRING as i32 as u32 == 0 as i32 as u32
                                && (*(*(**yyvsp.offset(0 as i32 as isize)).d.di).d.dn).flags
                                    as u32 & flagvals::NUMCONSTSTR as i32 as u32
                                    != 0 as i32 as u32
                            {
                                let mut n_5: *mut NODE = (*(**yyvsp
                                    .offset(0 as i32 as isize))
                                    .d
                                    .di)
                                    .d
                                    .dn;
                                add_sign_to_num(n_5, '+' as i32 as i8);
                                yyval = *yyvsp.offset(0 as i32 as isize);
                                bcfree(*yyvsp.offset(-(1 as i32) as isize));
                            } else {
                                (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_unary_plus;
                                yyval = list_append(
                                    *yyvsp.offset(0 as i32 as isize),
                                    *yyvsp.offset(-(1 as i32) as isize),
                                );
                            }
                        }
                        181 => {
                            func_use(
                                (*(**yyvsp.offset(0 as i32 as isize)).d.di).d.name,
                                defref::FUNC_USE,
                            );
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        182 => {
                            let mut f: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut t_2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                            let mut name_0: *mut i8 = 0 as *mut i8;
                            let mut indirect_var: *mut NODE = 0 as *mut NODE;
                            static mut warned_2: bool = 0 as i32 != 0;
                            let mut msg_0: *const i8 = dcgettext(
                                0 as *const i8,
                                b"indirect function calls are a gawk extension\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            );
                            if do_flags as u32
                                & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
                                || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32
                                    != 0
                            {
                                yyerror(b"%s\0" as *const u8 as *const i8, msg_0);
                            } else if do_flags as u32
                                & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                                && !warned_2
                            {
                                warned_2 = 1 as i32 != 0;
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const i8,
                                    2025 as i32,
                                );
                                (Some(lintfunc.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(b"%s\0" as *const u8 as *const i8, msg_0);
                            }
                            f = (**yyvsp.offset(0 as i32 as isize)).d.di;
                            (*f).opcode = opcodeval::Op_indirect_func_call;
                            name_0 = estrdup((*f).d.name, strlen((*f).d.name));
                            if is_std_var(name_0) != 0 {
                                yyerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"cannot use special variable `%s' for indirect function call\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    name_0,
                                );
                            }
                            indirect_var = variable(
                                (*f).source_line as i32,
                                name_0,
                                nodevals::Node_var_new,
                            );
                            t_2 = bcalloc(opcodeval::Op_push, 1 as i32, 0 as i32);
                            (*t_2).d.dn = indirect_var;
                            yyval = list_prepend(*yyvsp.offset(0 as i32 as isize), t_2);
                            at_seen -= 1;
                            at_seen;
                        }
                        183 => {
                            let mut n_6: *mut NODE = 0 as *mut NODE;
                            let mut name_1: *mut i8 = (**yyvsp
                                .offset(-(3 as i32) as isize))
                                .d
                                .name;
                            let mut qname_0: *mut i8 = qualify_name(
                                name_1,
                                strlen(name_1),
                            );
                            if qname_0 != name_1 {
                                pma_free(name_1 as *mut libc::c_void);
                                let ref mut fresh79 = (**yyvsp.offset(-(3 as i32) as isize))
                                    .d
                                    .name;
                                *fresh79 = qname_0;
                            }
                            if at_seen == 0 {
                                n_6 = lookup((**yyvsp.offset(-(3 as i32) as isize)).d.name);
                                if !n_6.is_null()
                                    && (*n_6).type_0 as u32 != nodevals::Node_func as i32 as u32
                                    && (*n_6).type_0 as u32
                                        != nodevals::Node_ext_func as i32 as u32
                                {
                                    error_ln(
                                        (**yyvsp.offset(-(3 as i32) as isize)).source_line as i32,
                                        dcgettext(
                                            0 as *const i8,
                                            b"attempt to use non-function `%s' in function call\0"
                                                as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                        (**yyvsp.offset(-(3 as i32) as isize)).d.name,
                                    );
                                }
                            }
                            param_sanity(*yyvsp.offset(-(1 as i32) as isize));
                            (**yyvsp.offset(-(3 as i32) as isize)).opcode = opcodeval::Op_func_call;
                            let ref mut fresh80 = (**yyvsp.offset(-(3 as i32) as isize))
                                .x
                                .xn;
                            *fresh80 = 0 as *mut NODE;
                            if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                (*(*yyvsp.offset(-(3 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .x
                                    .xl = 0 as i32 as i64;
                                yyval = list_create(*yyvsp.offset(-(3 as i32) as isize));
                            } else {
                                let mut t_3: *mut INSTRUCTION = *yyvsp
                                    .offset(-(1 as i32) as isize);
                                (*(*yyvsp.offset(-(3 as i32) as isize))
                                    .offset(1 as i32 as isize))
                                    .x
                                    .xl = count_expressions(&mut t_3, 1 as i32 != 0) as i64;
                                yyval = list_append(
                                    t_3,
                                    *yyvsp.offset(-(3 as i32) as isize),
                                );
                            }
                        }
                        184 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        185 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        186 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        187 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        188 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        189 => {
                            yyval = list_merge(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        190 => {
                            let mut ip_7: *mut INSTRUCTION = (**yyvsp
                                .offset(0 as i32 as isize))
                                .d
                                .di;
                            let mut count_0: i32 = (*ip_7).d.dl as i32;
                            if count_0 > 1 as i32 {
                                (*ip_7).opcode = opcodeval::Op_concat;
                                (*ip_7).d.dl = 1 as i32 as i64;
                                (*ip_7).x.xl = count_0 as i64;
                            } else {
                                (*ip_7).opcode = opcodeval::Op_no_op;
                            }
                            sub_counter += 1;
                            sub_counter;
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        191 => {
                            let mut t_4: *mut INSTRUCTION = *yyvsp
                                .offset(-(1 as i32) as isize);
                            if (*yyvsp.offset(-(1 as i32) as isize)).is_null() {
                                error_ln(
                                    (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"invalid subscript expression\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                t_4 = list_create(
                                    bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32),
                                );
                                (*(*t_4).nexti).d.dn = dupnode(Nnull_string);
                                (**yyvsp.offset(0 as i32 as isize)).d.dl = 1 as i32 as i64;
                            } else {
                                (**yyvsp.offset(0 as i32 as isize)).d.dl = count_expressions(
                                    &mut t_4,
                                    0 as i32 != 0,
                                ) as i64;
                            }
                            yyval = list_append(t_4, *yyvsp.offset(0 as i32 as isize));
                        }
                        192 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        193 => {
                            yyval = list_merge(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(0 as i32 as isize),
                            );
                        }
                        194 => {
                            yyval = *yyvsp.offset(-(1 as i32) as isize);
                        }
                        195 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_push;
                            let ref mut fresh81 = (**yyvsp.offset(0 as i32 as isize))
                                .d
                                .dn;
                            *fresh81 = variable(
                                (**yyvsp.offset(0 as i32 as isize)).source_line as i32,
                                (**yyvsp.offset(0 as i32 as isize)).d.name,
                                nodevals::Node_var_new,
                            );
                            yyval = list_create(*yyvsp.offset(0 as i32 as isize));
                        }
                        196 => {
                            let mut arr_2: *mut i8 = (**yyvsp
                                .offset(-(1 as i32) as isize))
                                .d
                                .name;
                            let ref mut fresh82 = (**yyvsp.offset(-(1 as i32) as isize))
                                .d
                                .dn;
                            *fresh82 = variable(
                                (**yyvsp.offset(-(1 as i32) as isize)).source_line as i32,
                                arr_2,
                                nodevals::Node_var_new,
                            );
                            (**yyvsp.offset(-(1 as i32) as isize)).opcode = opcodeval::Op_push_array;
                            yyval = list_prepend(
                                *yyvsp.offset(0 as i32 as isize),
                                *yyvsp.offset(-(1 as i32) as isize),
                            );
                        }
                        197 => {
                            let mut ip_8: *mut INSTRUCTION = (**yyvsp
                                .offset(0 as i32 as isize))
                                .nexti;
                            if (*ip_8).opcode as u32 == opcodeval::Op_push as i32 as u32
                                && (*(*ip_8).d.dn).type_0 as u32
                                    == nodevals::Node_var as i32 as u32
                                && ((*(*ip_8).d.dn).sub.nodep.r.uptr).is_some()
                            {
                                yyval = list_prepend(
                                    *yyvsp.offset(0 as i32 as isize),
                                    bcalloc(opcodeval::Op_var_update, 1 as i32, 0 as i32),
                                );
                                (*(*yyval).nexti).x.aptr = (*(*ip_8).d.dn).sub.nodep.r.uptr;
                            } else {
                                yyval = *yyvsp.offset(0 as i32 as isize);
                            }
                        }
                        198 => {
                            yyval = list_append(
                                *yyvsp.offset(-(1 as i32) as isize),
                                *yyvsp.offset(-(2 as i32) as isize),
                            );
                            if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                                mk_assignment(
                                    *yyvsp.offset(-(1 as i32) as isize),
                                    0 as *mut INSTRUCTION,
                                    *yyvsp.offset(0 as i32 as isize),
                                );
                            }
                        }
                        199 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_postincrement;
                        }
                        200 => {
                            (**yyvsp.offset(0 as i32 as isize)).opcode = opcodeval::Op_postdecrement;
                        }
                        201 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        202 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                        }
                        203 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                            yyerrstatus = 0 as i32;
                        }
                        204 => {
                            yyerrstatus = 0 as i32;
                        }
                        205 => {
                            yyval = 0 as *mut INSTRUCTION;
                        }
                        207 => {
                            yyerrstatus = 0 as i32;
                        }
                        208 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                            yyerrstatus = 0 as i32;
                        }
                        209 => {
                            yyval = *yyvsp.offset(0 as i32 as isize);
                            yyerrstatus = 0 as i32;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as i32;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: i32 = yyr1[yyn as usize] as i32 - 77 as i32;
                    let yyi: i32 = yypgoto[yylhs as usize] as i32 + *yyssp as i32;
                    yystate = if 0 as i32 <= yyi && yyi <= 1200 as i32
                        && yycheck[yyi as usize] as i32 == *yyssp as i32
                    {
                        yytable[yyi as usize] as i32
                    } else {
                        yydefgoto[yylhs as usize] as i32
                    };
                    current_block = 7394502214160662244;
                }
                _ => {}
            }
            match current_block {
                14798202723045015423 => {
                    yyerrstatus = 3 as i32;
                    loop {
                        yyn = yypact[yystate as usize] as i32;
                        if !(yyn == -(276 as i32)) {
                            yyn += YYSYMBOL_YYerror as i32;
                            if 0 as i32 <= yyn && yyn <= 1200 as i32
                                && yycheck[yyn as usize] as i32 == YYSYMBOL_YYerror as i32
                            {
                                yyn = yytable[yyn as usize] as i32;
                                if (0 as i32) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 11477185723268747871;
                            break 's_46;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const i8,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                        );
                        yyvsp = yyvsp.offset(-(1 as i32 as isize));
                        yyssp = yyssp.offset(-(1 as i32 as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        10279487622471394456 => {
            yyerror(b"memory exhausted\0" as *const u8 as *const i8);
            yyresult = 2 as i32;
        }
        11477185723268747871 => {
            yyresult = 1 as i32;
        }
        _ => {}
    }
    if yychar != -(2 as i32) {
        yytoken = (if 0 as i32 <= yychar && yychar <= 310 as i32 {
            yytranslate[yychar as usize] as yysymbol_kind_t as i32
        } else {
            YYSYMBOL_YYUNDEF as i32
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const i8,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const i8,
            yystos[*yyssp as i32 as usize] as yysymbol_kind_t,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as i32 as isize));
        yyssp = yyssp.offset(-(1 as i32 as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        pma_free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
static mut tokentab: [token; 71] = unsafe {
    [
        {
            let mut init = token {
                operator: b"defrule::BEGIN\0" as *const u8 as *const i8,
                value: opcodeval::Op_rule,
                class: 273 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"defrule::BEGINFILE\0" as *const u8 as *const i8,
                value: opcodeval::Op_rule,
                class: 292 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"defrule::END\0" as *const u8 as *const i8,
                value: opcodeval::Op_rule,
                class: 274 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"defrule::ENDFILE\0" as *const u8 as *const i8,
                value: opcodeval::Op_rule,
                class: 293 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"and\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: 0x400 as i32 as u32,
                ptr: Some(do_and as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"asort\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32
                    | (1 as i32) << 3 as i32) as u32,
                ptr: Some(do_asort as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"asorti\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32
                    | (1 as i32) << 3 as i32) as u32,
                ptr: Some(do_asorti as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"atan2\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 2 as i32) as u32,
                ptr: Some(do_atan2 as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"bindtextdomain\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
                    as u32,
                ptr: Some(do_bindtextdomain as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"break\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_break,
                class: 285 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"case\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_case,
                class: 280 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"close\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
                    as u32,
                ptr: Some(do_close as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"compl\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_compl as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"continue\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_continue,
                class: 286 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"cos\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_cos as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"dcgettext\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32
                    | (1 as i32) << 3 as i32) as u32,
                ptr: Some(do_dcgettext as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"dcngettext\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32
                    | (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32
                    | (1 as i32) << 5 as i32) as u32,
                ptr: Some(do_dcngettext as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"default\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_default,
                class: 281 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"delete\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_delete,
                class: 278 as i32,
                flags: 0x100 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"do\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_do,
                class: 283 as i32,
                flags: (0x100 as i32 | 0x800 as i32 | 0x1000 as i32) as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"else\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_else,
                class: 276 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"eval\0" as *const u8 as *const i8,
                value: opcodeval::Op_symbol,
                class: 305 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"exit\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_exit,
                class: 290 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"exp\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 1 as i32) as u32,
                ptr: Some(do_exp as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"fflush\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 0 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_fflush as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"for\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_for,
                class: 284 as i32,
                flags: (0x800 as i32 | 0x1000 as i32) as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"func\0" as *const u8 as *const i8,
                value: opcodeval::Op_func,
                class: 291 as i32,
                flags: (0x200 as i32 | 0x100 as i32) as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"function\0" as *const u8 as *const i8,
                value: opcodeval::Op_func,
                class: 291 as i32,
                flags: 0x100 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"gensub\0" as *const u8 as *const i8,
                value: opcodeval::Op_sub_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 3 as i32 | (1 as i32) << 4 as i32)
                    as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"getline\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_getline_redir,
                class: 294 as i32,
                flags: 0x100 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"gsub\0" as *const u8 as *const i8,
                value: opcodeval::Op_sub_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 2 as i32 | (1 as i32) << 3 as i32)
                    as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"if\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_if,
                class: 275 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"in\0" as *const u8 as *const i8,
                value: opcodeval::Op_symbol,
                class: 296 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"include\0" as *const u8 as *const i8,
                value: opcodeval::Op_symbol,
                class: 304 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"index\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 2 as i32) as u32,
                ptr: Some(do_index as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"int\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 1 as i32) as u32,
                ptr: Some(do_int as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"isarray\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_isarray as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"length\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 302 as i32,
                flags: ((1 as i32) << 0 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_length as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"load\0" as *const u8 as *const i8,
                value: opcodeval::Op_symbol,
                class: 306 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"log\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 1 as i32) as u32,
                ptr: Some(do_log as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"lshift\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 2 as i32) as u32,
                ptr: Some(do_lshift as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"match\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 2 as i32 | (1 as i32) << 3 as i32)
                    as u32,
                ptr: Some(do_match as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"mkbool\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_mkbool as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"mktime\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
                    as u32,
                ptr: Some(do_mktime as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"namespace\0" as *const u8 as *const i8,
                value: opcodeval::Op_symbol,
                class: 307 as i32,
                flags: 0x400 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"next\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_next,
                class: 289 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"nextfile\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_nextfile,
                class: 295 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"or\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: 0x400 as i32 as u32,
                ptr: Some(do_or as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"patsplit\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 2 as i32 | (1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) as u32,
                ptr: Some(do_patsplit as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"print\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_print,
                class: 287 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"printf\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_printf,
                class: 288 as i32,
                flags: 0 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"rand\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 0 as i32) as u32,
                ptr: Some(do_rand as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"return\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_return,
                class: 277 as i32,
                flags: 0x100 as i32 as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"rshift\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 2 as i32) as u32,
                ptr: Some(do_rshift as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sin\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_sin as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"split\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 2 as i32 | (1 as i32) << 3 as i32
                    | (1 as i32) << 4 as i32) as u32,
                ptr: Some(do_split as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sprintf\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: 0 as i32 as u32,
                ptr: Some(do_sprintf as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sqrt\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 1 as i32) as u32,
                ptr: Some(do_sqrt as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"srand\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32)
                    as u32,
                ptr: Some(do_srand as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"strftime\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 0 as i32 | (1 as i32) << 1 as i32
                    | (1 as i32) << 2 as i32 | (1 as i32) << 3 as i32) as u32,
                ptr: Some(do_strftime as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"strtonum\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_strtonum as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"sub\0" as *const u8 as *const i8,
                value: opcodeval::Op_sub_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 2 as i32 | (1 as i32) << 3 as i32)
                    as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"substr\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: ((1 as i32) << 2 as i32 | (1 as i32) << 3 as i32) as u32,
                ptr: Some(do_substr as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"switch\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_switch,
                class: 279 as i32,
                flags: (0x400 as i32 | 0x800 as i32) as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"system\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_system as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"systime\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 0 as i32) as u32,
                ptr: Some(do_systime as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"tolower\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_tolower as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"toupper\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x100 as i32 | (1 as i32) << 1 as i32) as u32,
                ptr: Some(do_toupper as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"typeof\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: (0x400 as i32 | (1 as i32) << 1 as i32 | (1 as i32) << 2 as i32)
                    as u32,
                ptr: Some(do_typeof as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"while\0" as *const u8 as *const i8,
                value: opcodeval::Op_K_while,
                class: 282 as i32,
                flags: (0x800 as i32 | 0x1000 as i32) as u32,
                ptr: None,
                ptr2: None,
            };
            init
        },
        {
            let mut init = token {
                operator: b"xor\0" as *const u8 as *const i8,
                value: opcodeval::Op_builtin,
                class: 301 as i32,
                flags: 0x400 as i32 as u32,
                ptr: Some(do_xor as unsafe extern "C" fn(i32) -> *mut NODE),
                ptr2: None,
            };
            init
        },
    ]
};
static mut cur_mbstate: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed { __wch: 0 },
};
static mut cur_char_ring: [i8; 128] = [0; 128];
static mut cur_ring_idx: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn getfname(
    mut fptr: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
    mut prepend_awk: bool,
) -> *const i8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    static mut buf: [i8; 100] = [0; 100];
    j = (::core::mem::size_of::<[token; 71]>() as u64)
        .wrapping_div(::core::mem::size_of::<token>() as u64) as i32;
    i = 0 as i32;
    while i < j {
        if tokentab[i as usize].ptr == fptr || tokentab[i as usize].ptr2 == fptr {
            if prepend_awk as i32 != 0
                && tokentab[i as usize].flags & 0x400 as i32 as u32 != 0 as i32 as u32
            {
                sprintf(
                    buf.as_mut_ptr(),
                    b"awk::%s\0" as *const u8 as *const i8,
                    tokentab[i as usize].operator,
                );
                return buf.as_mut_ptr();
            }
            return tokentab[i as usize].operator;
        }
        i += 1;
        i;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn negate_num(mut n: *mut NODE) {
    add_sign_to_num(n, '-' as i32 as i8);
    if 0 as i32 == 0 {
        (*n).sub.val.fltnum = -(*n).sub.val.fltnum;
        return;
    }
}
unsafe extern "C" fn add_sign_to_num(mut n: *mut NODE, mut sign: i8) {
    if (*n).flags as u32 & flagvals::NUMCONSTSTR as i32 as u32 != 0 as i32 as u32 {
        let mut s: *mut i8 = 0 as *mut i8;
        s = (*n).sub.val.sp;
        memmove(
            &mut *s.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
            &mut *s.offset(0 as i32 as isize) as *mut i8 as *const libc::c_void,
            ((*n).sub.val.slen).wrapping_add(1 as i32 as u64),
        );
        *s.offset(0 as i32 as isize) = sign;
        (*n).sub.val.slen = ((*n).sub.val.slen).wrapping_add(1);
        (*n).sub.val.slen;
    }
}
unsafe extern "C" fn print_included_from() {
    let mut saveline: i32 = 0;
    let mut line: i32 = 0;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    saveline = sourceline;
    sourceline = 0 as i32;
    s = sourcefile;
    while !s.is_null() && (*s).stype as u32 == srctype::SRC_INC as i32 as u32 {
        s = (*s).next;
        if s.is_null() || (*s).fd <= -(1 as i32) {
            continue;
        }
        line = (*s).srclines;
        if (*s).lasttok == 308 as i32 {
            line -= 1;
            line;
        }
        msg(
            b"%s %s:%d%c\0" as *const u8 as *const i8,
            if (*s).prev == sourcefile {
                b"In file included from\0" as *const u8 as *const i8
            } else {
                b"                 from\0" as *const u8 as *const i8
            },
            if (*s).stype as u32 == srctype::SRC_INC as i32 as u32
                || (*s).stype as u32 == srctype::SRC_FILE as i32 as u32
            {
                (*s).src
            } else {
                b"cmd. line\0" as *const u8 as *const i8
            },
            line,
            if (*s).stype as u32 == srctype::SRC_INC as i32 as u32 {
                ',' as i32
            } else {
                ':' as i32
            },
        );
    }
    sourceline = saveline;
}
unsafe extern "C" fn warning_ln(mut line: i32, mut mesg: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut saveline: i32 = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    args_0 = args.clone();
    err(
        0 as i32 != 0,
        dcgettext(0 as *const i8, b"warning: \0" as *const u8 as *const i8, 5 as i32),
        mesg,
        args_0.as_va_list(),
    );
    sourceline = saveline;
}
unsafe extern "C" fn lintwarn_ln(mut line: i32, mut mesg: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut saveline: i32 = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    args_0 = args.clone();
    if lintfunc == Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()) {
        err(
            1 as i32 != 0,
            dcgettext(0 as *const i8, b"fatal: \0" as *const u8 as *const i8, 5 as i32),
            mesg,
            args_0.as_va_list(),
        );
    } else {
        err(
            0 as i32 != 0,
            dcgettext(
                0 as *const i8,
                b"warning: \0" as *const u8 as *const i8,
                5 as i32,
            ),
            mesg,
            args_0.as_va_list(),
        );
    }
    sourceline = saveline;
    if lintfunc == Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()) {
        gawk_exit(2 as i32);
    }
}
unsafe extern "C" fn error_ln(mut line: i32, mut m: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut saveline: i32 = 0;
    saveline = sourceline;
    sourceline = line;
    print_included_from();
    errcount += 1;
    errcount;
    args_0 = args.clone();
    err(0 as i32 != 0, b"error: \0" as *const u8 as *const i8, m, args_0.as_va_list());
    sourceline = saveline;
}
static mut syn_err_len: size_t = 0;
unsafe extern "C" fn yyerror(mut m: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut mesg: *const i8 = 0 as *const i8;
    let mut bp: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut scan: *mut i8 = 0 as *mut i8;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut count: i32 = 0;
    static mut end_of_file_line: [i8; 14] = unsafe {
        *::core::mem::transmute::<&[u8; 14], &mut [i8; 14]>(b"(defrule::END OF FILE)\0")
    };
    static mut syntax_error: [i8; 13] = unsafe {
        *::core::mem::transmute::<&[u8; 13], &mut [i8; 13]>(b"syntax error\0")
    };
    let mut generic_error: bool = strncmp(m, syntax_error.as_mut_ptr(), syn_err_len)
        == 0 as i32;
    print_included_from();
    errcount += 1;
    errcount;
    if !lexptr.is_null() && !lexeme.is_null() {
        if thisline.is_null() {
            cp = lexeme;
            if *cp as i32 == '\n' as i32 {
                if cp > lexptr_begin {
                    cp = cp.offset(-1);
                    cp;
                }
                mesg = dcgettext(
                    0 as *const i8,
                    b"unexpected newline or end of string\0" as *const u8 as *const i8,
                    5 as i32,
                );
            }
            while cp != lexptr_begin && *cp as i32 != '\n' as i32 {
                cp = cp.offset(-1);
                cp;
            }
            if *cp as i32 == '\n' as i32 {
                cp = cp.offset(1);
                cp;
            }
            thisline = cp;
        }
        bp = lexeme;
        if bp < thisline {
            bp = thisline.offset(1 as i32 as isize);
        }
        while bp < lexend && *bp as i32 != 0 && *bp as i32 != '\n' as i32 {
            bp = bp.offset(1);
            bp;
        }
    } else {
        thisline = end_of_file_line.as_mut_ptr();
        bp = thisline.offset(strlen(thisline) as isize);
    }
    if lexeof as i32 != 0 && mesg.is_null() && generic_error as i32 != 0 {
        msg(b"%s\0" as *const u8 as *const i8, end_of_file_line.as_mut_ptr());
        mesg = dcgettext(
            0 as *const i8,
            b"source files / command-line arguments must contain complete functions or rules\0"
                as *const u8 as *const i8,
            5 as i32,
        );
    } else {
        msg(
            b"%.*s\0" as *const u8 as *const i8,
            bp.offset_from(thisline) as i64 as i32,
            thisline,
        );
    }
    args_0 = args.clone();
    if mesg.is_null() {
        mesg = m;
    }
    count = (strlen(mesg)).wrapping_add(1 as i32 as u64) as i32;
    if !lexptr.is_null() {
        count = (count as i64 + (lexeme.offset_from(thisline) as i64 + 2 as i32 as i64))
            as i32;
    }
    buf = ezalloc_real(
        (count + 1 as i32) as size_t,
        b"yyerror\0" as *const u8 as *const i8,
        b"buf\0" as *const u8 as *const i8,
        b"awkgram.y\0" as *const u8 as *const i8,
        2608 as i32,
    ) as *mut i8;
    bp = buf;
    if !lexptr.is_null() {
        scan = thisline;
        while scan < lexeme {
            let fresh83 = scan;
            scan = scan.offset(1);
            if *fresh83 as i32 == '\t' as i32 {
                let fresh84 = bp;
                bp = bp.offset(1);
                *fresh84 = '\t' as i32 as i8;
            } else {
                let fresh85 = bp;
                bp = bp.offset(1);
                *fresh85 = ' ' as i32 as i8;
            }
        }
        let fresh86 = bp;
        bp = bp.offset(1);
        *fresh86 = '^' as i32 as i8;
        let fresh87 = bp;
        bp = bp.offset(1);
        *fresh87 = ' ' as i32 as i8;
    }
    strcpy(bp, mesg);
    err(0 as i32 != 0, b"\0" as *const u8 as *const i8, buf, args_0.as_va_list());
    pma_free(buf as *mut libc::c_void);
    exit(1 as i32);
}
unsafe extern "C" fn mk_program() -> *mut INSTRUCTION {
    let mut current_block: u64;
    let mut cp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tmp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if (rule_block[defrule::END as i32 as usize]).is_null() {
        rule_block[defrule::END as i32 as usize] = list_create(ip_end);
    } else {
        list_prepend(rule_block[defrule::END as i32 as usize], ip_end);
    }
    if in_main_context() == 0 {
        if !(rule_block[defrule::BEGIN as i32 as usize]).is_null()
            && !(rule_block[defrule::Rule as i32 as usize]).is_null()
        {
            cp = list_merge(
                rule_block[defrule::BEGIN as i32 as usize],
                rule_block[defrule::Rule as i32 as usize],
            );
        } else {
            cp = if !(rule_block[defrule::BEGIN as i32 as usize]).is_null() {
                rule_block[defrule::BEGIN as i32 as usize]
            } else {
                rule_block[defrule::Rule as i32 as usize]
            };
        }
        if !cp.is_null() {
            list_merge(cp, rule_block[defrule::END as i32 as usize]);
        } else {
            cp = rule_block[defrule::END as i32 as usize];
        }
        list_append(cp, bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32));
    } else {
        if (rule_block[defrule::ENDFILE as i32 as usize]).is_null() {
            rule_block[defrule::ENDFILE as i32 as usize] = list_create(ip_endfile);
        } else {
            (*ip_rec).x.xl = 1 as i32 as i64;
            list_prepend(rule_block[defrule::ENDFILE as i32 as usize], ip_endfile);
        }
        if (rule_block[defrule::BEGINFILE as i32 as usize]).is_null() {
            rule_block[defrule::BEGINFILE as i32 as usize] = list_create(ip_beginfile);
        } else {
            list_prepend(rule_block[defrule::BEGINFILE as i32 as usize], ip_beginfile);
        }
        if (rule_block[defrule::Rule as i32 as usize]).is_null() {
            if (*rule_block[defrule::END as i32 as usize]).nexti
                == (*rule_block[defrule::END as i32 as usize]).d.di
                && (*rule_block[defrule::BEGINFILE as i32 as usize]).nexti
                    == (*rule_block[defrule::BEGINFILE as i32 as usize]).d.di
                && (*rule_block[defrule::ENDFILE as i32 as usize]).nexti
                    == (*rule_block[defrule::ENDFILE as i32 as usize]).d.di
            {
                bcfree(ip_rec);
                bcfree(ip_newfile);
                ip_newfile = 0 as *mut INSTRUCTION;
                ip_rec = ip_newfile;
                list_append(
                    rule_block[defrule::BEGINFILE as i32 as usize],
                    bcalloc(opcodeval::Op_after_beginfile, 1 as i32, 0 as i32),
                );
                list_append(
                    rule_block[defrule::ENDFILE as i32 as usize],
                    bcalloc(opcodeval::Op_after_endfile, 1 as i32, 0 as i32),
                );
                if (rule_block[defrule::BEGIN as i32 as usize]).is_null() {
                    cp = rule_block[defrule::END as i32 as usize];
                } else {
                    cp = list_merge(
                        rule_block[defrule::BEGIN as i32 as usize],
                        rule_block[defrule::END as i32 as usize],
                    );
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                list_append(cp, ip_atexit);
                list_append(cp, bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32));
                list_merge(cp, rule_block[defrule::BEGINFILE as i32 as usize]);
                list_merge(cp, rule_block[defrule::ENDFILE as i32 as usize]);
                if !outer_comment.is_null() {
                    cp = list_merge(list_create(outer_comment), cp);
                    outer_comment = 0 as *mut INSTRUCTION;
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                current_block = 8030954807531722260;
            } else {
                rule_block[defrule::Rule as i32 as usize] = list_create(
                    bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32),
                );
                current_block = 11048769245176032998;
            }
        } else {
            current_block = 11048769245176032998;
        }
        match current_block {
            8030954807531722260 => {}
            _ => {
                list_append(
                    rule_block[defrule::ENDFILE as i32 as usize],
                    bcalloc(opcodeval::Op_after_endfile, 1 as i32, 0 as i32),
                );
                list_prepend(rule_block[defrule::Rule as i32 as usize], ip_rec);
                list_append(
                    rule_block[defrule::Rule as i32 as usize],
                    bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32),
                );
                (*(*rule_block[defrule::Rule as i32 as usize]).d.di).d.di = ip_rec;
                list_append(
                    rule_block[defrule::BEGINFILE as i32 as usize],
                    bcalloc(opcodeval::Op_after_beginfile, 1 as i32, 0 as i32),
                );
                cp = list_merge(
                    rule_block[defrule::BEGINFILE as i32 as usize],
                    rule_block[defrule::Rule as i32 as usize],
                );
                list_prepend(cp, ip_newfile);
                list_merge(cp, rule_block[defrule::ENDFILE as i32 as usize]);
                list_merge(cp, rule_block[defrule::END as i32 as usize]);
                if !(rule_block[defrule::BEGIN as i32 as usize]).is_null() {
                    cp = list_merge(rule_block[defrule::BEGIN as i32 as usize], cp);
                }
                if !outer_comment.is_null() {
                    cp = list_merge(list_create(outer_comment), cp);
                    outer_comment = 0 as *mut INSTRUCTION;
                }
                if !interblock_comment.is_null() {
                    list_append(cp, interblock_comment);
                    interblock_comment = 0 as *mut INSTRUCTION;
                }
                list_append(cp, ip_atexit);
                list_append(cp, bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32));
            }
        }
    }
    tmp = (*cp).nexti;
    bcfree(cp);
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn parse_program(
    mut pcode: *mut *mut INSTRUCTION,
    mut from_eval: bool,
) -> i32 {
    let mut ret: i32 = 0;
    called_from_eval = from_eval;
    ip_end = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
    if in_main_context() == 0 {
        ip_endfile = 0 as *mut INSTRUCTION;
        ip_beginfile = ip_endfile;
        ip_atexit = ip_beginfile;
        ip_rec = ip_atexit;
        ip_newfile = ip_rec;
    } else {
        ip_endfile = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
        ip_beginfile = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
        main_beginfile = ip_beginfile;
        ip_rec = bcalloc(opcodeval::Op_get_record, 1 as i32, 0 as i32);
        ip_newfile = bcalloc(opcodeval::Op_newfile, 2 as i32, 0 as i32);
        (*ip_newfile).d.di = ip_end;
        (*ip_newfile).x.xi = ip_endfile;
        let ref mut fresh88 = (*ip_newfile.offset(1 as i32 as isize)).x.xi;
        *fresh88 = ip_rec;
        (*ip_rec).d.di = ip_newfile;
        ip_atexit = bcalloc(opcodeval::Op_atexit, 1 as i32, 0 as i32);
    }
    sourcefile = (*srcfiles).next;
    while (*sourcefile).stype as u32 == srctype::SRC_EXTLIB as i32 as u32 {
        sourcefile = (*sourcefile).next;
    }
    lexeof = 0 as i32 != 0;
    lexptr = 0 as *mut i8;
    lasttok = 0 as i32;
    memset(
        rule_block.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[*mut INSTRUCTION; 6]>() as u64,
    );
    errcount = 0 as i32;
    tok = if !tokstart.is_null() { tokstart } else { tokexpand() };
    ret = yyparse();
    *pcode = mk_program();
    source = 0 as *mut i8;
    sourceline = 0 as i32;
    if ret == 0 as i32 {
        check_funcs();
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
        && !check_param_names()
    {
        errcount += 1;
        errcount;
    }
    if args_array.is_null() {
        args_array = emalloc_real(
            ((max_args + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"parse_program\0" as *const u8 as *const i8,
            b"args_array\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            2819 as i32,
        ) as *mut *mut NODE;
    } else {
        args_array = erealloc_real(
            args_array as *mut libc::c_void,
            ((max_args + 2 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            b"parse_program\0" as *const u8 as *const i8,
            b"args_array\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            2821 as i32,
        ) as *mut *mut NODE;
    }
    return (ret != 0 || errcount != 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn free_srcfile(mut thisfile: *mut SRCFILE) {
    pma_free((*thisfile).src as *mut libc::c_void);
    pma_free(thisfile as *mut libc::c_void);
}
unsafe extern "C" fn do_add_srcfile(
    mut stype: srctype,
    mut src: *mut i8,
    mut path: *mut i8,
    mut thisfile: *mut SRCFILE,
) -> *mut SRCFILE {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    s = ezalloc_real(
        ::core::mem::size_of::<SRCFILE>() as u64,
        b"do_add_srcfile\0" as *const u8 as *const i8,
        b"s\0" as *const u8 as *const i8,
        b"awkgram.y\0" as *const u8 as *const i8,
        2842 as i32,
    ) as *mut SRCFILE;
    (*s).src = estrdup(src, strlen(src));
    (*s).fullpath = path;
    (*s).stype = stype;
    (*s).fd = -(1 as i32);
    (*s).next = thisfile;
    (*s).prev = (*thisfile).prev;
    (*(*thisfile).prev).next = s;
    (*thisfile).prev = s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn add_srcfile(
    mut stype: srctype,
    mut src: *mut i8,
    mut thisfile: *mut SRCFILE,
    mut already_included: *mut bool,
    mut errcode: *mut i32,
) -> *mut SRCFILE {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
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
    let mut path: *mut i8 = 0 as *mut i8;
    let mut errno_val: i32 = 0 as i32;
    if !already_included.is_null() {
        *already_included = 0 as i32 != 0;
    }
    if !errcode.is_null() {
        *errcode = 0 as i32;
    }
    if stype as u32 == srctype::SRC_CMDLINE as i32 as u32
        || stype as u32 == srctype::SRC_STDIN as i32 as u32
    {
        return do_add_srcfile(stype, src, 0 as *mut i8, thisfile);
    }
    path = find_source(
        src,
        &mut sbuf,
        &mut errno_val,
        (stype as u32 == srctype::SRC_EXTLIB as i32 as u32) as i32,
    );
    if path.is_null() {
        if !errcode.is_null() {
            *errcode = errno_val;
            return 0 as *mut SRCFILE;
        }
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 2880 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            if stype as u32 != srctype::SRC_EXTLIB as i32 as u32 {
                dcgettext(
                    0 as *const i8,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"cannot open shared library `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                )
            },
            src,
            if errno_val != 0 {
                strerror(errno_val)
            } else {
                dcgettext(
                    0 as *const i8,
                    b"reason unknown\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if ((*s).stype as u32 == srctype::SRC_FILE as i32 as u32
            || (*s).stype as u32 == srctype::SRC_INC as i32 as u32
            || (*s).stype as u32 == srctype::SRC_EXTLIB as i32 as u32)
            && files_are_same(path, s) != 0
        {
            if stype as u32 == srctype::SRC_INC as i32 as u32
                || stype as u32 == srctype::SRC_EXTLIB as i32 as u32
            {
                if stype as u32 == srctype::SRC_INC as i32 as u32
                    && (*s).stype as u32 == srctype::SRC_FILE as i32 as u32
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 2893 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot include `%s' and use it as a program file\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        src,
                    );
                }
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    let mut line: i32 = sourceline;
                    if sourceline > 1 as i32 && lasttok == 308 as i32 {
                        line -= 1;
                        line;
                    }
                    lintwarn_ln(
                        line,
                        if stype as u32 != srctype::SRC_EXTLIB as i32 as u32 {
                            dcgettext(
                                0 as *const i8,
                                b"already included source file `%s'\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            )
                        } else {
                            dcgettext(
                                0 as *const i8,
                                b"already loaded shared library `%s'\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            )
                        },
                        src,
                    );
                }
                pma_free(path as *mut libc::c_void);
                if !already_included.is_null() {
                    *already_included = 1 as i32 != 0;
                }
                return 0 as *mut SRCFILE;
            } else {
                if (*s).stype as u32 == srctype::SRC_INC as i32 as u32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 2917 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"cannot include `%s' and use it as a program file\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        src,
                    );
                }
                break;
            }
        } else {
            s = (*s).next;
        }
    }
    s = do_add_srcfile(stype, src, path, thisfile);
    (*s).sbuf = sbuf;
    (*s).mtime = sbuf.st_mtim.tv_sec;
    return s;
}
unsafe extern "C" fn include_source(
    mut file: *mut INSTRUCTION,
    mut srcfile_p: *mut *mut libc::c_void,
) -> bool {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut src: *mut i8 = (*file).d.name;
    let mut errcode: i32 = 0;
    let mut already_included: bool = false;
    *srcfile_p = 0 as *mut libc::c_void;
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
    {
        error_ln(
            (*file).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"@include is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32 != 0;
    }
    if strlen(src) == 0 as i32 as u64 {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            lintwarn_ln(
                (*file).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"empty filename after @include\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        return 1 as i32 != 0;
    }
    s = add_srcfile(
        srctype::SRC_INC,
        src,
        sourcefile,
        &mut already_included,
        &mut errcode,
    );
    if s.is_null() {
        if already_included {
            return 1 as i32 != 0;
        }
        error_ln(
            (*file).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            src,
            if errcode != 0 {
                strerror(errcode)
            } else {
                dcgettext(
                    0 as *const i8,
                    b"reason unknown\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
        return 0 as i32 != 0;
    }
    (*sourcefile).srclines = sourceline;
    (*sourcefile).lexptr = lexptr;
    (*sourcefile).lexend = lexend;
    (*sourcefile).lexptr_begin = lexptr_begin;
    (*sourcefile).lexeme = lexeme;
    (*sourcefile).lasttok = lasttok;
    (*sourcefile).namespace = current_namespace;
    sourcefile = s;
    lexptr = 0 as *mut i8;
    sourceline = 0 as i32;
    source = 0 as *mut i8;
    lasttok = 0 as i32;
    lexeof = 0 as i32 != 0;
    eof_warned = 0 as i32 != 0;
    current_namespace = awk_namespace.as_ptr();
    *srcfile_p = s as *mut libc::c_void;
    return 1 as i32 != 0;
}
unsafe extern "C" fn load_library(
    mut file: *mut INSTRUCTION,
    mut srcfile_p: *mut *mut libc::c_void,
) -> bool {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut src: *mut i8 = (*file).d.name;
    let mut errcode: i32 = 0;
    let mut already_included: bool = false;
    *srcfile_p = 0 as *mut libc::c_void;
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
    {
        error_ln(
            (*file).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"@load is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32 != 0;
    }
    if strlen(src) == 0 as i32 as u64 {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            lintwarn_ln(
                (*file).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"empty filename after @load\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        return 1 as i32 != 0;
    }
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
        && do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 == 0
    {
        s = do_add_srcfile(srctype::SRC_EXTLIB, src, src, sourcefile);
    } else {
        s = add_srcfile(
            srctype::SRC_EXTLIB,
            src,
            sourcefile,
            &mut already_included,
            &mut errcode,
        );
        if s.is_null() {
            if already_included {
                return 1 as i32 != 0;
            }
            error_ln(
                (*file).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"cannot open shared library `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                src,
                if errcode != 0 {
                    strerror(errcode)
                } else {
                    dcgettext(
                        0 as *const i8,
                        b"reason unknown\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                },
            );
            return 0 as i32 != 0;
        }
        load_ext((*s).fullpath);
    }
    *srcfile_p = s as *mut libc::c_void;
    return 1 as i32 != 0;
}
unsafe extern "C" fn next_sourcefile() {
    static mut closefunc: Option<unsafe extern "C" fn(i32) -> i32> = None;
    if closefunc.is_none() {
        let mut cp: *mut i8 = getenv(b"AWKREADFUNC\0" as *const u8 as *const i8);
        if cp.is_null() {
            closefunc = Some(close as unsafe extern "C" fn(i32) -> i32);
        } else {
            closefunc = Some(one_line_close as unsafe extern "C" fn(i32) -> i32);
        }
    }
    lexeof = 0 as i32 != 0;
    eof_warned = 0 as i32 != 0;
    (*sourcefile).srclines = sourceline;
    if (*sourcefile).fd > -(1 as i32) {
        if (*sourcefile).fd != fileno(stdin) {
            (Some(closefunc.expect("non-null function pointer")))
                .expect("non-null function pointer")((*sourcefile).fd);
        }
        (*sourcefile).fd = -(1 as i32);
    }
    if !((*sourcefile).buf).is_null() {
        pma_free((*sourcefile).buf as *mut libc::c_void);
        (*sourcefile).buf = 0 as *mut i8;
        (*sourcefile).lexptr_begin = 0 as *mut i8;
    }
    loop {
        sourcefile = (*sourcefile).next;
        if sourcefile.is_null() {
            break;
        }
        if sourcefile == srcfiles {
            return;
        }
        if (*sourcefile).stype as u32 != srctype::SRC_EXTLIB as i32 as u32 {
            break;
        }
    }
    if !((*sourcefile).lexptr_begin).is_null() {
        lexptr = (*sourcefile).lexptr;
        lexend = (*sourcefile).lexend;
        lasttok = (*sourcefile).lasttok;
        lexptr_begin = (*sourcefile).lexptr_begin;
        lexeme = (*sourcefile).lexeme;
        sourceline = (*sourcefile).srclines;
        source = (*sourcefile).src;
        set_current_namespace((*sourcefile).namespace);
    } else {
        lexptr = 0 as *mut i8;
        sourceline = 0 as i32;
        source = 0 as *mut i8;
        lasttok = 0 as i32;
        set_current_namespace(awk_namespace.as_ptr());
    };
}
unsafe extern "C" fn get_src_buf() -> *mut i8 {
    let mut n: i32 = 0;
    let mut scan: *mut i8 = 0 as *mut i8;
    let mut newfile: bool = false;
    let mut savelen: i32 = 0;
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
    static mut readfunc: Option<
        unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
    > = None;
    if readfunc.is_none() {
        let mut cp: *mut i8 = getenv(b"AWKREADFUNC\0" as *const u8 as *const i8);
        if cp.is_null() {
            readfunc = ::core::mem::transmute::<
                Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
                Option<unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t>,
            >(
                Some(
                    read
                        as unsafe extern "C" fn(
                            i32,
                            *mut libc::c_void,
                            size_t,
                        ) -> ssize_t,
                ),
            );
        } else {
            readfunc = Some(
                read_one_line
                    as unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
            );
        }
    }
    newfile = 0 as i32 != 0;
    if sourcefile == srcfiles {
        return 0 as *mut i8;
    }
    if (*sourcefile).stype as u32 == srctype::SRC_CMDLINE as i32 as u32 {
        if (*sourcefile).bufsize == 0 as i32 as u64 {
            (*sourcefile).bufsize = strlen((*sourcefile).src);
            lexeme = (*sourcefile).src;
            lexptr_begin = lexeme;
            lexptr = lexptr_begin;
            lexend = lexptr.offset((*sourcefile).bufsize as isize);
            sourceline = 1 as i32;
            if (*sourcefile).bufsize == 0 as i32 as u64 {
                static mut warned: bool = 0 as i32 != 0;
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0 && !warned
                {
                    warned = 1 as i32 != 0;
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 3149 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"empty program text on command line\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                lexeof = 1 as i32 != 0;
            }
        } else if ((*sourcefile).buf).is_null()
            && *lexptr.offset(-(1 as i32 as isize)) as i32 != '\n' as i32
        {
            let mut offset: i32 = 0;
            let mut buf: *mut i8 = 0 as *mut i8;
            offset = lexptr.offset_from(lexeme) as i64 as i32;
            scan = lexeme;
            while scan > lexptr_begin {
                if *scan as i32 == '\n' as i32 {
                    scan = scan.offset(1);
                    scan;
                    break;
                } else {
                    scan = scan.offset(-1);
                    scan;
                }
            }
            savelen = lexptr.offset_from(scan) as i64 as i32;
            buf = emalloc_real(
                (savelen + 1 as i32) as size_t,
                b"get_src_buf\0" as *const u8 as *const i8,
                b"buf\0" as *const u8 as *const i8,
                b"awkgram.y\0" as *const u8 as *const i8,
                3169 as i32,
            ) as *mut i8;
            memcpy(
                buf as *mut libc::c_void,
                scan as *const libc::c_void,
                savelen as u64,
            );
            thisline = buf;
            lexptr = buf.offset(savelen as isize);
            *lexptr = '\n' as i32 as i8;
            lexeme = lexptr.offset(-(offset as isize));
            lexptr_begin = buf;
            lexend = lexptr.offset(1 as i32 as isize);
            (*sourcefile).buf = buf;
        } else {
            lexeof = 1 as i32 != 0;
        }
        return lexptr;
    }
    if (*sourcefile).fd <= -(1 as i32) {
        let mut fd: i32 = 0;
        let mut l: i32 = 0;
        source = (*sourcefile).src;
        if source.is_null() {
            return 0 as *mut i8;
        }
        fd = srcopen(sourcefile);
        if fd <= -(1 as i32) {
            let mut in_0: *mut i8 = 0 as *mut i8;
            in_0 = source;
            source = 0 as *mut i8;
            error(
                dcgettext(
                    0 as *const i8,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                in_0,
                strerror(*__errno_location()),
            );
            errcount += 1;
            errcount;
            lexeof = 1 as i32 != 0;
            return (*sourcefile).src;
        }
        (*sourcefile).fd = fd;
        l = optimal_bufsize(fd, &mut sbuf) as i32;
        if l < 128 as i32 {
            l = 128 as i32;
        }
        (*sourcefile).bufsize = l as size_t;
        newfile = 1 as i32 != 0;
        (*sourcefile).buf = emalloc_real(
            (*sourcefile).bufsize,
            b"get_src_buf\0" as *const u8 as *const i8,
            b"sourcefile->buf\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            3217 as i32,
        ) as *mut i8;
        memset(
            (*sourcefile).buf as *mut libc::c_void,
            '\0' as i32,
            (*sourcefile).bufsize,
        );
        lexeme = (*sourcefile).buf;
        lexptr_begin = lexeme;
        lexptr = lexptr_begin;
        savelen = 0 as i32;
        sourceline = 1 as i32;
        thisline = 0 as *mut i8;
    } else {
        let mut offset_0: i32 = 0;
        scan = lexeme;
        while scan > lexptr_begin {
            if *scan as i32 == '\n' as i32 {
                scan = scan.offset(1);
                scan;
                break;
            } else {
                scan = scan.offset(-1);
                scan;
            }
        }
        savelen = lexptr.offset_from(scan) as i64 as i32;
        offset_0 = lexptr.offset_from(lexeme) as i64 as i32;
        if savelen > 0 as i32 {
            if savelen as u64 > ((*sourcefile).bufsize).wrapping_div(2 as i32 as u64) {
                (*sourcefile).bufsize = ((*sourcefile).bufsize as u64)
                    .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                (*sourcefile).buf = erealloc_real(
                    (*sourcefile).buf as *mut libc::c_void,
                    (*sourcefile).bufsize,
                    b"get_src_buf\0" as *const u8 as *const i8,
                    b"sourcefile->buf\0" as *const u8 as *const i8,
                    b"awkgram.y\0" as *const u8 as *const i8,
                    3247 as i32,
                ) as *mut i8;
                scan = ((*sourcefile).buf)
                    .offset(scan.offset_from(lexptr_begin) as i64 as isize);
                lexptr_begin = (*sourcefile).buf;
            }
            thisline = lexptr_begin;
            memmove(
                thisline as *mut libc::c_void,
                scan as *const libc::c_void,
                savelen as u64,
            );
            lexptr = thisline.offset(savelen as isize);
            lexeme = lexptr.offset(-(offset_0 as isize));
        } else {
            savelen = 0 as i32;
            lexeme = lexptr_begin;
            lexptr = lexeme;
            thisline = 0 as *mut i8;
        }
    }
    n = (Some(readfunc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (*sourcefile).fd,
        lexptr as *mut libc::c_void,
        ((*sourcefile).bufsize).wrapping_sub(savelen as u64),
    ) as i32;
    if n == -(1 as i32) {
        error(
            dcgettext(
                0 as *const i8,
                b"cannot read source file `%s': %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            source,
            strerror(*__errno_location()),
        );
        errcount += 1;
        errcount;
        lexeof = 1 as i32 != 0;
    } else {
        lexend = lexptr.offset(n as isize);
        if n == 0 as i32 {
            static mut warned_0: bool = 0 as i32 != 0;
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                && newfile as i32 != 0 && !warned_0
            {
                warned_0 = 1 as i32 != 0;
                sourceline = 0 as i32;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 3276 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"source file `%s' is empty\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    source,
                );
            }
            lexeof = 1 as i32 != 0;
        }
    }
    return (*sourcefile).buf;
}
unsafe extern "C" fn tokexpand() -> *mut i8 {
    static mut toksize: size_t = 0;
    let mut tokoffset: i32 = 0;
    if !tokstart.is_null() {
        tokoffset = tok.offset_from(tokstart) as i64 as i32;
        toksize = (toksize as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
        tokstart = erealloc_real(
            tokstart as *mut libc::c_void,
            toksize,
            b"tokexpand\0" as *const u8 as *const i8,
            b"tokstart\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            3299 as i32,
        ) as *mut i8;
        tok = tokstart.offset(tokoffset as isize);
    } else {
        toksize = 60 as i32 as size_t;
        tokstart = emalloc_real(
            toksize,
            b"tokexpand\0" as *const u8 as *const i8,
            b"tokstart\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            3303 as i32,
        ) as *mut i8;
        tok = tokstart;
    }
    tokend = tokstart.offset(toksize as isize);
    return tok;
}
unsafe extern "C" fn check_bad_char(mut c: i32) {
    match c {
        7 | 8 | 12 | 10 | 13 | 9 => return,
        _ => {}
    }
    if *(*__ctype_b_loc()).offset(c as isize) as i32
        & C2RustUnnamed_0::_IScntrl as i32 as libc::c_ushort as i32 != 0
        && *(*__ctype_b_loc()).offset(c as isize) as i32
            & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 == 0
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 3336 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"error: invalid character '\\%03o' in source code\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            c & 0xff as i32,
        );
    }
}
unsafe extern "C" fn nextc(mut check_for_bad: bool) -> i32 {
    if gawk_mb_cur_max > 1 as i32 {
        loop {
            if lexeof {
                return -(1000 as i32);
            }
            if lexptr.is_null() || lexptr >= lexend {
                if !(get_src_buf()).is_null() {
                    continue;
                }
                return -(2000 as i32);
            } else {
                cur_ring_idx = if cur_ring_idx == 8 as i32 * 16 as i32 - 1 as i32 {
                    0 as i32
                } else {
                    cur_ring_idx + 1 as i32
                };
                if cur_char_ring[cur_ring_idx as usize] as i32 == 0 as i32 {
                    let mut idx: i32 = 0;
                    let mut work_ring_idx: i32 = cur_ring_idx;
                    let mut tmp_state: mbstate_t = mbstate_t {
                        __count: 0,
                        __value: C2RustUnnamed { __wch: 0 },
                    };
                    let mut mbclen: size_t = 0;
                    idx = 0 as i32;
                    while lexptr.offset(idx as isize) < lexend {
                        tmp_state = cur_mbstate;
                        mbclen = mbrlen(
                            lexptr,
                            (idx + 1 as i32) as size_t,
                            &mut tmp_state,
                        );
                        if mbclen == 1 as i32 as u64 || mbclen == -(1 as i32) as size_t
                            || mbclen == 0 as i32 as u64
                        {
                            cur_char_ring[work_ring_idx as usize] = 1 as i32 as i8;
                            break;
                        } else if mbclen == -(2 as i32) as size_t {
                            cur_char_ring[work_ring_idx as usize] = (idx + 1 as i32)
                                as i8;
                            work_ring_idx = if work_ring_idx
                                == 8 as i32 * 16 as i32 - 1 as i32
                            {
                                0 as i32
                            } else {
                                work_ring_idx + 1 as i32
                            };
                            idx += 1;
                            idx;
                        } else {
                            cur_char_ring[work_ring_idx as usize] = mbclen as i8;
                            break;
                        }
                    }
                    cur_mbstate = tmp_state;
                    work_ring_idx = if work_ring_idx == 8 as i32 * 16 as i32 - 1 as i32 {
                        0 as i32
                    } else {
                        work_ring_idx + 1 as i32
                    };
                    cur_char_ring[work_ring_idx as usize] = 0 as i32 as i8;
                }
                if check_for_bad as i32 != 0 || *lexptr as i32 == '\0' as i32 {
                    check_bad_char(*lexptr as i32);
                }
                let fresh89 = lexptr;
                lexptr = lexptr.offset(1);
                return *fresh89 as u8 as i32;
            }
        }
    } else {
        loop {
            if lexeof {
                return -(1000 as i32);
            }
            if !lexptr.is_null() && lexptr < lexend {
                if check_for_bad as i32 != 0 || *lexptr as i32 == '\0' as i32 {
                    check_bad_char(*lexptr as i32);
                }
                let fresh90 = lexptr;
                lexptr = lexptr.offset(1);
                return *fresh90 as u8 as i32;
            }
            if (get_src_buf()).is_null() {
                break;
            }
        }
        return -(2000 as i32);
    };
}
#[inline]
unsafe extern "C" fn pushback() {
    if gawk_mb_cur_max > 1 as i32 {
        cur_ring_idx = if cur_ring_idx == 0 as i32 {
            8 as i32 * 16 as i32 - 1 as i32
        } else {
            cur_ring_idx - 1 as i32
        };
    }
    if !lexeof && !lexptr.is_null() && lexptr > lexptr_begin {
        lexptr = lexptr.offset(-1);
        lexptr;
    } else {};
}
unsafe extern "C" fn get_comment(
    mut flag: commenttype,
    mut comment_instruction: *mut *mut INSTRUCTION,
) -> i32 {
    let mut c: i32 = 0;
    let mut sl: i32 = 0;
    let mut p1: *mut i8 = 0 as *mut i8;
    let mut p2: *mut i8 = 0 as *mut i8;
    tok = tokstart;
    let fresh91 = tok;
    tok = tok.offset(1);
    *fresh91 = '#' as i32 as i8;
    if tok == tokend {
        tokexpand();
    } else {};
    sl = sourceline;
    loop {
        loop {
            c = nextc(0 as i32 != 0);
            if !(c != '\n' as i32 && c != -(1000 as i32)) {
                break;
            }
            if c != '\r' as i32 {
                let fresh92 = tok;
                tok = tok.offset(1);
                *fresh92 = c as i8;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
        if flag as u32 == commenttype::EOL_COMMENT as i32 as u32 {
            if c == '\n' as i32 {
                let fresh93 = tok;
                tok = tok.offset(1);
                *fresh93 = c as i8;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
            break;
        } else {
            if !(c == '\n' as i32) {
                break;
            }
            let fresh94 = tok;
            tok = tok.offset(1);
            *fresh94 = c as i8;
            if tok == tokend {
                tokexpand();
            } else {};
            sourceline += 1;
            sourceline;
            loop {
                c = nextc(0 as i32 != 0);
                if c == '\n' as i32 {
                    sourceline += 1;
                    sourceline;
                    let fresh95 = tok;
                    tok = tok.offset(1);
                    *fresh95 = c as i8;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                }
                if !(c != -(1000 as i32)
                    && *(*__ctype_b_loc()).offset(c as isize) as i32
                        & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 != 0)
                {
                    break;
                }
            }
            if c == -(1000 as i32) {
                break;
            }
            if c != '#' as i32 {
                pushback();
                sourceline -= 1;
                sourceline;
                break;
            } else {
                let fresh96 = tok;
                tok = tok.offset(1);
                *fresh96 = c as i8;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
    }
    p1 = tok.offset(-(1 as i32 as isize));
    p2 = tok.offset(-(2 as i32 as isize));
    while *p1 as i32 == '\n' as i32 && *p2 as i32 == '\n' as i32 {
        p1 = p1.offset(-1);
        p1;
        p2 = p2.offset(-1);
        p2;
        tok = tok.offset(-1);
        tok;
    }
    *comment_instruction = bcalloc(opcodeval::Op_comment, 1 as i32, sl);
    (**comment_instruction).d.name = source;
    (**comment_instruction).d.dn = make_str_node(
        tokstart,
        tok.offset_from(tokstart) as i64 as size_t,
        0 as i32,
    );
    (*(**comment_instruction).d.dn).sub.val.comtype = flag;
    return c;
}
unsafe extern "C" fn allow_newline(mut new_comment: *mut *mut INSTRUCTION) {
    let mut c: i32 = 0;
    loop {
        c = nextc(1 as i32 != 0);
        if c == -(1000 as i32) {
            pushback();
            break;
        } else {
            if c == '#' as i32 {
                if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                    && do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 == 0
                {
                    c = get_comment(commenttype::EOL_COMMENT, new_comment);
                } else {
                    loop {
                        c = nextc(0 as i32 != 0);
                        if !(c != '\n' as i32 && c != -(1000 as i32)) {
                            break;
                        }
                    }
                }
                if c == -(1000 as i32) {
                    pushback();
                    break;
                }
            }
            if c == '\n' as i32 {
                sourceline += 1;
                sourceline;
            }
            if !(*(*__ctype_b_loc()).offset(c as isize) as i32
                & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 == 0)
            {
                continue;
            }
            pushback();
            break;
        }
    };
}
unsafe extern "C" fn newline_eof() -> i32 {
    if lasttok != 308 as i32 {
        pushback();
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0 && !eof_warned
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 3563 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"source file does not end in newline\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            eof_warned = 1 as i32 != 0;
        }
        sourceline += 1;
        sourceline;
        return 308 as i32;
    }
    sourceline -= 1;
    sourceline;
    eof_warned = 0 as i32 != 0;
    return 303 as i32;
}
unsafe extern "C" fn yylex() -> i32 {
    let mut in_brack: i32 = 0;
    let mut b_index: i32 = 0;
    let mut cur_index: i32 = 0;
    let mut current_block: u64;
    let mut c: i32 = 0;
    let mut seen_e: bool = 0 as i32 != 0;
    let mut seen_point: bool = 0 as i32 != 0;
    let mut esc_seen: bool = false;
    let mut mid: i32 = 0;
    let mut base: i32 = 0;
    static mut did_newline: bool = 0 as i32 != 0;
    let mut tokkey: *mut i8 = 0 as *mut i8;
    let mut inhex: bool = 0 as i32 != 0;
    let mut intlstr: bool = 0 as i32 != 0;
    let mut d: libc::c_double = 0.;
    let mut collecting_typed_regexp: bool = 0 as i32 != 0;
    static mut qm_col_count: i32 = 0 as i32;
    yylval = 0 as *mut libc::c_void as *mut INSTRUCTION;
    if lasttok == 272 as i32 {
        lasttok = 0 as i32;
        return 272 as i32;
    }
    if lasttok == 303 as i32 {
        return 0 as i32;
    }
    c = nextc(!want_regexp);
    if c == -(2000 as i32) {
        return 0 as i32;
    }
    if c == -(1000 as i32) {
        lasttok = newline_eof();
        return lasttok;
    }
    pushback();
    lexeme = lexptr;
    thisline = 0 as *mut i8;
    '_collect_regexp: loop {
        if want_regexp {
            in_brack = 0 as i32;
            b_index = -(1 as i32);
            cur_index = 0 as i32;
            want_regexp = 0 as i32 != 0;
            tok = tokstart;
            current_block = 2370887241019905314;
            break;
        } else {
            loop {
                loop {
                    c = nextc(1 as i32 != 0);
                    if !(c == ' ' as i32 || c == '\t' as i32 || c == '\r' as i32) {
                        break;
                    }
                }
                lexeme = if !lexptr.is_null() {
                    lexptr.offset(-(1 as i32 as isize))
                } else {
                    lexptr
                };
                thisline = 0 as *mut i8;
                tok = tokstart;
                if !(gawk_mb_cur_max == 1 as i32
                    || cur_char_ring[cur_ring_idx as usize] as i32 == 1 as i32)
                {
                    current_block = 13837311639792312879;
                    break '_collect_regexp;
                }
                match c {
                    -2000 => return 0 as i32,
                    -1000 => {
                        lasttok = newline_eof();
                        return lasttok;
                    }
                    10 => {
                        sourceline += 1;
                        sourceline;
                        lasttok = 308 as i32;
                        return lasttok;
                    }
                    35 => {
                        yylval = 0 as *mut INSTRUCTION;
                        if do_flags as u32
                            & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
                            && do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32
                                == 0
                        {
                            let mut new_comment: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            if lasttok == 308 as i32 || lasttok == 0 as i32 {
                                c = get_comment(
                                    commenttype::BLOCK_COMMENT,
                                    &mut new_comment,
                                );
                            } else {
                                c = get_comment(commenttype::EOL_COMMENT, &mut new_comment);
                            }
                            yylval = new_comment;
                            if c == -(1000 as i32) {
                                pushback();
                                lasttok = 308 as i32;
                                return lasttok;
                            }
                        } else {
                            loop {
                                c = nextc(0 as i32 != 0);
                                if !(c != '\n' as i32) {
                                    break;
                                }
                                if c == -(1000 as i32) {
                                    lasttok = newline_eof();
                                    return lasttok;
                                }
                            }
                        }
                        sourceline += 1;
                        sourceline;
                        lasttok = 308 as i32;
                        return lasttok;
                    }
                    64 => {
                        c = nextc(1 as i32 != 0);
                        if c == '/' as i32 {
                            current_block = 17688141731389699982;
                            break;
                        } else {
                            current_block = 1852451392920375136;
                            break;
                        }
                    }
                    92 => {
                        c = nextc(1 as i32 != 0);
                        if c == '\r' as i32 {
                            c = nextc(1 as i32 != 0);
                        }
                        if c == '\n' as i32 {
                            sourceline += 1;
                            sourceline;
                        } else {
                            yyerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"backslash not last character on line\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                            lasttok = 303 as i32;
                            return lasttok;
                        }
                    }
                    63 => {
                        qm_col_count += 1;
                        qm_col_count;
                        current_block = 644217479689742678;
                        break '_collect_regexp;
                    }
                    58 => {
                        current_block = 644217479689742678;
                        break '_collect_regexp;
                    }
                    41 => {
                        in_parens -= 1;
                        in_parens;
                        lasttok = c;
                        return lasttok;
                    }
                    40 => {
                        in_parens += 1;
                        in_parens;
                        lasttok = c;
                        return lasttok;
                    }
                    36 => {
                        yylval = bcalloc(opcodeval::Op_field_spec, 1 as i32, sourceline);
                        lasttok = c;
                        return lasttok;
                    }
                    123 => {
                        in_braces += 1;
                        if in_braces == 1 as i32 {
                            firstline = sourceline;
                        }
                        current_block = 5931128170645171119;
                        break '_collect_regexp;
                    }
                    59 | 44 | 91 => {
                        current_block = 5931128170645171119;
                        break '_collect_regexp;
                    }
                    93 => {
                        c = nextc(1 as i32 != 0);
                        pushback();
                        if c == '[' as i32 {
                            if do_flags as u32
                                & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const i8,
                                    3880 as i32,
                                );
                                (Some(
                                    (Some(
                                        r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                                    ))
                                        .expect("non-null function pointer"),
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        0 as *const i8,
                                        b"multidimensional arrays are a gawk extension\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            if do_flags as u32
                                & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                            {
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const i8,
                                    3882 as i32,
                                );
                                (Some(lintfunc.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        0 as *const i8,
                                        b"multidimensional arrays are a gawk extension\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            yylval = bcalloc(
                                opcodeval::Op_sub_array,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = ']' as i32;
                        } else {
                            yylval = bcalloc(
                                opcodeval::Op_subscript,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 272 as i32;
                        }
                        return ']' as i32;
                    }
                    42 => {
                        c = nextc(1 as i32 != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_assign_times,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 268 as i32;
                            return lasttok;
                        } else if do_flags as u32
                            & do_flag_values::DO_POSIX as i32 as u32 != 0
                        {
                            pushback();
                            yylval = bcalloc(opcodeval::Op_times, 1 as i32, sourceline);
                            lasttok = '*' as i32;
                            return lasttok;
                        } else if c == '*' as i32 {
                            static mut did_warn_op: bool = 0 as i32 != 0;
                            static mut did_warn_assgn: bool = 0 as i32 != 0;
                            if nextc(1 as i32 != 0) == '=' as i32 {
                                if !did_warn_assgn {
                                    did_warn_assgn = 1 as i32 != 0;
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const i8,
                                                i32,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const i8,
                                            3907 as i32,
                                        );
                                        (Some(lintfunc.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                0 as *const i8,
                                                b"POSIX does not allow operator `%s'\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            b"**=\0" as *const u8 as *const i8,
                                        );
                                    }
                                    if do_flags as u32
                                        & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const i8,
                                                i32,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const i8,
                                            3909 as i32,
                                        );
                                        (Some(lintfunc.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                0 as *const i8,
                                                b"operator `%s' is not supported in old awk\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            b"**=\0" as *const u8 as *const i8,
                                        );
                                    }
                                }
                                yylval = bcalloc(
                                    opcodeval::Op_assign_exp,
                                    1 as i32,
                                    sourceline,
                                );
                                return 268 as i32;
                            } else {
                                pushback();
                                if !did_warn_op {
                                    did_warn_op = 1 as i32 != 0;
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const i8,
                                                i32,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const i8,
                                            3918 as i32,
                                        );
                                        (Some(lintfunc.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                0 as *const i8,
                                                b"POSIX does not allow operator `%s'\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            b"**\0" as *const u8 as *const i8,
                                        );
                                    }
                                    if do_flags as u32
                                        & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                    {
                                        (set_loc
                                            as unsafe extern "C" fn(
                                                *const i8,
                                                i32,
                                            ) -> ())(
                                            b"awkgram.y\0" as *const u8 as *const i8,
                                            3920 as i32,
                                        );
                                        (Some(lintfunc.expect("non-null function pointer")))
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            dcgettext(
                                                0 as *const i8,
                                                b"operator `%s' is not supported in old awk\0" as *const u8
                                                    as *const i8,
                                                5 as i32,
                                            ),
                                            b"**\0" as *const u8 as *const i8,
                                        );
                                    }
                                }
                                yylval = bcalloc(opcodeval::Op_exp, 1 as i32, sourceline);
                                lasttok = '^' as i32;
                                return lasttok;
                            }
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_times, 1 as i32, sourceline);
                        lasttok = '*' as i32;
                        return lasttok;
                    }
                    47 => {
                        if nextc(0 as i32 != 0) == '=' as i32 {
                            pushback();
                            lasttok = 309 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_quotient, 1 as i32, sourceline);
                        lasttok = '/' as i32;
                        return lasttok;
                    }
                    37 => {
                        if nextc(1 as i32 != 0) == '=' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_assign_mod,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 268 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_mod, 1 as i32, sourceline);
                        lasttok = '%' as i32;
                        return lasttok;
                    }
                    94 => {
                        static mut did_warn_op_0: bool = 0 as i32 != 0;
                        static mut did_warn_assgn_0: bool = 0 as i32 != 0;
                        if nextc(1 as i32 != 0) == '=' as i32 {
                            if do_flags as u32
                                & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                && !did_warn_assgn_0
                            {
                                did_warn_assgn_0 = 1 as i32 != 0;
                                (set_loc
                                    as unsafe extern "C" fn(
                                        *const i8,
                                        i32,
                                    ) -> ())(
                                    b"awkgram.y\0" as *const u8 as *const i8,
                                    3955 as i32,
                                );
                                (Some(lintfunc.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dcgettext(
                                        0 as *const i8,
                                        b"operator `%s' is not supported in old awk\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                    b"^=\0" as *const u8 as *const i8,
                                );
                            }
                            yylval = bcalloc(
                                opcodeval::Op_assign_exp,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 268 as i32;
                            return lasttok;
                        }
                        pushback();
                        if do_flags as u32 & do_flag_values::DO_LINT_OLD as i32 as u32
                            != 0 && !did_warn_op_0
                        {
                            did_warn_op_0 = 1 as i32 != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"awkgram.y\0" as *const u8 as *const i8,
                                3963 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"operator `%s' is not supported in old awk\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                b"^\0" as *const u8 as *const i8,
                            );
                        }
                        yylval = bcalloc(opcodeval::Op_exp, 1 as i32, sourceline);
                        lasttok = '^' as i32;
                        return lasttok;
                    }
                    43 => {
                        c = nextc(1 as i32 != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_assign_plus,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 268 as i32;
                            return lasttok;
                        }
                        if c == '+' as i32 {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            lasttok = 299 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_plus, 1 as i32, sourceline);
                        lasttok = '+' as i32;
                        return lasttok;
                    }
                    33 => {
                        c = nextc(1 as i32 != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_notequal,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 265 as i32;
                            return lasttok;
                        }
                        if c == '~' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_nomatch,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 270 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                        lasttok = '!' as i32;
                        return lasttok;
                    }
                    60 => {
                        if nextc(1 as i32 != 0) == '=' as i32 {
                            yylval = bcalloc(opcodeval::Op_leq, 1 as i32, sourceline);
                            lasttok = 265 as i32;
                            return lasttok;
                        }
                        yylval = bcalloc(opcodeval::Op_less, 1 as i32, sourceline);
                        pushback();
                        lasttok = '<' as i32;
                        return lasttok;
                    }
                    61 => {
                        if nextc(1 as i32 != 0) == '=' as i32 {
                            yylval = bcalloc(opcodeval::Op_equal, 1 as i32, sourceline);
                            lasttok = 265 as i32;
                            return lasttok;
                        }
                        yylval = bcalloc(opcodeval::Op_assign, 1 as i32, sourceline);
                        pushback();
                        lasttok = 269 as i32;
                        return lasttok;
                    }
                    62 => {
                        c = nextc(1 as i32 != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(opcodeval::Op_geq, 1 as i32, sourceline);
                            lasttok = 265 as i32;
                            return lasttok;
                        } else if c == '>' as i32 {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            (*yylval).d.dl = redirval::redirect_append as i32 as i64;
                            lasttok = 266 as i32;
                            return lasttok;
                        }
                        pushback();
                        if in_print as i32 != 0 && in_parens == 0 as i32 {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            (*yylval).d.dl = redirval::redirect_output as i32 as i64;
                            lasttok = 266 as i32;
                            return lasttok;
                        }
                        yylval = bcalloc(opcodeval::Op_greater, 1 as i32, sourceline);
                        lasttok = '>' as i32;
                        return lasttok;
                    }
                    126 => {
                        yylval = bcalloc(opcodeval::Op_match, 1 as i32, sourceline);
                        lasttok = 270 as i32;
                        return lasttok;
                    }
                    125 => {
                        if did_newline {
                            did_newline = 0 as i32 != 0;
                            in_braces -= 1;
                            if in_braces == 0 as i32 {
                                lastline = sourceline;
                            }
                            lasttok = c;
                            return lasttok;
                        }
                        did_newline = 1 as i32 != 0;
                        lexptr = lexptr.offset(-1);
                        lexptr;
                        lasttok = 308 as i32;
                        return lasttok;
                    }
                    34 => {
                        current_block = 4474591357935695607;
                        break '_collect_regexp;
                    }
                    45 => {
                        c = nextc(1 as i32 != 0);
                        if c == '=' as i32 {
                            yylval = bcalloc(
                                opcodeval::Op_assign_minus,
                                1 as i32,
                                sourceline,
                            );
                            lasttok = 268 as i32;
                            return lasttok;
                        }
                        if c == '-' as i32 {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            lasttok = 300 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_minus, 1 as i32, sourceline);
                        lasttok = '-' as i32;
                        return lasttok;
                    }
                    46 => {
                        c = nextc(1 as i32 != 0);
                        pushback();
                        if *(*__ctype_b_loc()).offset(c as isize) as i32
                            & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                            == 0
                        {
                            lasttok = '.' as i32;
                            return lasttok;
                        } else {
                            c = '.' as i32;
                        }
                        current_block = 11475295656646479480;
                        break '_collect_regexp;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 11475295656646479480;
                        break '_collect_regexp;
                    }
                    38 => {
                        c = nextc(1 as i32 != 0);
                        if c == '&' as i32 {
                            yylval = bcalloc(opcodeval::Op_and, 1 as i32, sourceline);
                            let mut new_comment_1: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            allow_newline(&mut new_comment_1);
                            (*yylval).comment = new_comment_1;
                            lasttok = 297 as i32;
                            return lasttok;
                        }
                        pushback();
                        yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                        lasttok = '&' as i32;
                        return lasttok;
                    }
                    124 => {
                        c = nextc(1 as i32 != 0);
                        if c == '|' as i32 {
                            yylval = bcalloc(opcodeval::Op_or, 1 as i32, sourceline);
                            let mut new_comment_2: *mut INSTRUCTION = 0
                                as *mut INSTRUCTION;
                            allow_newline(&mut new_comment_2);
                            (*yylval).comment = new_comment_2;
                            lasttok = 298 as i32;
                            return lasttok;
                        } else if do_flags as u32
                            & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                            && c == '&' as i32
                        {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            (*yylval).d.dl = redirval::redirect_twoway as i32 as i64;
                            lasttok = if in_print as i32 != 0 && in_parens == 0 as i32 {
                                266 as i32
                            } else {
                                267 as i32
                            };
                            return lasttok;
                        }
                        pushback();
                        if in_print as i32 != 0 && in_parens == 0 as i32 {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            (*yylval).d.dl = redirval::redirect_pipe as i32 as i64;
                            lasttok = 266 as i32;
                            return lasttok;
                        } else {
                            yylval = bcalloc(opcodeval::Op_symbol, 1 as i32, sourceline);
                            (*yylval).d.dl = redirval::redirect_pipein as i32 as i64;
                            lasttok = 267 as i32;
                            return lasttok;
                        }
                    }
                    _ => {
                        current_block = 13837311639792312879;
                        break '_collect_regexp;
                    }
                }
            }
            match current_block {
                1852451392920375136 => {
                    pushback();
                    at_seen += 1;
                    at_seen;
                    lasttok = '@' as i32;
                    return lasttok;
                }
                _ => {
                    want_regexp = 1 as i32 != 0;
                    collecting_typed_regexp = 1 as i32 != 0;
                }
            }
        }
    }
    match current_block {
        11475295656646479480 => {
            loop {
                let mut gotnumber: bool = 0 as i32 != 0;
                let fresh102 = tok;
                tok = tok.offset(1);
                *fresh102 = c as i8;
                if tok == tokend {
                    tokexpand();
                } else {};
                match c {
                    120 | 88 => {
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            != 0
                        {
                            current_block = 4964131994796058463;
                        } else if tok == tokstart.offset(2 as i32 as isize) {
                            let mut peek_0: i32 = nextc(1 as i32 != 0);
                            if *(*__ctype_b_loc()).offset(peek_0 as isize) as i32
                                & C2RustUnnamed_0::_ISxdigit as i32 as libc::c_ushort as i32
                                != 0
                            {
                                inhex = 1 as i32 != 0;
                                pushback();
                                current_block = 8145001134476292673;
                            } else {
                                pushback();
                                current_block = 4964131994796058463;
                            }
                        } else {
                            current_block = 8145001134476292673;
                        }
                    }
                    46 => {
                        if seen_point as i32 != 0 || seen_e as i32 != 0 {
                            gotnumber = 1 as i32 != 0;
                        } else {
                            seen_point = 1 as i32 != 0;
                        }
                        current_block = 8145001134476292673;
                    }
                    101 | 69 => {
                        if inhex {
                            current_block = 8145001134476292673;
                        } else {
                            if seen_e {
                                gotnumber = 1 as i32 != 0;
                            } else {
                                seen_e = 1 as i32 != 0;
                                c = nextc(1 as i32 != 0);
                                if c == '-' as i32 || c == '+' as i32 {
                                    let mut c2: i32 = nextc(1 as i32 != 0);
                                    if *(*__ctype_b_loc()).offset(c2 as isize) as i32
                                        & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                        != 0
                                    {
                                        let fresh103 = tok;
                                        tok = tok.offset(1);
                                        *fresh103 = c as i8;
                                        if tok == tokend {
                                            tokexpand();
                                        } else {};
                                        let fresh104 = tok;
                                        tok = tok.offset(1);
                                        *fresh104 = c2 as i8;
                                        if tok == tokend {
                                            tokexpand();
                                        } else {};
                                    } else {
                                        pushback();
                                        pushback();
                                        pushback();
                                    }
                                } else if *(*__ctype_b_loc()).offset(c as isize) as i32
                                    & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32
                                    == 0
                                {
                                    pushback();
                                    pushback();
                                } else {
                                    pushback();
                                }
                            }
                            current_block = 8145001134476292673;
                        }
                    }
                    97 | 65 | 98 | 66 | 99 | 67 | 68 | 100 | 102 | 70 => {
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            != 0 || !inhex
                        {
                            current_block = 4964131994796058463;
                        } else {
                            current_block = 8145001134476292673;
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 8145001134476292673;
                    }
                    _ => {
                        current_block = 4964131994796058463;
                    }
                }
                match current_block {
                    4964131994796058463 => {
                        gotnumber = 1 as i32 != 0;
                    }
                    _ => {}
                }
                if gotnumber {
                    break;
                }
                c = nextc(1 as i32 != 0);
            }
            pushback();
            let fresh105 = tok;
            tok = tok.offset(1);
            *fresh105 = '\0' as i32 as i8;
            if tok == tokend {
                tokexpand();
            } else {};
            yylval = bcalloc(opcodeval::Op_push_i, 1 as i32, sourceline);
            base = 10 as i32;
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
                base = get_numbase(
                    tokstart,
                    (strlen(tokstart)).wrapping_sub(1 as i32 as u64),
                    0 as i32 != 0,
                );
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    if base == 8 as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const i8,
                            4237 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            b"numeric constant `%.*s' treated as octal\0" as *const u8
                                as *const i8,
                            strlen(tokstart) as i32 - 1 as i32,
                            tokstart,
                        );
                    } else if base == 16 as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const i8,
                            4240 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            b"numeric constant `%.*s' treated as hexadecimal\0"
                                as *const u8 as *const i8,
                            strlen(tokstart) as i32 - 1 as i32,
                            tokstart,
                        );
                    }
                }
            }
            if base != 10 as i32 {
                d = nondec2awknum(
                    tokstart,
                    (strlen(tokstart)).wrapping_sub(1 as i32 as u64),
                    0 as *mut *mut i8,
                );
            } else {
                d = atof(tokstart);
            }
            (*yylval).d.dn = set_profile_text(
                make_number.expect("non-null function pointer")(d),
                tokstart,
                (strlen(tokstart)).wrapping_sub(1 as i32 as u64),
            );
            if d <= 2147483647 as i32 as libc::c_double
                && d >= (-(2147483647 as i32) - 1 as i32) as libc::c_double
                && d == d as int32_t as libc::c_double
            {
                (*(*yylval).d.dn).flags = ::core::mem::transmute::<
                    u32,
                    flagvals,
                >((*(*yylval).d.dn).flags as u32 | flagvals::NUMINT as i32 as u32);
            }
            lasttok = 262 as i32;
            return lasttok;
        }
        13837311639792312879 => {
            if !is_letter(c) {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"invalid char '%c' in expression\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    c,
                );
                lasttok = 303 as i32;
                return lasttok;
            }
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
                && c == '_' as i32 && lasttok != '$' as i32
            {
                c = nextc(1 as i32 != 0);
                if c == '"' as i32 {
                    intlstr = 1 as i32 != 0;
                    current_block = 4474591357935695607;
                } else {
                    pushback();
                    c = '_' as i32;
                    current_block = 15508192053828812388;
                }
            } else {
                current_block = 15508192053828812388;
            }
            match current_block {
                4474591357935695607 => {}
                _ => {
                    tok = tokstart;
                    while c != -(1000 as i32) && is_identchar(c) as i32 != 0 {
                        let fresh106 = tok;
                        tok = tok.offset(1);
                        *fresh106 = c as i8;
                        if tok == tokend {
                            tokexpand();
                        } else {};
                        c = nextc(1 as i32 != 0);
                        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32
                            == 0 && c == ':' as i32
                        {
                            let mut peek_1: i32 = nextc(1 as i32 != 0);
                            if peek_1 == ':' as i32 {
                                let fresh107 = tok;
                                tok = tok.offset(1);
                                *fresh107 = c as i8;
                                if tok == tokend {
                                    tokexpand();
                                } else {};
                                let fresh108 = tok;
                                tok = tok.offset(1);
                                *fresh108 = c as i8;
                                if tok == tokend {
                                    tokexpand();
                                } else {};
                                c = nextc(1 as i32 != 0);
                            } else {
                                pushback();
                            }
                        }
                    }
                    let fresh109 = tok;
                    tok = tok.offset(1);
                    *fresh109 = '\0' as i32 as i8;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                    pushback();
                    validate_qualified_name(tokstart);
                    mid = check_qualified_special(tokstart);
                    if mid >= 0 as i32 {
                        static mut warntab: [i32; 71] = [0; 71];
                        let mut class: i32 = tokentab[mid as usize].class;
                        match class {
                            305 | 304 | 306 | 307 => {
                                if lasttok != '@' as i32 {
                                    current_block = 12844405573106229270;
                                } else {
                                    current_block = 763911284580919563;
                                }
                            }
                            _ => {
                                current_block = 763911284580919563;
                            }
                        }
                        match current_block {
                            12844405573106229270 => {}
                            _ => {
                                if tokentab[mid as usize].flags & 0x400 as i32 as u32
                                    != 0 as i32 as u32
                                {
                                    let mut f: *mut NODE = 0 as *mut NODE;
                                    match want_param_names as u32 {
                                        0 => {
                                            current_block = 12844405573106229270;
                                        }
                                        1 => {
                                            current_block = 12726453990834516271;
                                            match current_block {
                                                944459782820670644 => {
                                                    r_fatal(
                                                        b"internal error: file %s, line %d: bad value %d for want_param_names\0"
                                                            as *const u8 as *const i8,
                                                        b"awkgram.y\0" as *const u8 as *const i8,
                                                        4401 as i32,
                                                        want_param_names as i32,
                                                    );
                                                    current_block = 16055061654577082179;
                                                }
                                                _ => {
                                                    f = lookup(tokstart);
                                                    if !f.is_null() {
                                                        if (*f).type_0 as u32
                                                            == nodevals::Node_builtin_func as i32 as u32
                                                        {
                                                            current_block = 16055061654577082179;
                                                        } else {
                                                            current_block = 12844405573106229270;
                                                        }
                                                    } else {
                                                        current_block = 16055061654577082179;
                                                    }
                                                }
                                            }
                                        }
                                        2 => {
                                            current_block = 16055061654577082179;
                                        }
                                        _ => {
                                            current_block = 944459782820670644;
                                            match current_block {
                                                944459782820670644 => {
                                                    r_fatal(
                                                        b"internal error: file %s, line %d: bad value %d for want_param_names\0"
                                                            as *const u8 as *const i8,
                                                        b"awkgram.y\0" as *const u8 as *const i8,
                                                        4401 as i32,
                                                        want_param_names as i32,
                                                    );
                                                    current_block = 16055061654577082179;
                                                }
                                                _ => {
                                                    f = lookup(tokstart);
                                                    if !f.is_null() {
                                                        if (*f).type_0 as u32
                                                            == nodevals::Node_builtin_func as i32 as u32
                                                        {
                                                            current_block = 16055061654577082179;
                                                        } else {
                                                            current_block = 12844405573106229270;
                                                        }
                                                    } else {
                                                        current_block = 16055061654577082179;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 16055061654577082179;
                                }
                                match current_block {
                                    12844405573106229270 => {}
                                    _ => {
                                        if do_flags as u32
                                            & (do_flag_values::DO_LINT_INVALID as i32
                                                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                        {
                                            if do_flags as u32
                                                & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                                                && tokentab[mid as usize].flags & 0x400 as i32 as u32
                                                    != 0 as i32 as u32
                                                && warntab[mid as usize] & 0x400 as i32 == 0 as i32
                                            {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    4408 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"`%s' is a gawk extension\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    tokentab[mid as usize].operator,
                                                );
                                                warntab[mid as usize] |= 0x400 as i32;
                                            }
                                            if tokentab[mid as usize].flags & 0x200 as i32 as u32
                                                != 0 as i32 as u32
                                                && warntab[mid as usize] & 0x200 as i32 == 0 as i32
                                            {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    4413 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"POSIX does not allow `%s'\0" as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    tokentab[mid as usize].operator,
                                                );
                                                warntab[mid as usize] |= 0x200 as i32;
                                            }
                                        }
                                        if do_flags as u32
                                            & do_flag_values::DO_LINT_OLD as i32 as u32 != 0
                                            && tokentab[mid as usize].flags & 0x100 as i32 as u32
                                                != 0 as i32 as u32
                                            && warntab[mid as usize] & 0x100 as i32 == 0 as i32
                                        {
                                            (set_loc
                                                as unsafe extern "C" fn(
                                                    *const i8,
                                                    i32,
                                                ) -> ())(
                                                b"awkgram.y\0" as *const u8 as *const i8,
                                                4421 as i32,
                                            );
                                            (Some(lintfunc.expect("non-null function pointer")))
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                dcgettext(
                                                    0 as *const i8,
                                                    b"`%s' is not supported in old awk\0" as *const u8
                                                        as *const i8,
                                                    5 as i32,
                                                ),
                                                tokentab[mid as usize].operator,
                                            );
                                            warntab[mid as usize] |= 0x100 as i32;
                                        }
                                        if tokentab[mid as usize].flags & 0x800 as i32 as u32
                                            != 0 as i32 as u32
                                        {
                                            break_allowed += 1;
                                            break_allowed;
                                        }
                                        if tokentab[mid as usize].flags & 0x1000 as i32 as u32
                                            != 0 as i32 as u32
                                        {
                                            continue_allowed += 1;
                                            continue_allowed;
                                        }
                                        match class {
                                            307 => {
                                                want_namespace = 1 as i32 != 0;
                                                current_block = 4882834556340228006;
                                            }
                                            304 | 306 => {
                                                current_block = 4882834556340228006;
                                            }
                                            305 => {
                                                if in_main_context() != 0 {
                                                    current_block = 12844405573106229270;
                                                } else {
                                                    tokkey = emalloc_real(
                                                        (tok.offset_from(tokstart) as i64 + 1 as i32 as i64)
                                                            as size_t,
                                                        b"yylex\0" as *const u8 as *const i8,
                                                        b"tokkey\0" as *const u8 as *const i8,
                                                        b"awkgram.y\0" as *const u8 as *const i8,
                                                        4442 as i32,
                                                    ) as *mut i8;
                                                    *tokkey.offset(0 as i32 as isize) = '@' as i32 as i8;
                                                    memcpy(
                                                        tokkey.offset(1 as i32 as isize) as *mut libc::c_void,
                                                        tokstart as *const libc::c_void,
                                                        tok.offset_from(tokstart) as i64 as u64,
                                                    );
                                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                                    (*yylval).d.name = tokkey;
                                                    current_block = 14170946608255986518;
                                                }
                                            }
                                            291 | 273 | 274 | 292 | 293 => {
                                                yylval = bcalloc(
                                                    tokentab[mid as usize].value,
                                                    4 as i32,
                                                    sourceline,
                                                );
                                                current_block = 14170946608255986518;
                                            }
                                            284 | 282 | 283 | 279 => {
                                                if do_flags as u32
                                                    & do_flag_values::DO_PRETTY_PRINT as i32 as u32 == 0
                                                {
                                                    lasttok = class;
                                                    return lasttok;
                                                }
                                                current_block = 3448797690533440671;
                                            }
                                            280 => {
                                                current_block = 3448797690533440671;
                                            }
                                            286 => {
                                                if continue_allowed == 0 {
                                                    error_ln(
                                                        sourceline,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"`continue' is not allowed outside a loop\0" as *const u8
                                                                as *const i8,
                                                            5 as i32,
                                                        ),
                                                    );
                                                    errcount += 1;
                                                    errcount;
                                                }
                                                current_block = 5010899242954472415;
                                            }
                                            285 => {
                                                if break_allowed == 0 {
                                                    error_ln(
                                                        sourceline,
                                                        dcgettext(
                                                            0 as *const i8,
                                                            b"`break' is not allowed outside a loop or switch\0"
                                                                as *const u8 as *const i8,
                                                            5 as i32,
                                                        ),
                                                    );
                                                    errcount += 1;
                                                    errcount;
                                                }
                                                current_block = 5010899242954472415;
                                            }
                                            _ => {
                                                current_block = 5010899242954472415;
                                            }
                                        }
                                        match current_block {
                                            12844405573106229270 => {}
                                            _ => {
                                                match current_block {
                                                    5010899242954472415 => {
                                                        yylval = bcalloc(
                                                            tokentab[mid as usize].value,
                                                            1 as i32,
                                                            sourceline,
                                                        );
                                                        if class == 301 as i32 || class == 302 as i32 {
                                                            (*yylval).d.dl = mid as i64;
                                                        }
                                                    }
                                                    4882834556340228006 => {
                                                        want_source = 1 as i32 != 0;
                                                    }
                                                    3448797690533440671 => {
                                                        yylval = bcalloc(
                                                            tokentab[mid as usize].value,
                                                            2 as i32,
                                                            sourceline,
                                                        );
                                                    }
                                                    _ => {}
                                                }
                                                lasttok = class;
                                                return lasttok;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if want_param_names as u32
                        == C2RustUnnamed_12::FUNC_HEADER as i32 as u32
                    {
                        tokkey = estrdup(
                            tokstart,
                            (tok.offset_from(tokstart) as i64 - 1 as i32 as i64)
                                as size_t,
                        );
                    } else {
                        tokkey = qualify_name(
                            tokstart,
                            (tok.offset_from(tokstart) as i64 - 1 as i32 as i64)
                                as size_t,
                        );
                    }
                    if *lexptr as i32 == '(' as i32 {
                        yylval = bcalloc(opcodeval::Op_token, 2 as i32, sourceline);
                        (*yylval).d.name = tokkey;
                        lasttok = 258 as i32;
                        return lasttok;
                    } else {
                        static mut goto_warned: bool = 0 as i32 != 0;
                        yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                        (*yylval).d.name = tokkey;
                        if 1 as i32 != 0
                            && do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            && !goto_warned
                            && ({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<i8>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = *tokkey.offset(0 as i32 as isize) as i32;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower(*tokkey.offset(0 as i32 as isize) as i32);
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(*tokkey.offset(0 as i32 as isize) as i32 as isize);
                                }
                                __res
                            }) == 'g' as i32
                            && strcasecmp(tokkey, b"goto\0" as *const u8 as *const i8)
                                == 0 as i32
                        {
                            goto_warned = 1 as i32 != 0;
                            (set_loc
                                as unsafe extern "C" fn(
                                    *const i8,
                                    i32,
                                ) -> ())(
                                b"awkgram.y\0" as *const u8 as *const i8,
                                4521 as i32,
                            );
                            (Some(lintfunc.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                dcgettext(
                                    0 as *const i8,
                                    b"`goto' considered harmful!\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        lasttok = 259 as i32;
                        return lasttok;
                    }
                }
            }
        }
        644217479689742678 => {
            yylval = bcalloc(opcodeval::Op_cond_exp, 1 as i32, sourceline);
            if qm_col_count > 0 as i32 {
                if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0 {
                    let mut new_comment_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                    allow_newline(&mut new_comment_0);
                    (*yylval).comment = new_comment_0;
                }
                if c == ':' as i32 {
                    qm_col_count -= 1;
                    qm_col_count;
                }
            }
            lasttok = c;
            return lasttok;
        }
        2370887241019905314 => {
            loop {
                c = nextc(0 as i32 != 0);
                cur_index = tok.offset_from(tokstart) as i64 as i32;
                if gawk_mb_cur_max == 1 as i32
                    || cur_char_ring[cur_ring_idx as usize] as i32 == 1 as i32
                {
                    match c {
                        91 => {
                            current_block = 5321283069724646002;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        93 => {
                            current_block = 16736214510549657799;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        92 => {
                            current_block = 2391368634336484244;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        47 => {
                            current_block = 5882019081378595537;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        10 => {
                            current_block = 5699956056677916277;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        -1000 => {
                            current_block = 10528847967584931547;
                            match current_block {
                                5882019081378595537 => {
                                    if in_brack > 0 as i32 {
                                        current_block = 6545907279487748450;
                                    } else {
                                        current_block = 14336382367753288579;
                                    }
                                }
                                10528847967584931547 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp at end of file\0" as *const u8
                                                as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5699956056677916277 => {
                                    pushback();
                                    yyerror(
                                        dcgettext(
                                            0 as *const i8,
                                            b"unterminated regexp\0" as *const u8 as *const i8,
                                            5 as i32,
                                        ),
                                    );
                                    current_block = 14336382367753288579;
                                }
                                5321283069724646002 => {
                                    if nextc(0 as i32 != 0) == ':' as i32
                                        || in_brack == 0 as i32
                                    {
                                        in_brack += 1;
                                        in_brack;
                                        if in_brack == 1 as i32 {
                                            b_index = tok.offset_from(tokstart) as i64 as i32;
                                        }
                                    }
                                    pushback();
                                    current_block = 6545907279487748450;
                                }
                                16736214510549657799 => {
                                    if !(in_brack > 0 as i32
                                        && (cur_index == b_index + 1 as i32
                                            || cur_index == b_index + 2 as i32
                                                && *tok.offset(-(1 as i32) as isize) as i32 == '^' as i32))
                                    {
                                        in_brack -= 1;
                                        in_brack;
                                        if in_brack == 0 as i32 {
                                            b_index = -(1 as i32);
                                        }
                                    }
                                    current_block = 6545907279487748450;
                                }
                                _ => {
                                    c = nextc(0 as i32 != 0);
                                    if c == -(1000 as i32) {
                                        pushback();
                                        yyerror(
                                            dcgettext(
                                                0 as *const i8,
                                                b"unterminated regexp ends with `\\' at end of file\0"
                                                    as *const u8 as *const i8,
                                                5 as i32,
                                            ),
                                        );
                                    } else {
                                        if c == '\r' as i32 {
                                            c = nextc(1 as i32 != 0);
                                        }
                                        if c == '\n' as i32 {
                                            sourceline += 1;
                                            sourceline;
                                            continue;
                                        } else {
                                            let fresh97 = tok;
                                            tok = tok.offset(1);
                                            *fresh97 = '\\' as i32 as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            let fresh98 = tok;
                                            tok = tok.offset(1);
                                            *fresh98 = c as i8;
                                            if tok == tokend {
                                                tokexpand();
                                            } else {};
                                            continue;
                                        }
                                    }
                                    current_block = 14336382367753288579;
                                }
                            }
                            match current_block {
                                6545907279487748450 => {}
                                _ => {
                                    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
                                    (*yylval).d.name = estrdup(
                                        tokstart,
                                        tok.offset_from(tokstart) as i64 as size_t,
                                    );
                                    if do_flags as u32
                                        & (do_flag_values::DO_LINT_INVALID as i32
                                            | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                                    {
                                        let mut peek: i32 = nextc(1 as i32 != 0);
                                        pushback();
                                        if peek == 'i' as i32 || peek == 's' as i32 {
                                            if !source.is_null() {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3699 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"%s: %d: tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    source,
                                                    sourceline,
                                                    peek,
                                                );
                                            } else {
                                                (set_loc
                                                    as unsafe extern "C" fn(
                                                        *const i8,
                                                        i32,
                                                    ) -> ())(
                                                    b"awkgram.y\0" as *const u8 as *const i8,
                                                    3703 as i32,
                                                );
                                                (Some(lintfunc.expect("non-null function pointer")))
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    dcgettext(
                                                        0 as *const i8,
                                                        b"tawk regex modifier `/.../%c' doesn't work in gawk\0"
                                                            as *const u8 as *const i8,
                                                        5 as i32,
                                                    ),
                                                    peek,
                                                );
                                            }
                                        }
                                    }
                                    if collecting_typed_regexp {
                                        collecting_typed_regexp = 0 as i32 != 0;
                                        lasttok = 264 as i32;
                                    } else {
                                        lasttok = 260 as i32;
                                    }
                                    return lasttok;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                let fresh99 = tok;
                tok = tok.offset(1);
                *fresh99 = c as i8;
                if tok == tokend {
                    tokexpand();
                } else {};
            }
        }
        5931128170645171119 => {
            lasttok = c;
            return lasttok;
        }
        _ => {}
    }
    esc_seen = 0 as i32 != 0;
    loop {
        c = nextc(0 as i32 != 0);
        if !(c != '"' as i32) {
            break;
        }
        if c == '\n' as i32 {
            pushback();
            yyerror(
                dcgettext(
                    0 as *const i8,
                    b"unterminated string\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            lasttok = 303 as i32;
            return lasttok;
        }
        if (gawk_mb_cur_max == 1 as i32
            || cur_char_ring[cur_ring_idx as usize] as i32 == 1 as i32)
            && c == '\\' as i32
        {
            c = nextc(1 as i32 != 0);
            if c == '\r' as i32 {
                c = nextc(1 as i32 != 0);
            }
            if c == '\n' as i32 {
                if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4070 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"POSIX does not allow physical newlines in string values\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                } else if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4072 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"backslash string continuation is not portable\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                sourceline += 1;
                sourceline;
                continue;
            } else {
                esc_seen = 1 as i32 != 0;
                if !want_source || c != '"' as i32 {
                    let fresh100 = tok;
                    tok = tok.offset(1);
                    *fresh100 = '\\' as i32 as i8;
                    if tok == tokend {
                        tokexpand();
                    } else {};
                }
            }
        }
        if c == -(1000 as i32) {
            pushback();
            yyerror(
                dcgettext(
                    0 as *const i8,
                    b"unterminated string\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            lasttok = 303 as i32;
            return lasttok;
        }
        let fresh101 = tok;
        tok = tok.offset(1);
        *fresh101 = c as i8;
        if tok == tokend {
            tokexpand();
        } else {};
    }
    yylval = bcalloc(opcodeval::Op_token, 1 as i32, sourceline);
    if want_source {
        (*yylval).d.name = estrdup(tokstart, tok.offset_from(tokstart) as i64 as size_t);
        if want_namespace {
            change_namespace((*yylval).d.name);
        }
        lasttok = 261 as i32;
        return lasttok;
    }
    (*yylval).opcode = opcodeval::Op_push_i;
    (*yylval).d.dn = make_str_node(
        tokstart,
        tok.offset_from(tokstart) as i64 as size_t,
        if esc_seen as i32 != 0 { 1 as i32 } else { 0 as i32 },
    );
    if intlstr {
        (*(*yylval).d.dn).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*(*yylval).d.dn).flags as u32 | flagvals::INTLSTR as i32 as u32);
        intlstr = 0 as i32 != 0;
        if do_flags as u32 & do_flag_values::DO_INTL as i32 as u32 != 0 {
            dumpintlstr((*(*yylval).d.dn).sub.val.sp, (*(*yylval).d.dn).sub.val.slen);
        }
    }
    lasttok = 263 as i32;
    return lasttok;
}
unsafe extern "C" fn snode(
    mut subn: *mut INSTRUCTION,
    mut r: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut arg: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut nexp: i32 = 0 as i32;
    let mut args_allowed: i32 = 0;
    let mut idx: i32 = (*r).d.dl as i32;
    if !subn.is_null() {
        let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        tp = (*subn).nexti;
        while !tp.is_null() {
            tp = (*tp).d.di;
            nexp += 1;
            nexp;
            tp = (*tp).nexti;
        }
    }
    args_allowed = (tokentab[idx as usize].flags & 0xff as i32 as u32) as i32;
    if args_allowed != 0 && args_allowed & (1 as i32) << nexp == 0 as i32 {
        yyerror(
            dcgettext(
                0 as *const i8,
                b"%d is invalid as number of arguments for %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            nexp,
            tokentab[idx as usize].operator,
        );
        return 0 as *mut INSTRUCTION;
    }
    if tokentab[idx as usize].value as u32 == opcodeval::Op_sub_builtin as i32 as u32 {
        let mut operator: *const i8 = tokentab[idx as usize].operator;
        (*r).d.dl = 0 as i32 as i64;
        arg = (*subn).nexti;
        mk_rexp(arg);
        if strcmp(operator, b"gensub\0" as *const u8 as *const i8) != 0 as i32 {
            if strcmp(operator, b"gsub\0" as *const u8 as *const i8) == 0 as i32 {
                (*r).d.dl |= 0x1 as i32 as i64;
            }
            arg = (*(*arg).d.di).nexti;
            if nexp == 2 as i32 {
                let mut expr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
                expr = list_create(bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32));
                (*(*expr).nexti).d.dn = set_profile_text(
                    make_number.expect("non-null function pointer")(0.0f64),
                    b"0\0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                );
                mk_expression_list(
                    subn,
                    list_append(
                        expr,
                        bcalloc(opcodeval::Op_field_spec, 1 as i32, 0 as i32),
                    ),
                );
            }
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push_i as i32 as u32 {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4625 as i32);
                    (Some(lintfunc.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"%s: string literal as last argument of substitute has no effect\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        operator,
                    );
                }
                (*r).d.dl |= 0x4 as i32 as i64;
            } else if (make_assignable(ip)).is_null() {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"%s third parameter is not a changeable object\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    operator,
                );
            } else {
                (*ip).x.xl = 1 as i32 as i64;
            }
            (*r).x.xl = count_expressions(&mut subn, 0 as i32 != 0) as i64;
            ip = (*subn).d.di;
            list_append(subn, r);
            if (*ip).opcode as u32 == opcodeval::Op_push_lhs as i32 as u32
                && (*(*ip).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
                && ((*(*ip).d.dn).sub.nodep.x.aptr).is_some()
            {
                list_append(subn, bcalloc(opcodeval::Op_var_assign, 1 as i32, 0 as i32));
                (*(*subn).d.di).d.dl = opcodeval::Op_sub_builtin as i32 as i64;
                (*(*subn).d.di).x.aptr = (*(*ip).d.dn).sub.nodep.x.aptr;
            } else if (*ip).opcode as u32 == opcodeval::Op_field_spec_lhs as i32 as u32 {
                list_append(
                    subn,
                    bcalloc(opcodeval::Op_field_assign, 1 as i32, 0 as i32),
                );
                (*(*subn).d.di).d.dl = opcodeval::Op_sub_builtin as i32 as i64;
                (*(*subn).d.di).x.aptr = None;
                (*ip).d.di = (*subn).d.di;
            } else if (*ip).opcode as u32 == opcodeval::Op_subscript_lhs as i32 as u32 {
                list_append(
                    subn,
                    bcalloc(opcodeval::Op_subscript_assign, 1 as i32, 0 as i32),
                );
                (*(*subn).d.di).d.dl = opcodeval::Op_sub_builtin as i32 as i64;
            }
            return subn;
        } else {
            (*r).d.dl |= 0x2 as i32 as i64;
            if nexp == 3 as i32 {
                ip = bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32);
                (*ip).d.dn = set_profile_text(
                    make_number.expect("non-null function pointer")(0.0f64),
                    b"0\0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                );
                mk_expression_list(
                    subn,
                    list_append(
                        list_create(ip),
                        bcalloc(opcodeval::Op_field_spec, 1 as i32, 0 as i32),
                    ),
                );
            }
            (*r).x.xl = count_expressions(&mut subn, 0 as i32 != 0) as i64;
            return list_append(subn, r);
        }
    }
    (*r).d.fptr = tokentab[idx as usize].ptr;
    if (*r).d.fptr == Some(do_length as unsafe extern "C" fn(i32) -> *mut NODE) {
        if nexp == 0 as i32 {
            let mut list: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            (*r).x.xl = 1 as i32 as i64;
            list = list_create(r);
            list_prepend(list, bcalloc(opcodeval::Op_field_spec, 1 as i32, 0 as i32));
            list_prepend(list, bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32));
            (*(*list).nexti).d.dn = set_profile_text(
                make_number.expect("non-null function pointer")(0.0f64),
                b"0\0" as *const u8 as *const i8,
                1 as i32 as size_t,
            );
            return list;
        } else {
            arg = (*subn).nexti;
            if (*arg).nexti == (*arg).d.di
                && (*(*arg).nexti).opcode as u32 == opcodeval::Op_push as i32 as u32
            {
                (*(*arg).nexti).opcode = opcodeval::Op_push_arg;
            }
        }
    } else if (*r).d.fptr == Some(do_isarray as unsafe extern "C" fn(i32) -> *mut NODE) {
        arg = (*subn).nexti;
        if (*arg).nexti == (*arg).d.di
            && (*(*arg).nexti).opcode as u32 == opcodeval::Op_push as i32 as u32
        {
            (*(*arg).nexti).opcode = opcodeval::Op_push_arg_untyped;
        }
    } else if (*r).d.fptr == Some(do_typeof as unsafe extern "C" fn(i32) -> *mut NODE) {
        arg = (*subn).nexti;
        if (*arg).nexti == (*arg).d.di
            && (*(*arg).nexti).opcode as u32 == opcodeval::Op_push as i32 as u32
        {
            (*(*arg).nexti).opcode = opcodeval::Op_push_arg_untyped;
        }
        if nexp == 2 as i32 {
            arg = (*(*(*subn).nexti).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
                (*ip).opcode = opcodeval::Op_push_array;
            }
        }
    } else if (*r).d.fptr == Some(do_match as unsafe extern "C" fn(i32) -> *mut NODE) {
        static mut warned: bool = 0 as i32 != 0;
        arg = (*(*(*subn).nexti).d.di).nexti;
        mk_rexp(arg);
        if nexp == 3 as i32 {
            if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                && !warned
            {
                warned = 1 as i32 != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4734 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"match: third argument is a gawk extension\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"match: third argument is a gawk extension\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as *mut INSTRUCTION;
            }
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
                (*ip).opcode = opcodeval::Op_push_array;
            }
        }
    } else if (*r).d.fptr == Some(do_split as unsafe extern "C" fn(i32) -> *mut NODE) {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
            (*ip).opcode = opcodeval::Op_push_array;
        }
        if nexp == 2 as i32 {
            let mut expr_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            expr_0 = list_create(bcalloc(opcodeval::Op_push, 1 as i32, 0 as i32));
            (*(*expr_0).nexti).d.dn = FS_node;
            mk_expression_list(subn, expr_0);
        }
        arg = (*(*arg).d.di).nexti;
        n = mk_rexp(arg);
        if nexp == 2 as i32 {
            (*n).sub.nodep.reflags = ::core::mem::transmute::<
                u32,
                reflagvals,
            >((*n).sub.nodep.reflags as u32 | reflagvals::FS_DFLT as i32 as u32);
        }
        if nexp == 4 as i32 {
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
                (*ip).opcode = opcodeval::Op_push_array;
            }
        }
    } else if (*r).d.fptr == Some(do_patsplit as unsafe extern "C" fn(i32) -> *mut NODE)
    {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
            (*ip).opcode = opcodeval::Op_push_array;
        }
        if nexp == 2 as i32 {
            let mut expr_1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            expr_1 = list_create(bcalloc(opcodeval::Op_push, 1 as i32, 0 as i32));
            (*(*expr_1).nexti).d.dn = FPAT_node;
            mk_expression_list(subn, expr_1);
        }
        arg = (*(*arg).d.di).nexti;
        n = mk_rexp(arg);
        if nexp == 4 as i32 {
            arg = (*(*arg).d.di).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
                (*ip).opcode = opcodeval::Op_push_array;
            }
        }
    } else if (*r).d.fptr == Some(do_close as unsafe extern "C" fn(i32) -> *mut NODE) {
        static mut warned_0: bool = 0 as i32 != 0;
        if nexp == 2 as i32 {
            if do_flags as u32 & do_flag_values::DO_LINT_EXTENSIONS as i32 as u32 != 0
                && !warned_0
            {
                warned_0 = 1 as i32 != 0;
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4791 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"close: second argument is a gawk extension\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
            }
            if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
                yyerror(
                    dcgettext(
                        0 as *const i8,
                        b"close: second argument is a gawk extension\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as *mut INSTRUCTION;
            }
        }
    } else if do_flags as u32 & do_flag_values::DO_INTL as i32 as u32 != 0
        && (*r).d.fptr == Some(do_dcgettext as unsafe extern "C" fn(i32) -> *mut NODE)
        && (*(*(*subn).nexti).d.di).opcode as u32 == opcodeval::Op_push_i as i32 as u32
        && (*(*(*(*subn).nexti).d.di).d.dn).flags as u32 & flagvals::STRING as i32 as u32
            != 0 as i32 as u32
    {
        let mut str: *mut NODE = (*(*(*subn).nexti).d.di).d.dn;
        if (*str).flags as u32 & flagvals::INTLSTR as i32 as u32 != 0 as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4806 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"use of dcgettext(_\"...\") is incorrect: remove leading underscore\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            dumpintlstr((*str).sub.val.sp, (*str).sub.val.slen);
        }
    } else if do_flags as u32 & do_flag_values::DO_INTL as i32 as u32 != 0
        && (*r).d.fptr == Some(do_dcngettext as unsafe extern "C" fn(i32) -> *mut NODE)
        && (*(*(*subn).nexti).d.di).opcode as u32 == opcodeval::Op_push_i as i32 as u32
        && (*(*(*(*subn).nexti).d.di).d.dn).flags as u32 & flagvals::STRING as i32 as u32
            != 0 as i32 as u32
        && (*(*(*(*(*subn).nexti).d.di).nexti).d.di).opcode as u32
            == opcodeval::Op_push_i as i32 as u32
        && (*(*(*(*(*(*subn).nexti).d.di).nexti).d.di).d.dn).flags as u32
            & flagvals::STRING as i32 as u32 != 0 as i32 as u32
    {
        let mut str1: *mut NODE = (*(*(*subn).nexti).d.di).d.dn;
        let mut str2: *mut NODE = (*(*(*(*(*subn).nexti).d.di).nexti).d.di).d.dn;
        if ((*str1).flags as u32 | (*str2).flags as u32)
            & flagvals::INTLSTR as i32 as u32 != 0 as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4821 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"use of dcngettext(_\"...\") is incorrect: remove leading underscore\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            dumpintlstr2(
                (*str1).sub.val.sp,
                (*str1).sub.val.slen,
                (*str2).sub.val.sp,
                (*str2).sub.val.slen,
            );
        }
    } else if (*r).d.fptr == Some(do_asort as unsafe extern "C" fn(i32) -> *mut NODE)
        || (*r).d.fptr == Some(do_asorti as unsafe extern "C" fn(i32) -> *mut NODE)
    {
        arg = (*subn).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
            (*ip).opcode = opcodeval::Op_push_array;
        }
        if nexp >= 2 as i32 {
            arg = (*ip).nexti;
            ip = (*arg).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_push as i32 as u32 {
                (*ip).opcode = opcodeval::Op_push_array;
            }
        }
    } else if (*r).d.fptr == Some(do_index as unsafe extern "C" fn(i32) -> *mut NODE) {
        arg = (*(*(*subn).nexti).d.di).nexti;
        ip = (*arg).d.di;
        if (*ip).opcode as u32 == opcodeval::Op_match_rec as i32 as u32
            || (*ip).opcode as u32 == opcodeval::Op_push_re as i32 as u32
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4840 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"index: regexp constant as second argument is not allowed\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    if !subn.is_null() {
        (*r).x.xl = count_expressions(&mut subn, 0 as i32 != 0) as i64;
        return list_append(subn, r);
    }
    (*r).x.xl = 0 as i32 as i64;
    return list_create(r);
}
unsafe extern "C" fn parms_shadow(
    mut pc: *mut INSTRUCTION,
    mut shadow: *mut bool,
) -> i32 {
    let mut pcount: i32 = 0;
    let mut i: i32 = 0;
    let mut ret: bool = 0 as i32 != 0;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut fp_0: *mut NODE = 0 as *mut NODE;
    let mut fname: *mut i8 = 0 as *mut i8;
    func = (*pc).x.xn;
    fname = (*func).sub.nodep.name;
    fp_0 = (*func).sub.nodep.rn;
    pcount = (*func).sub.nodep.l.ll as i32;
    if pcount == 0 as i32 {
        return 0 as i32;
    }
    source = (*pc).d.name;
    sourceline = (*pc).source_line as i32;
    i = 0 as i32;
    while i < pcount {
        if !(lookup((*fp_0.offset(i as isize)).sub.nodep.name)).is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4892 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"function `%s': parameter `%s' shadows global variable\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                (*fp_0.offset(i as isize)).sub.nodep.name,
            );
            ret = 1 as i32 != 0;
        }
        i += 1;
        i;
    }
    *shadow = (*shadow as i32 | ret as i32) as bool;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn valinfo(
    mut n: *mut NODE,
    mut print_func: Func_print,
    mut fp_0: *mut FILE,
) {
    if n == Nnull_string {
        print_func
            .expect(
                "non-null function pointer",
            )(fp_0, b"uninitialized scalar\n\0" as *const u8 as *const i8);
    } else if (*n).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32 {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp_0,
            b"@/%.*s/\n\0" as *const u8 as *const i8,
            (*n).sub.val.slen,
            (*n).sub.val.sp,
        );
    } else if (*n).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32 {
        pp_string_fp(
            print_func,
            fp_0,
            (*n).sub.val.sp,
            (*n).sub.val.slen,
            '"' as i32,
            0 as i32 != 0,
        );
        print_func
            .expect(
                "non-null function pointer",
            )(fp_0, b"\n\0" as *const u8 as *const i8);
    } else if (*n).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32 {
        print_func
            .expect(
                "non-null function pointer",
            )(fp_0, b"%.17g\n\0" as *const u8 as *const i8, (*n).sub.val.fltnum);
    } else {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp_0,
            b"?? flags %s\n\0" as *const u8 as *const i8,
            flags2str((*n).flags as i32),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_vars(mut fname: *const i8) {
    let mut fp_0: *mut FILE = 0 as *mut FILE;
    let mut vars: *mut *mut NODE = 0 as *mut *mut NODE;
    if fname.is_null() {
        fp_0 = stderr;
    } else if strcmp(fname, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        fp_0 = stdout;
    } else {
        fp_0 = fopen(fname, b"w\0" as *const u8 as *const i8);
        if fp_0.is_null() {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4942 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"could not open `%s' for writing: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                strerror(*__errno_location()),
            );
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4943 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"sending variable list to standard error\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            fp_0 = stderr;
        }
    }
    vars = variable_list();
    print_vars(
        vars,
        Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
        fp_0,
    );
    pma_free(vars as *mut libc::c_void);
    if fp_0 != stdout && fp_0 != stderr && fclose(fp_0) != 0 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4951 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: close failed: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
            strerror(*__errno_location()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn dump_funcs() {
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    funcs = function_list(1 as i32 != 0);
    foreach_func(
        funcs,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
            Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
        >(
            Some(
                pp_func
                    as unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32,
            ),
        ),
        0 as *mut libc::c_void,
    );
    pma_free(funcs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn shadow_funcs() {
    static mut calls: i32 = 0 as i32;
    let mut shadow: bool = 0 as i32 != 0;
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    let fresh110 = calls;
    calls = calls + 1;
    if fresh110 != 0 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4976 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"shadow_funcs() called twice!\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    funcs = function_list(1 as i32 != 0);
    foreach_func(
        funcs,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut bool) -> i32>,
            Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
        >(
            Some(
                parms_shadow as unsafe extern "C" fn(*mut INSTRUCTION, *mut bool) -> i32,
            ),
        ),
        &mut shadow as *mut bool as *mut libc::c_void,
    );
    pma_free(funcs as *mut libc::c_void);
    if shadow as i32 != 0
        && lintfunc == Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ())
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 4984 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"there were shadowed variables\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
unsafe extern "C" fn mk_function(
    mut fi: *mut INSTRUCTION,
    mut def: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut thisfunc: *mut NODE = 0 as *mut NODE;
    thisfunc = (*fi).x.xn;
    if !interblock_comment.is_null() {
        (*interblock_comment).source_line = 0 as i32 as libc::c_short;
        merge_comments(interblock_comment, (*fi).comment);
        (*fi).comment = interblock_comment;
        interblock_comment = 0 as *mut INSTRUCTION;
    }
    list_append(def, bcalloc(opcodeval::Op_push_i, 1 as i32, 0 as i32));
    (*(*def).d.di).d.dn = dupnode(Nnull_string);
    list_append(def, bcalloc(opcodeval::Op_K_return, 1 as i32, 0 as i32));
    if !trailing_comment.is_null() {
        list_append(def, trailing_comment);
        trailing_comment = 0 as *mut INSTRUCTION;
    }
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        let ref mut fresh111 = (*fi.offset(3 as i32 as isize)).nexti;
        *fresh111 = namespace_chain;
        namespace_chain = 0 as *mut INSTRUCTION;
        list_prepend(def, bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32));
    }
    let ref mut fresh112 = (*fi.offset(1 as i32 as isize)).x.xi;
    *fresh112 = (*def).nexti;
    let ref mut fresh113 = (*fi.offset(1 as i32 as isize)).d.di;
    *fresh113 = (*def).d.di;
    (*fi.offset(2 as i32 as isize)).source_line = (*fi).source_line;
    (*fi.offset(2 as i32 as isize)).x.xl = lastline as i64;
    (*fi).nexti = (*def).nexti;
    bcfree(def);
    list_append(rule_list, fi.offset(1 as i32 as isize));
    func_use((*thisfunc).sub.nodep.name, defref::FUNC_DEFINE);
    remove_params(thisfunc);
    return fi;
}
unsafe extern "C" fn install_function(
    mut fname: *mut i8,
    mut fi: *mut INSTRUCTION,
    mut plist: *mut INSTRUCTION,
) -> i32 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut pcount: i32 = 0 as i32;
    r = lookup(fname);
    if !r.is_null() {
        error_ln(
            (*fi).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"function name `%s' previously defined\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fname,
        );
        return -(1 as i32);
    }
    if !plist.is_null() {
        pcount = ((*(*plist).d.di).x.xl + 1 as i32 as i64) as i32;
    }
    f = install_symbol(fname, nodevals::Node_func);
    if (*f).sub.nodep.name != fname {
        fname = (*f).sub.nodep.name;
    }
    (*fi).x.xn = f;
    (*f).sub.nodep.l.ll = pcount as i64;
    (*f).sub.nodep.r.iptr = fi;
    (*f).sub.nodep.rn = 0 as *mut exp_node;
    if pcount > 0 as i32 {
        let mut pnames: *mut *mut i8 = 0 as *mut *mut i8;
        pnames = check_params(fname, pcount, plist);
        (*f).sub.nodep.rn = make_params(pnames, pcount);
        pma_free(pnames as *mut libc::c_void);
        install_params(f);
    }
    return 0 as i32;
}
unsafe extern "C" fn check_params(
    mut fname: *mut i8,
    mut pcount: i32,
    mut list: *mut INSTRUCTION,
) -> *mut *mut i8 {
    let mut p: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut np: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut pnames: *mut *mut i8 = 0 as *mut *mut i8;
    pnames = emalloc_real(
        (pcount as u64).wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        b"check_params\0" as *const u8 as *const i8,
        b"pnames\0" as *const u8 as *const i8,
        b"awkgram.y\0" as *const u8 as *const i8,
        5102 as i32,
    ) as *mut *mut i8;
    i = 0 as i32;
    p = (*list).nexti;
    while !p.is_null() {
        np = (*p).nexti;
        name = (*p).d.name;
        (*p).d.name = 0 as *mut i8;
        if strcmp(name, fname) == 0 as i32 {
            error_ln(
                (*p).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"function `%s': cannot use function name as parameter name\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
            );
        } else if is_std_var(name) != 0 {
            error_ln(
                (*p).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"function `%s': cannot use special variable `%s' as a function parameter\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                name,
            );
        } else if !(strchr(name, ':' as i32)).is_null() {
            error_ln(
                (*p).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"function `%s': parameter `%s' cannot contain a namespace\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                fname,
                name,
            );
        }
        j = 0 as i32;
        while j < i {
            if strcmp(name, *pnames.offset(j as isize)) == 0 as i32 {
                error_ln(
                    (*p).source_line as i32,
                    dcgettext(
                        0 as *const i8,
                        b"function `%s': parameter #%d, `%s', duplicates parameter #%d\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    fname,
                    i + 1 as i32,
                    name,
                    j + 1 as i32,
                );
            }
            j += 1;
            j;
        }
        let ref mut fresh114 = *pnames.offset(i as isize);
        *fresh114 = name;
        bcfree(p);
        i += 1;
        i;
        p = np;
    }
    bcfree(list);
    return pnames;
}
static mut ftable: [*mut fdesc; 1021] = [0 as *const fdesc as *mut fdesc; 1021];
unsafe extern "C" fn func_use(mut name: *const i8, mut how: defref) {
    let mut current_block: u64;
    let mut fp_0: *mut fdesc = 0 as *mut fdesc;
    let mut len: i32 = 0;
    let mut ind: i32 = 0;
    len = strlen(name) as i32;
    ind = hash
        .expect(
            "non-null function pointer",
        )(name, len as size_t, 1021 as i32 as u64, 0 as *mut size_t) as i32;
    fp_0 = ftable[ind as usize];
    loop {
        if fp_0.is_null() {
            current_block = 7095457783677275021;
            break;
        }
        if strcmp((*fp_0).name, name) == 0 as i32 {
            current_block = 14680491677114005255;
            break;
        }
        fp_0 = (*fp_0).next;
    }
    match current_block {
        7095457783677275021 => {
            fp_0 = ezalloc_real(
                ::core::mem::size_of::<fdesc>() as u64,
                b"func_use\0" as *const u8 as *const i8,
                b"fp\0" as *const u8 as *const i8,
                b"awkgram.y\0" as *const u8 as *const i8,
                5171 as i32,
            ) as *mut fdesc;
            (*fp_0).name = emalloc_real(
                (len + 1 as i32) as size_t,
                b"func_use\0" as *const u8 as *const i8,
                b"fp->name\0" as *const u8 as *const i8,
                b"awkgram.y\0" as *const u8 as *const i8,
                5172 as i32,
            ) as *mut i8;
            strcpy((*fp_0).name, name);
            (*fp_0).next = ftable[ind as usize];
            ftable[ind as usize] = fp_0;
        }
        _ => {}
    }
    if how as u32 == defref::FUNC_DEFINE as i32 as u32 {
        (*fp_0).defined += 1;
        (*fp_0).defined;
    } else if how as u32 == defref::FUNC_EXT as i32 as u32 {
        (*fp_0).defined += 1;
        (*fp_0).defined;
        (*fp_0).extension += 1;
        (*fp_0).extension;
    } else {
        (*fp_0).used += 1;
        (*fp_0).used;
    };
}
#[no_mangle]
pub unsafe extern "C" fn track_ext_func(mut name: *const i8) {
    func_use(name, defref::FUNC_EXT);
}
unsafe extern "C" fn check_funcs() {
    let mut fp_0: *mut fdesc = 0 as *mut fdesc;
    let mut next: *mut fdesc = 0 as *mut fdesc;
    let mut i: i32 = 0;
    if !(in_main_context() == 0) {
        i = 0 as i32;
        while i < 1021 as i32 {
            fp_0 = ftable[i as usize];
            while !fp_0.is_null() {
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                    && (*fp_0).extension == 0
                {
                    if (*fp_0).defined as i32 == 0 as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const i8,
                            5215 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function `%s' called but never defined\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            (*fp_0).name,
                        );
                    }
                    if (*fp_0).used as i32 == 0 as i32 {
                        (set_loc
                            as unsafe extern "C" fn(
                                *const i8,
                                i32,
                            ) -> ())(
                            b"awkgram.y\0" as *const u8 as *const i8,
                            5219 as i32,
                        );
                        (Some(lintfunc.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(
                            dcgettext(
                                0 as *const i8,
                                b"function `%s' defined but never called directly\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*fp_0).name,
                        );
                    }
                }
                fp_0 = (*fp_0).next;
            }
            i += 1;
            i;
        }
    }
    i = 0 as i32;
    while i < 1021 as i32 {
        fp_0 = ftable[i as usize];
        while !fp_0.is_null() {
            next = (*fp_0).next;
            pma_free((*fp_0).name as *mut libc::c_void);
            pma_free(fp_0 as *mut libc::c_void);
            fp_0 = next;
        }
        ftable[i as usize] = 0 as *mut fdesc;
        i += 1;
        i;
    }
}
unsafe extern "C" fn param_sanity(mut arglist: *mut INSTRUCTION) {
    let mut argl: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut arg: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i: i32 = 1 as i32;
    if arglist.is_null() {
        return;
    }
    argl = (*arglist).nexti;
    while !argl.is_null() {
        arg = (*argl).d.di;
        if (*arg).opcode as u32 == opcodeval::Op_match_rec as i32 as u32 {
            warning_ln(
                (*arg).source_line as i32,
                dcgettext(
                    0 as *const i8,
                    b"regexp constant for parameter #%d yields boolean value\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                i,
            );
        }
        argl = (*arg).nexti;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn variable(
    mut location: i32,
    mut name: *mut i8,
    mut type_0: NODETYPE,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = lookup(name);
    if !r.is_null() {
        if (*r).type_0 as u32 == nodevals::Node_func as i32 as u32
            || (*r).type_0 as u32 == nodevals::Node_ext_func as i32 as u32
        {
            error_ln(
                location,
                dcgettext(
                    0 as *const i8,
                    b"function `%s' called with space between name and `(',\nor used as a variable or an array\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*r).sub.nodep.name,
            );
        }
    } else {
        return install_symbol(name, type_0)
    }
    pma_free(name as *mut libc::c_void);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn make_regnode(
    mut type_0: NODETYPE,
    mut exp: *mut NODE,
) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    n = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !n.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(n as *mut block_item))
            .freep;
    } else {
        n = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(n as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<NODE>() as u64);
    (*n).type_0 = type_0;
    (*n).flags = flagvals::MALLOC;
    if type_0 as u32 == nodevals::Node_regex as i32 as u32 {
        (*n).sub.nodep.r.preg[0 as i32 as usize] = make_regexp(
            (*exp).sub.val.sp,
            (*exp).sub.val.slen,
            0 as i32 != 0,
            1 as i32 != 0,
            0 as i32 != 0,
        );
        if ((*n).sub.nodep.r.preg[0 as i32 as usize]).is_null() {
            let ref mut fresh115 = (*(n as *mut block_item)).freep;
            *fresh115 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = n as *mut block_item;
            return 0 as *mut NODE;
        }
        (*n).sub.nodep.x.extra = exp;
        (*n).sub.nodep.reflags = reflagvals::CONSTANT;
    }
    return n;
}
unsafe extern "C" fn mk_rexp(mut list: *mut INSTRUCTION) -> *mut NODE {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    ip = (*list).nexti;
    if ip == (*list).d.di && (*ip).opcode as u32 == opcodeval::Op_match_rec as i32 as u32
    {
        (*ip).opcode = opcodeval::Op_push_re;
    } else if !(ip == (*list).d.di
        && (*ip).opcode as u32 == opcodeval::Op_push_re as i32 as u32)
    {
        ip = bcalloc(opcodeval::Op_push_re, 1 as i32, 0 as i32);
        (*ip).d.dn = make_regnode(nodevals::Node_dynregex, 0 as *mut NODE);
        (*ip).nexti = (*(*list).d.di).nexti;
        (*(*list).d.di).nexti = ip;
        (*list).d.di = ip;
    }
    return (*ip).d.dn;
}
unsafe extern "C" fn isnoeffect(mut type_0: OPCODE) -> i32 {
    match type_0 as u32 {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 16 | 13 | 11 | 12 | 22 | 24 | 39 | 41
        | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 50 | 49 | 25 | 72 => return 1 as i32,
        38 | 40 | 75 | 78 | 80 | 86 | 106 => return 1 as i32,
        _ => {}
    }
    return 0 as i32;
}
unsafe extern "C" fn make_assignable(mut ip: *mut INSTRUCTION) -> *mut INSTRUCTION {
    match (*ip).opcode as u32 {
        75 => {
            (*ip).opcode = opcodeval::Op_push_lhs;
            return ip;
        }
        24 => {
            (*ip).opcode = opcodeval::Op_field_spec_lhs;
            return ip;
        }
        16 => {
            (*ip).opcode = opcodeval::Op_subscript_lhs;
            return ip;
        }
        97 => return ip,
        _ => {}
    }
    return 0 as *mut INSTRUCTION;
}
#[no_mangle]
pub unsafe extern "C" fn stopme(mut nargs: i32) -> *mut NODE {
    return make_number.expect("non-null function pointer")(0.0f64);
}
unsafe extern "C" fn dumpintlstr(mut str: *const i8, mut len: size_t) {
    let mut cp: *mut i8 = 0 as *mut i8;
    if !source.is_null() {
        cp = source;
        while *cp.offset(0 as i32 as isize) as i32 == '.' as i32
            && *cp.offset(1 as i32 as isize) as i32 == '/' as i32
        {
            cp = cp.offset(2 as i32 as isize);
        }
        printf(b"#: %s:%d\n\0" as *const u8 as *const i8, cp, sourceline);
    }
    printf(b"msgid \0" as *const u8 as *const i8);
    pp_string_fp(
        Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
        stdout,
        str,
        len,
        '"' as i32,
        1 as i32 != 0,
    );
    putchar('\n' as i32);
    printf(b"msgstr \"\"\n\n\0" as *const u8 as *const i8);
    fflush(stdout);
}
unsafe extern "C" fn dumpintlstr2(
    mut str1: *const i8,
    mut len1: size_t,
    mut str2: *const i8,
    mut len2: size_t,
) {
    let mut cp: *mut i8 = 0 as *mut i8;
    if !source.is_null() {
        cp = source;
        while *cp.offset(0 as i32 as isize) as i32 == '.' as i32
            && *cp.offset(1 as i32 as isize) as i32 == '/' as i32
        {
            cp = cp.offset(2 as i32 as isize);
        }
        printf(b"#: %s:%d\n\0" as *const u8 as *const i8, cp, sourceline);
    }
    printf(b"msgid \0" as *const u8 as *const i8);
    pp_string_fp(
        Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
        stdout,
        str1,
        len1,
        '"' as i32,
        1 as i32 != 0,
    );
    putchar('\n' as i32);
    printf(b"msgid_plural \0" as *const u8 as *const i8);
    pp_string_fp(
        Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
        stdout,
        str2,
        len2,
        '"' as i32,
        1 as i32 != 0,
    );
    putchar('\n' as i32);
    printf(b"msgstr[0] \"\"\nmsgstr[1] \"\"\n\n\0" as *const u8 as *const i8);
    fflush(stdout);
}
unsafe extern "C" fn mk_binary(
    mut s1: *mut INSTRUCTION,
    mut s2: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut current_block: u64;
    let mut ip1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lint_plus: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut res: libc::c_double = 0.;
    ip2 = (*s2).nexti;
    if (*s2).d.di == ip2 && (*ip2).opcode as u32 == opcodeval::Op_push_i as i32 as u32 {
        ip1 = (*s1).nexti;
        if do_optimize as i32 != 0 && ip1 == (*s1).d.di
            && (*ip1).opcode as u32 == opcodeval::Op_push_i as i32 as u32
            && (*(*ip1).d.dn).flags as u32
                & (flagvals::MPFN as i32 | flagvals::MPZN as i32
                    | flagvals::STRCUR as i32 | flagvals::STRING as i32) as u32
                == 0 as i32 as u32
            && (*(*ip2).d.dn).flags as u32
                & (flagvals::MPFN as i32 | flagvals::MPZN as i32
                    | flagvals::STRCUR as i32 | flagvals::STRING as i32) as u32
                == 0 as i32 as u32
        {
            let mut n1: *mut NODE = (*ip1).d.dn;
            let mut n2: *mut NODE = (*ip2).d.dn;
            res = (*force_number(n1)).sub.val.fltnum;
            force_number(n2);
            match (*op).opcode as u32 {
                1 => {
                    current_block = 12532600039944777921;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                3 => {
                    current_block = 5858530316265163821;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                5 => {
                    current_block = 3782427828431648317;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                7 => {
                    current_block = 4508456222488031579;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                9 => {
                    current_block = 10521348832502884984;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                11 => {
                    current_block = 14384785785743493926;
                    match current_block {
                        14384785785743493926 => {
                            res = calc_exp(res, (*n2).sub.val.fltnum);
                            current_block = 6057473163062296781;
                        }
                        5858530316265163821 => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res /= (*n2).sub.val.fltnum;
                                current_block = 6057473163062296781;
                            }
                        }
                        12532600039944777921 => {
                            res *= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        4508456222488031579 => {
                            res += (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        10521348832502884984 => {
                            res -= (*n2).sub.val.fltnum;
                            current_block = 6057473163062296781;
                        }
                        _ => {
                            if (*n2).flags as u32 & flagvals::NUMBER as i32 as u32
                                != 0 as i32 as u32 && (*n2).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                res = fmod(res, (*n2).sub.val.fltnum);
                                current_block = 6057473163062296781;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).opcode = opcodeval::Op_push_i;
                            (*op).d.dn = make_number
                                .expect("non-null function pointer")(res);
                            unref(n1);
                            unref(n2);
                            bcfree(ip1);
                            bcfree(ip2);
                            bcfree(s1);
                            bcfree(s2);
                            return list_create(op);
                        }
                    }
                }
                _ => {}
            }
        } else {
            match (*op).opcode as u32 {
                1 => {
                    current_block = 11082864473178571682;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                3 => {
                    current_block = 9721778599664766590;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                5 => {
                    current_block = 13789447811171709891;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                7 => {
                    current_block = 5044529757515784355;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                9 => {
                    current_block = 11397603411307666477;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                11 => {
                    current_block = 14472253109775686402;
                    match current_block {
                        14472253109775686402 => {
                            (*op).opcode = opcodeval::Op_exp_i;
                            current_block = 980989089337379490;
                        }
                        13789447811171709891 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted in `%%'\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_mod_i;
                                current_block = 980989089337379490;
                            }
                        }
                        9721778599664766590 => {
                            if (*(*ip2).d.dn).flags as u32
                                & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
                                && (*(*ip2).d.dn).sub.val.fltnum == 0.0f64
                            {
                                error_ln(
                                    (*op).source_line as i32,
                                    dcgettext(
                                        0 as *const i8,
                                        b"division by zero attempted\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_quotient_i;
                                current_block = 980989089337379490;
                            }
                        }
                        11082864473178571682 => {
                            (*op).opcode = opcodeval::Op_times_i;
                            current_block = 980989089337379490;
                        }
                        11397603411307666477 => {
                            (*op).opcode = opcodeval::Op_minus_i;
                            current_block = 980989089337379490;
                        }
                        _ => {
                            if do_flags as u32
                                & (do_flag_values::DO_LINT_INVALID as i32
                                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                            {
                                current_block = 15419492165577754711;
                            } else {
                                (*op).opcode = opcodeval::Op_plus_i;
                                current_block = 980989089337379490;
                            }
                        }
                    }
                    match current_block {
                        15419492165577754711 => {}
                        _ => {
                            (*op).d.dn = (*ip2).d.dn;
                            bcfree(ip2);
                            bcfree(s2);
                            return list_append(s1, op);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    list_merge(s1, s2);
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0 && (*op).opcode as u32 == opcodeval::Op_plus as i32 as u32
    {
        lint_plus = bcalloc(opcodeval::Op_lint_plus, 1 as i32, 0 as i32);
        list_append(s1, lint_plus);
    }
    return list_append(s1, op);
}
unsafe extern "C" fn mk_boolean(
    mut left: *mut INSTRUCTION,
    mut right: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut opc: OPCODE = opcodeval::Op_illegal;
    let mut final_opc: OPCODE = opcodeval::Op_illegal;
    opc = (*op).opcode;
    final_opc = (if opc as u32 == opcodeval::Op_or as i32 as u32 {
        opcodeval::Op_or_final as i32
    } else {
        opcodeval::Op_and_final as i32
    }) as OPCODE;
    add_lint(right, lintvals::LINT_assign_in_cond);
    tp = (*left).d.di;
    if (*tp).opcode as u32 != final_opc as u32 {
        list_append(right, bcalloc(final_opc, 1 as i32, 0 as i32));
        add_lint(left, lintvals::LINT_assign_in_cond);
        list_append(left, op);
        (*(*left).d.di).d.di = (*right).d.di;
        (*(*left).d.di).x.xi = (*left).d.di;
        (*(*right).d.di).x.xi = (*left).d.di;
    } else {
        let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        (*op).opcode = final_opc;
        list_append(right, op);
        (*op).x.xi = tp;
        (*tp).opcode = opc;
        (*tp).d.di = op;
        ip = (*tp).x.xi;
        loop {
            (*ip).d.di = op;
            if (*ip).x.xi == ip {
                break;
            }
            ip = (*ip).x.xi;
        }
    }
    return list_merge(left, right);
}
unsafe extern "C" fn mk_condition(
    mut cond: *mut INSTRUCTION,
    mut ifp: *mut INSTRUCTION,
    mut true_branch: *mut INSTRUCTION,
    mut elsep: *mut INSTRUCTION,
    mut false_branch: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut setup_else_part: bool = 1 as i32 != 0;
    if false_branch.is_null() {
        false_branch = list_create(bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32));
        if elsep.is_null() {
            setup_else_part = 0 as i32 != 0;
        }
    } else if (*(*false_branch).d.di).opcode as u32 != opcodeval::Op_no_op as i32 as u32
    {
        list_append(false_branch, bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32));
    }
    if setup_else_part {
        if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
            list_prepend(false_branch, elsep);
            (*(*false_branch).nexti).x.xi = (*false_branch).d.di;
            list_prepend(
                false_branch,
                bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
            );
        } else {
            bcfree(elsep);
        }
    }
    list_prepend(false_branch, bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32));
    (*(*false_branch).nexti).d.di = (*false_branch).d.di;
    add_lint(cond, lintvals::LINT_assign_in_cond);
    ip = list_append(cond, bcalloc(opcodeval::Op_jmp_false, 1 as i32, 0 as i32));
    (*(*ip).d.di).d.di = (*(*false_branch).nexti).nexti;
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        list_prepend(ip, ifp);
        list_append(ip, bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32));
        (*(*ip).nexti).d.di = (*ip).d.di;
        (*(*ip).nexti).x.xi = (*false_branch).nexti;
    } else {
        bcfree(ifp);
    }
    if !true_branch.is_null() {
        list_merge(ip, true_branch);
    }
    return list_merge(ip, false_branch);
}
unsafe extern "C" fn find_line(mut pattern: *mut INSTRUCTION, mut what: defline) -> i32 {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lineno: i32 = 0 as i32;
    ip = (*pattern).nexti;
    while !ip.is_null() {
        if what as u32 == defline::LAST_LINE as i32 as u32 {
            if (*ip).source_line as i32 > lineno {
                lineno = (*ip).source_line as i32;
            }
        } else if (*ip).source_line as i32 > 0 as i32
            && (lineno == 0 as i32 || ((*ip).source_line as i32) < lineno)
        {
            lineno = (*ip).source_line as i32;
        }
        if ip == (*pattern).d.di {
            break;
        }
        ip = (*ip).nexti;
    }
    return lineno;
}
unsafe extern "C" fn append_rule(
    mut pattern: *mut INSTRUCTION,
    mut action: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if rule != defrule::Rule as i32 {
        rp = pattern;
        if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
            let ref mut fresh116 = (*rp.offset(3 as i32 as isize)).nexti;
            *fresh116 = namespace_chain;
            namespace_chain = 0 as *mut INSTRUCTION;
            list_append(action, bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32));
        }
        let ref mut fresh117 = (*rp.offset(1 as i32 as isize)).x.xi;
        *fresh117 = (*action).nexti;
        let ref mut fresh118 = (*rp.offset(1 as i32 as isize)).d.di;
        *fresh118 = (*action).d.di;
        (*rp.offset(2 as i32 as isize)).source_line = (*pattern).source_line;
        (*rp.offset(2 as i32 as isize)).x.xl = lastline as i64;
        ip = list_prepend(action, rp);
        if !interblock_comment.is_null() {
            ip = list_prepend(ip, interblock_comment);
            interblock_comment = 0 as *mut INSTRUCTION;
        }
    } else {
        rp = bcalloc(opcodeval::Op_rule, 4 as i32, 0 as i32);
        (*rp).x.xl = defrule::Rule as i32 as i64;
        (*rp).d.name = source;
        tp = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
        if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
            let ref mut fresh119 = (*rp.offset(3 as i32 as isize)).nexti;
            *fresh119 = namespace_chain;
            namespace_chain = 0 as *mut INSTRUCTION;
        }
        if pattern.is_null() {
            if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
                list_prepend(
                    action,
                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                );
            }
            let ref mut fresh120 = (*rp.offset(1 as i32 as isize)).x.xi;
            *fresh120 = (*action).nexti;
            let ref mut fresh121 = (*rp.offset(1 as i32 as isize)).d.di;
            *fresh121 = tp;
            (*rp.offset(2 as i32 as isize)).source_line = firstline as libc::c_short;
            (*rp.offset(2 as i32 as isize)).x.xl = lastline as i64;
            (*rp).source_line = firstline as libc::c_short;
            ip = list_prepend(list_append(action, tp), rp);
        } else {
            list_append(pattern, bcalloc(opcodeval::Op_jmp_false, 1 as i32, 0 as i32));
            (*(*pattern).d.di).d.di = tp;
            (*rp.offset(2 as i32 as isize)).source_line = find_line(
                pattern,
                defline::FIRST_LINE,
            ) as libc::c_short;
            (*rp).source_line = (*rp.offset(2 as i32 as isize)).source_line;
            if action.is_null() {
                (*rp.offset(2 as i32 as isize)).x.xl = find_line(
                    pattern,
                    defline::LAST_LINE,
                ) as i64;
                action = list_create(
                    bcalloc(opcodeval::Op_K_print_rec, 1 as i32, 0 as i32),
                );
                if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
                    action = list_prepend(
                        action,
                        bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                    );
                }
            } else {
                (*rp.offset(2 as i32 as isize)).x.xl = lastline as i64;
            }
            if !interblock_comment.is_null() {
                pattern = list_prepend(pattern, interblock_comment);
                interblock_comment = 0 as *mut INSTRUCTION;
            }
            if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
                pattern = list_prepend(
                    pattern,
                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                );
                action = list_prepend(
                    action,
                    bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32),
                );
            }
            let ref mut fresh122 = (*rp.offset(1 as i32 as isize)).x.xi;
            *fresh122 = (*action).nexti;
            let ref mut fresh123 = (*rp.offset(1 as i32 as isize)).d.di;
            *fresh123 = tp;
            ip = list_append(list_merge(list_prepend(pattern, rp), action), tp);
        }
    }
    list_append(rule_list, rp.offset(1 as i32 as isize));
    if (rule_block[rule as usize]).is_null() {
        rule_block[rule as usize] = ip;
    } else {
        list_merge(rule_block[rule as usize], ip);
    }
    return rule_block[rule as usize];
}
unsafe extern "C" fn mk_assignment(
    mut lhs: *mut INSTRUCTION,
    mut rhs: *mut INSTRUCTION,
    mut op: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    tp = (*lhs).d.di;
    match (*tp).opcode as u32 {
        24 => {
            (*tp).opcode = opcodeval::Op_field_spec_lhs;
        }
        16 => {
            (*tp).opcode = opcodeval::Op_subscript_lhs;
        }
        75 | 80 => {
            (*tp).opcode = opcodeval::Op_push_lhs;
        }
        97 => {
            yyerror(
                dcgettext(
                    0 as *const i8,
                    b"cannot assign a value to the result of a field post-increment expression\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        _ => {
            yyerror(
                dcgettext(
                    0 as *const i8,
                    b"invalid target of assignment (opcode %s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                opcode2str((*tp).opcode),
            );
        }
    }
    (*tp).x.xl = ((*op).opcode as u32 != opcodeval::Op_assign as i32 as u32) as i32
        as i64;
    if !rhs.is_null() {
        ip = list_merge(rhs, lhs);
    } else {
        ip = lhs;
    }
    if (*tp).opcode as u32 != opcodeval::Op_field_spec_lhs as i32 as u32
        || (*op).opcode as u32 != opcodeval::Op_assign as i32 as u32
    {
        list_append(ip, op);
    }
    if (*tp).opcode as u32 == opcodeval::Op_push_lhs as i32 as u32
        && (*(*tp).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
        && ((*(*tp).d.dn).sub.nodep.x.aptr).is_some()
    {
        (*tp).x.xl = 0 as i32 as i64;
        list_append(ip, bcalloc(opcodeval::Op_var_assign, 1 as i32, 0 as i32));
        (*(*ip).d.di).x.aptr = (*(*tp).d.dn).sub.nodep.x.aptr;
    } else if (*tp).opcode as u32 == opcodeval::Op_field_spec_lhs as i32 as u32 {
        if (*op).opcode as u32 == opcodeval::Op_assign as i32 as u32 {
            bcfree(op);
            (*tp).opcode = opcodeval::Op_store_field_exp;
        } else {
            list_append(ip, bcalloc(opcodeval::Op_field_assign, 1 as i32, 0 as i32));
            (*(*ip).d.di).x.aptr = None;
            (*tp).d.di = (*ip).d.di;
        }
    } else if (*tp).opcode as u32 == opcodeval::Op_subscript_lhs as i32 as u32 {
        list_append(ip, bcalloc(opcodeval::Op_subscript_assign, 1 as i32, 0 as i32));
    }
    return ip;
}
unsafe extern "C" fn optimize_assignment(mut exp: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut i1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut i3: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    i2 = 0 as *mut INSTRUCTION;
    i1 = (*exp).d.di;
    if (*i1).opcode as u32 == opcodeval::Op_store_field_exp as i32 as u32 {
        (*i1).opcode = opcodeval::Op_store_field;
        return exp;
    }
    if (*i1).opcode as u32 != opcodeval::Op_assign as i32 as u32
        && (*i1).opcode as u32 != opcodeval::Op_field_assign as i32 as u32
    {
        return list_append(exp, bcalloc(opcodeval::Op_pop, 1 as i32, 0 as i32));
    }
    i2 = (*exp).nexti;
    while i2 != i1 {
        match (*i2).opcode as u32 {
            13 => {
                if (*(*i2).nexti).opcode as u32 == opcodeval::Op_push_lhs as i32 as u32
                    && (*i2).d.dl & 2 as i32 as i64 != 0 as i32 as i64
                    && (*(*i2).nexti).d.dn == (*(*exp).nexti).d.dn
                    && (*(*i2).nexti).nexti == i1
                    && (*i1).opcode as u32 == opcodeval::Op_assign as i32 as u32
                {
                    i3 = (*(*exp).nexti).nexti;
                    while i3 != i2 {
                        if (*i3).opcode as u32 == opcodeval::Op_push_lhs as i32 as u32
                            && (*i3).d.dn == (*(*i2).nexti).d.dn
                            || (*i3).opcode as u32
                                == opcodeval::Op_func_call as i32 as u32
                        {
                            return list_append(
                                exp,
                                bcalloc(opcodeval::Op_pop, 1 as i32, 0 as i32),
                            );
                        }
                        i3 = (*i3).nexti;
                    }
                    i3 = (*exp).nexti;
                    (*exp).nexti = (*i3).nexti;
                    bcfree(i3);
                    (*i2).x.xl -= 1;
                    if (*i2).x.xl == 1 as i32 as i64 {
                        (*i2).opcode = opcodeval::Op_no_op;
                    }
                    i3 = (*i2).nexti;
                    (*i3).opcode = opcodeval::Op_assign_concat;
                    (*i3).nexti = 0 as *mut exp_instruction;
                    bcfree(i1);
                    (*exp).d.di = i3;
                    return exp;
                }
            }
            80 => {
                if (*(*(*i2).nexti).nexti).opcode as u32
                    == opcodeval::Op_subscript_lhs as i32 as u32
                {
                    i3 = (*(*i2).nexti).nexti;
                    if (*i3).d.dl == 1 as i32 as i64 && (*i3).nexti == i1
                        && (*i1).opcode as u32 == opcodeval::Op_assign as i32 as u32
                    {
                        (*i3).opcode = opcodeval::Op_store_sub;
                        (*i3).d.dn = (*i2).d.dn;
                        (*i3).x.xl = 1 as i32 as i64;
                        (*i3).nexti = 0 as *mut exp_instruction;
                        (*i2).opcode = opcodeval::Op_no_op;
                        bcfree(i1);
                        (*exp).d.di = i3;
                        return exp;
                    }
                }
            }
            82 => {
                if (*i2).nexti == i1
                    && (*i1).opcode as u32 == opcodeval::Op_assign as i32 as u32
                {
                    (*i2).opcode = opcodeval::Op_store_var;
                    (*i2).nexti = 0 as *mut exp_instruction;
                    bcfree(i1);
                    (*exp).d.di = i2;
                    i3 = (*exp).nexti;
                    if (*i3).opcode as u32 == opcodeval::Op_push_i as i32 as u32
                        && (*(*i3).d.dn).flags as u32 & flagvals::INTLSTR as i32 as u32
                            == 0 as i32 as u32 && (*i3).nexti == i2
                    {
                        (*i2).x.xn = (*i3).d.dn;
                        bcfree(i3);
                        (*exp).nexti = i2;
                    } else {
                        (*i2).x.xn = 0 as *mut NODE;
                    }
                    return exp;
                }
            }
            _ => {}
        }
        i2 = (*i2).nexti;
    }
    return list_append(exp, bcalloc(opcodeval::Op_pop, 1 as i32, 0 as i32));
}
unsafe extern "C" fn mk_getline(
    mut op: *mut INSTRUCTION,
    mut var: *mut INSTRUCTION,
    mut redir: *mut INSTRUCTION,
    mut redirtype: i32,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut asgn: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if redir.is_null() {
        let mut sline: i32 = (*op).source_line as i32;
        bcfree(op);
        op = bcalloc(opcodeval::Op_K_getline, 2 as i32, sline);
        let ref mut fresh124 = (*op.offset(1 as i32 as isize)).x.xi;
        *fresh124 = ip_endfile;
        let ref mut fresh125 = (*op.offset(1 as i32 as isize)).d.di;
        *fresh125 = ip_beginfile;
    }
    if !var.is_null() {
        tp = make_assignable((*var).d.di);
        if (*tp).opcode as u32 == opcodeval::Op_push_lhs as i32 as u32
            && (*(*tp).d.dn).type_0 as u32 == nodevals::Node_var as i32 as u32
            && ((*(*tp).d.dn).sub.nodep.x.aptr).is_some()
        {
            asgn = bcalloc(opcodeval::Op_var_assign, 1 as i32, 0 as i32);
            (*asgn).d.dl = (*op).opcode as i64;
            (*asgn).x.aptr = (*(*tp).d.dn).sub.nodep.x.aptr;
        } else if (*tp).opcode as u32 == opcodeval::Op_field_spec_lhs as i32 as u32 {
            asgn = bcalloc(opcodeval::Op_field_assign, 1 as i32, 0 as i32);
            (*asgn).d.dl = (*op).opcode as i64;
            (*asgn).x.aptr = None;
            (*tp).d.di = asgn;
        } else if (*tp).opcode as u32 == opcodeval::Op_subscript_lhs as i32 as u32 {
            asgn = bcalloc(opcodeval::Op_subscript_assign, 1 as i32, 0 as i32);
            (*asgn).d.dl = (*op).opcode as i64;
        }
        if !redir.is_null() {
            ip = list_merge(redir, var);
            list_append(ip, op);
        } else {
            ip = list_append(var, op);
        }
    } else if !redir.is_null() {
        ip = list_append(redir, op);
    } else {
        ip = list_create(op);
    }
    (*op).x.xl = (var != 0 as *mut libc::c_void as *mut INSTRUCTION) as i32 as i64;
    (*op).d.dl = (if !redir.is_null() {
        redirtype
    } else {
        redirval::redirect_none as i32
    }) as i64;
    return if asgn.is_null() { ip } else { list_append(ip, asgn) };
}
unsafe extern "C" fn mk_for_loop(
    mut forp: *mut INSTRUCTION,
    mut init: *mut INSTRUCTION,
    mut cond: *mut INSTRUCTION,
    mut incr: *mut INSTRUCTION,
    mut body: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tbreak: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut tcont: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut jmp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut pp_cond: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ret: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    tbreak = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
    if !cond.is_null() {
        add_lint(cond, lintvals::LINT_assign_in_cond);
        pp_cond = (*cond).nexti;
        ip = cond;
        list_append(ip, bcalloc(opcodeval::Op_jmp_false, 1 as i32, 0 as i32));
        (*(*ip).d.di).d.di = tbreak;
    } else {
        pp_cond = bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32);
        ip = list_create(pp_cond);
    }
    if !init.is_null() {
        ip = list_merge(init, ip);
    }
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        list_append(ip, bcalloc(opcodeval::Op_exec_count, 1 as i32, 0 as i32));
        let ref mut fresh126 = (*forp.offset(1 as i32 as isize)).d.di;
        *fresh126 = pp_cond;
        let ref mut fresh127 = (*forp.offset(1 as i32 as isize)).x.xi;
        *fresh127 = (*ip).d.di;
    }
    if !body.is_null() {
        list_merge(ip, body);
    }
    jmp = bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32);
    (*jmp).d.di = pp_cond;
    if incr.is_null() {
        tcont = jmp;
    } else {
        tcont = (*incr).nexti;
        list_merge(ip, incr);
    }
    list_append(ip, jmp);
    ret = list_append(ip, tbreak);
    fix_break_continue(ret, tbreak, tcont);
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        (*forp).x.xi = tbreak;
        (*forp).d.di = tcont;
        ret = list_prepend(ret, forp);
    }
    return ret;
}
unsafe extern "C" fn add_lint(mut list: *mut INSTRUCTION, mut linttype: LINTTYPE) {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut no_effect: bool = 1 as i32 != 0;
    match linttype as u32 {
        1 => {
            ip = (*list).d.di;
            if (*ip).opcode as u32 == opcodeval::Op_var_assign as i32 as u32
                || (*ip).opcode as u32 == opcodeval::Op_field_assign as i32 as u32
            {
                ip = (*list).nexti;
                while (*ip).nexti != (*list).d.di {
                    ip = (*ip).nexti;
                }
            }
            if (*ip).opcode as u32 == opcodeval::Op_assign as i32 as u32
                || (*ip).opcode as u32 == opcodeval::Op_assign_concat as i32 as u32
            {
                list_append(list, bcalloc(opcodeval::Op_lint, 1 as i32, 0 as i32));
                (*(*list).d.di).d.dl = linttype as i64;
            }
        }
        2 => {
            if (*(*list).d.di).opcode as u32 == opcodeval::Op_pop as i32 as u32
                && (*list).nexti != (*list).d.di
            {
                let mut line: i32 = 0 as i32;
                ip = (*list).nexti;
                while (*ip).nexti != (*list).d.di {
                    if (*ip).source_line as i32 != 0 as i32 {
                        line = (*ip).source_line as i32;
                    }
                    no_effect = no_effect as i32 != 0 && isnoeffect((*ip).opcode) != 0;
                    ip = (*ip).nexti;
                }
                no_effect = no_effect as i32 != 0 && isnoeffect((*ip).opcode) != 0;
                if do_flags as u32
                    & (do_flag_values::DO_LINT_INVALID as i32
                        | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
                {
                    if no_effect {
                        if (*ip).source_line as i32 != 0 as i32 {
                            line = (*ip).source_line as i32;
                        }
                        lintwarn_ln(
                            line,
                            dcgettext(
                                0 as *const i8,
                                b"statement has no effect\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn mk_expression_list(
    mut list: *mut INSTRUCTION,
    mut s1: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut r: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if list.is_null() {
        list = bcalloc(opcodeval::Op_list, 1 as i32, 0 as i32);
        (*list).nexti = s1;
        (*list).d.di = (*s1).d.di;
        return list;
    }
    r = (*list).d.di;
    (*r).nexti = s1;
    (*list).d.di = (*s1).d.di;
    return list;
}
unsafe extern "C" fn count_expressions(
    mut list: *mut *mut INSTRUCTION,
    mut isarg: bool,
) -> i32 {
    let mut expr: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut r: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut count: i32 = 0 as i32;
    if (*list).is_null() {
        return 0 as i32;
    }
    expr = (**list).nexti;
    while !expr.is_null() {
        let mut t1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        let mut t2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        t1 = (*expr).nexti;
        t2 = (*expr).d.di;
        if isarg as i32 != 0 && t1 == t2
            && (*t1).opcode as u32 == opcodeval::Op_push as i32 as u32
        {
            (*t1).opcode = opcodeval::Op_push_param;
        }
        count += 1;
        if count == 1 as i32 {
            r = expr;
        } else {
            list_merge(r, expr);
        }
        expr = (*t2).nexti;
    }
    if !isarg && count > max_args {
        max_args = count;
    }
    bcfree(*list);
    *list = r;
    return count;
}
unsafe extern "C" fn fix_break_continue(
    mut list: *mut INSTRUCTION,
    mut b_target: *mut INSTRUCTION,
    mut c_target: *mut INSTRUCTION,
) {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    (*(*list).d.di).nexti = 0 as *mut exp_instruction;
    ip = (*list).nexti;
    while !ip.is_null() {
        match (*ip).opcode as u32 {
            54 => {
                if ((*ip).d.di).is_null() {
                    (*ip).d.di = b_target;
                }
            }
            55 => {
                if ((*ip).d.di).is_null() {
                    (*ip).d.di = c_target;
                }
            }
            _ => {}
        }
        ip = (*ip).nexti;
    }
}
#[inline]
unsafe extern "C" fn list_create(mut x: *mut INSTRUCTION) -> *mut INSTRUCTION {
    let mut l: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    l = bcalloc(opcodeval::Op_list, 1 as i32, 0 as i32);
    (*l).nexti = x;
    (*l).d.di = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_append(
    mut l: *mut INSTRUCTION,
    mut x: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*(*l).d.di).nexti = x;
    (*l).d.di = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_prepend(
    mut l: *mut INSTRUCTION,
    mut x: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*x).nexti = (*l).nexti;
    (*l).nexti = x;
    return l;
}
#[inline]
unsafe extern "C" fn list_merge(
    mut l1: *mut INSTRUCTION,
    mut l2: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    (*(*l1).d.di).nexti = (*l2).nexti;
    (*l1).d.di = (*l2).d.di;
    bcfree(l2);
    return l1;
}
#[no_mangle]
pub unsafe extern "C" fn check_special(mut name: *const i8) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    let mut mid: i32 = 0;
    let mut i: i32 = 0;
    let mut non_standard_flags: i32 = 0 as i32;
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        non_standard_flags |= 0x400 as i32;
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        non_standard_flags |= 0x200 as i32;
    }
    low = 0 as i32;
    high = (::core::mem::size_of::<[token; 71]>() as u64)
        .wrapping_div(::core::mem::size_of::<token>() as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    while low <= high {
        mid = (low + high) / 2 as i32;
        i = *name as i32
            - *(tokentab[mid as usize].operator).offset(0 as i32 as isize) as i32;
        if i == 0 as i32 {
            i = strcmp(name, tokentab[mid as usize].operator);
        }
        if i < 0 as i32 {
            high = mid - 1 as i32;
        } else if i > 0 as i32 {
            low = mid + 1 as i32;
        } else {
            if tokentab[mid as usize].flags & non_standard_flags as u32
                != 0 as i32 as u32
            {
                return -(1 as i32);
            }
            return mid;
        }
    }
    return -(1 as i32);
}
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn read_one_line(
    mut fd: i32,
    mut buffer: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut buf: [i8; 8192] = [0; 8192];
    if fp.is_null() {
        fp = fdopen(fd, b"r\0" as *const u8 as *const i8);
        if fp.is_null() {
            fprintf(
                stderr,
                b"ugh. fdopen: %s\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
            gawk_exit(1 as i32);
        }
    }
    if (fgets(buf.as_mut_ptr(), ::core::mem::size_of::<[i8; 8192]>() as u64 as i32, fp))
        .is_null()
    {
        return 0 as i32 as ssize_t;
    }
    memcpy(buffer, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()));
    return strlen(buf.as_mut_ptr()) as ssize_t;
}
unsafe extern "C" fn one_line_close(mut fd: i32) -> i32 {
    let mut ret: i32 = 0;
    if fp.is_null() || fd != fileno(fp) {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"awkgram.y\0" as *const u8 as *const i8, 6506 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(b"debugging read/close screwed up!\0" as *const u8 as *const i8);
    }
    ret = fclose(fp);
    fp = 0 as *mut FILE;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lookup_builtin(mut name: *const i8) -> builtin_func_t {
    if strncmp(name, b"awk::\0" as *const u8 as *const i8, 5 as i32 as u64) == 0 as i32 {
        name = name.offset(5 as i32 as isize);
    }
    let mut mid: i32 = check_special(name);
    if mid == -(1 as i32) {
        return None;
    }
    match tokentab[mid as usize].class {
        301 | 302 => {}
        _ => return None,
    }
    if tokentab[mid as usize].value as u32 == opcodeval::Op_sub_builtin as i32 as u32 {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(i32, u32) -> *mut NODE>,
            builtin_func_t,
        >(Some(do_sub as unsafe extern "C" fn(i32, u32) -> *mut NODE));
    }
    return tokentab[mid as usize].ptr;
}
#[no_mangle]
pub unsafe extern "C" fn install_builtins() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut flags_that_must_be_clear: i32 = 0x2000 as i32;
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
        flags_that_must_be_clear |= 0x400 as i32;
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        flags_that_must_be_clear |= 0x200 as i32;
    }
    j = (::core::mem::size_of::<[token; 71]>() as u64)
        .wrapping_div(::core::mem::size_of::<token>() as u64) as i32;
    i = 0 as i32;
    while i < j {
        if (tokentab[i as usize].class == 301 as i32
            || tokentab[i as usize].class == 302 as i32)
            && tokentab[i as usize].flags & flags_that_must_be_clear as u32
                == 0 as i32 as u32
        {
            install_symbol(tokentab[i as usize].operator, nodevals::Node_builtin_func);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn is_alpha(mut c: i32) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as i32 != 0,
        _ => {}
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_alnum(mut c: i32) -> bool {
    return is_alpha(c) as i32 != 0 || '0' as i32 <= c && c <= '9' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_letter(mut c: i32) -> bool {
    return is_alpha(c) as i32 != 0 || c == '_' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_identchar(mut c: i32) -> bool {
    return is_alnum(c) as i32 != 0 || c == '_' as i32;
}
unsafe extern "C" fn set_profile_text(
    mut n: *mut NODE,
    mut str: *const i8,
    mut len: size_t,
) -> *mut NODE {
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        (*n).sub.val.sp = emalloc_real(
            len.wrapping_add(2 as i32 as u64),
            b"set_profile_text\0" as *const u8 as *const i8,
            b"n->stptr\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            6647 as i32,
        ) as *mut i8;
        memcpy((*n).sub.val.sp as *mut libc::c_void, str as *const libc::c_void, len);
        *((*n).sub.val.sp).offset(len as isize) = '\0' as i32 as i8;
        (*n).sub.val.slen = len;
        (*n).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >(
            (*n).flags as u32
                | (flagvals::NUMCONSTSTR as i32 | flagvals::STRCUR as i32) as u32,
        );
        (*n).sub.val.idx = -(1 as i32);
    }
    return n;
}
unsafe extern "C" fn merge_comments(mut c1: *mut INSTRUCTION, mut c2: *mut INSTRUCTION) {
    if ((*c1).comment).is_null() && c2.is_null() {
        return;
    }
    let mut total: size_t = (*(*c1).d.dn).sub.val.slen;
    if !((*c1).comment).is_null() {
        total = (total as u64)
            .wrapping_add(
                (1 as i32 as u64).wrapping_add((*(*(*c1).comment).d.dn).sub.val.slen),
            ) as size_t as size_t;
    }
    if !c2.is_null() {
        total = (total as u64)
            .wrapping_add((1 as i32 as u64).wrapping_add((*(*c2).d.dn).sub.val.slen))
            as size_t as size_t;
        if !((*c2).comment).is_null() {
            total = (total as u64)
                .wrapping_add(
                    ((*(*(*c2).comment).d.dn).sub.val.slen).wrapping_add(1 as i32 as u64),
                ) as size_t as size_t;
        }
    }
    let mut buffer: *mut i8 = 0 as *mut i8;
    buffer = emalloc_real(
        total.wrapping_add(1 as i32 as u64),
        b"merge_comments\0" as *const u8 as *const i8,
        b"buffer\0" as *const u8 as *const i8,
        b"awkgram.y\0" as *const u8 as *const i8,
        6691 as i32,
    ) as *mut i8;
    strcpy(buffer, (*(*c1).d.dn).sub.val.sp);
    if !((*c1).comment).is_null() {
        strcat(buffer, b"\n\0" as *const u8 as *const i8);
        strcat(buffer, (*(*(*c1).comment).d.dn).sub.val.sp);
    }
    if !c2.is_null() {
        strcat(buffer, (*(*c2).d.dn).sub.val.sp);
        if !((*c2).comment).is_null() {
            strcat(buffer, b"\n\0" as *const u8 as *const i8);
            strcat(buffer, (*(*(*c2).comment).d.dn).sub.val.sp);
        }
        unref((*c2).d.dn);
        if !((*c2).comment).is_null() {
            unref((*(*c2).comment).d.dn);
            bcfree((*c2).comment);
            (*c2).comment = 0 as *mut exp_instruction;
        }
        bcfree(c2);
    }
    (*(*c1).d.dn).sub.val.comtype = commenttype::BLOCK_COMMENT;
    pma_free((*(*c1).d.dn).sub.val.sp as *mut libc::c_void);
    (*(*c1).d.dn).sub.val.sp = buffer;
    (*(*c1).d.dn).sub.val.slen = strlen(buffer);
    if !((*c1).comment).is_null() {
        unref((*(*c1).comment).d.dn);
        bcfree((*c1).comment);
        (*c1).comment = 0 as *mut exp_instruction;
    }
}
unsafe extern "C" fn make_braced_statements(
    mut lbrace: *mut INSTRUCTION,
    mut stmts: *mut INSTRUCTION,
    mut rbrace: *mut INSTRUCTION,
) -> *mut INSTRUCTION {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if stmts.is_null() {
        ip = list_create(bcalloc(opcodeval::Op_no_op, 1 as i32, 0 as i32));
    } else {
        ip = stmts;
    }
    if !lbrace.is_null() {
        let mut comment2: *mut INSTRUCTION = (*lbrace).comment;
        if !comment2.is_null() {
            ip = list_prepend(ip, comment2);
            (*lbrace).comment = 0 as *mut exp_instruction;
        }
        ip = list_prepend(ip, lbrace);
    }
    return ip;
}
#[no_mangle]
pub unsafe extern "C" fn validate_qualified_name(mut token: *mut i8) -> bool {
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    cp = strchr(token, ':' as i32);
    if cp.is_null() {
        return 1 as i32 != 0;
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
    {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const i8,
                b"identifier %s: qualified names not allowed in traditional / POSIX mode\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            token,
        );
        return 0 as i32 != 0;
    }
    if *cp.offset(1 as i32 as isize) as i32 != ':' as i32 {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const i8,
                b"identifier %s: namespace separator is two colons, not one\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            token,
        );
        return 0 as i32 != 0;
    }
    if !is_letter(*cp.offset(2 as i32 as isize) as i32) {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const i8,
                b"qualified identifier `%s' is badly formed\0" as *const u8 as *const i8,
                5 as i32,
            ),
            token,
        );
        return 0 as i32 != 0;
    }
    cp2 = strchr(cp.offset(2 as i32 as isize), ':' as i32);
    if !cp2.is_null() {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const i8,
                b"identifier `%s': namespace separator can only appear once in a qualified name\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            token,
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn check_qualified_special(mut token: *mut i8) -> i32 {
    let mut cp: *mut i8 = 0 as *mut i8;
    cp = strchr(token, ':' as i32);
    if cp.is_null() && current_namespace == awk_namespace.as_ptr() {
        return check_special(token);
    }
    let mut tok_0: *const token = 0 as *const token;
    let mut i: i32 = 0;
    if cp.is_null() {
        i = check_special(token);
        if i < 0 as i32 {
            return i;
        }
        tok_0 = &*tokentab.as_ptr().offset(i as isize) as *const token;
        if (*tok_0).flags & 0x400 as i32 as u32 != 0 as i32 as u32
            && (*tok_0).class == 301 as i32
        {
            return -(1 as i32)
        } else {
            return i
        }
    }
    let mut ns: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut subname: *mut i8 = 0 as *mut i8;
    ns = token;
    end = cp;
    *end = '\0' as i32 as i8;
    subname = end.offset(2 as i32 as isize);
    i = check_special(ns);
    if i >= 0 as i32
        && tokentab[i as usize].flags & 0x400 as i32 as u32 == 0 as i32 as u32
    {
        error_ln(
            sourceline,
            dcgettext(
                0 as *const i8,
                b"using reserved identifier `%s' as a namespace is not allowed\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            ns,
        );
    } else {
        i = check_special(subname);
        if i >= 0 as i32
            && tokentab[i as usize].flags & 0x400 as i32 as u32 == 0 as i32 as u32
            && strcmp(ns, awk_namespace.as_ptr()) != 0 as i32
        {
            error_ln(
                sourceline,
                dcgettext(
                    0 as *const i8,
                    b"using reserved identifier `%s' as second component of a qualified name is not allowed\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                subname,
            );
        } else if strcmp(ns, awk_namespace.as_ptr()) == 0 as i32 {
            i = check_special(subname);
            if i >= 0 as i32 {
                if !(tokentab[i as usize].flags & 0x400 as i32 as u32 != 0 as i32 as u32
                    && tokentab[i as usize].class == 301 as i32)
                {
                    error_ln(
                        sourceline,
                        dcgettext(
                            0 as *const i8,
                            b"using reserved identifier `%s' as second component of a qualified name is not allowed\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        subname,
                    );
                }
            }
        } else {
            i = -(1 as i32);
        }
    }
    *end = ':' as i32 as i8;
    return i;
}
unsafe extern "C" fn set_namespace(
    mut ns: *mut INSTRUCTION,
    mut comment: *mut INSTRUCTION,
) {
    if ns.is_null() {
        return;
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
        || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
    {
        error_ln(
            (*ns).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"@namespace is a gawk extension\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        pma_free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    if !is_valid_identifier((*ns).d.name) {
        error_ln(
            (*ns).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"namespace name `%s' must meet identifier naming rules\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            (*ns).d.name,
        );
        pma_free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    let mut mid: i32 = check_special((*ns).d.name);
    if mid >= 0 as i32 {
        error_ln(
            (*ns).source_line as i32,
            dcgettext(
                0 as *const i8,
                b"using reserved identifier `%s' as a namespace is not allowed\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            (*ns).d.name,
        );
        pma_free((*ns).d.name as *mut libc::c_void);
        bcfree(ns);
        return;
    }
    pma_free((*ns).d.name as *mut libc::c_void);
    let mut new_ns: *mut INSTRUCTION = bcalloc(
        opcodeval::Op_K_namespace,
        1 as i32,
        0 as i32,
    );
    (*new_ns).comment = comment;
    (*new_ns).d.name = estrdup(current_namespace, strlen(current_namespace));
    (*new_ns).nexti = namespace_chain;
    namespace_chain = new_ns;
    (*ns).d.name = 0 as *mut i8;
    bcfree(ns);
}
unsafe extern "C" fn change_namespace(mut new_namespace: *const i8) {
    if !is_valid_identifier(new_namespace) {
        return;
    }
    let mut mid: i32 = check_special(new_namespace);
    if mid >= 0 as i32 {
        return;
    }
    if !(strcmp(new_namespace, current_namespace) == 0 as i32) {
        if strcmp(new_namespace, awk_namespace.as_ptr()) == 0 as i32 {
            set_current_namespace(awk_namespace.as_ptr());
        } else {
            set_current_namespace(estrdup(new_namespace, strlen(new_namespace)));
        }
    }
    namespace_changed = 1 as i32 != 0;
}
unsafe extern "C" fn qualify_name(mut name: *const i8, mut len: size_t) -> *mut i8 {
    if !(strchr(name, ':' as i32)).is_null() {
        return estrdup(name, len);
    }
    let mut p: *mut NODE = lookup(name);
    if !p.is_null() && (*p).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
        return estrdup(name, len);
    }
    if current_namespace != awk_namespace.as_ptr() && !is_all_upper(name) {
        let mut length: size_t = (strlen(current_namespace))
            .wrapping_add(2 as i32 as u64)
            .wrapping_add(len)
            .wrapping_add(1 as i32 as u64);
        let mut buf: *mut i8 = 0 as *mut i8;
        buf = emalloc_real(
            length,
            b"qualify_name\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"awkgram.y\0" as *const u8 as *const i8,
            6953 as i32,
        ) as *mut i8;
        sprintf(buf, b"%s::%s\0" as *const u8 as *const i8, current_namespace, name);
        return buf;
    }
    return estrdup(name, len);
}
unsafe extern "C" fn run_static_initializers() {
    syn_err_len = (::core::mem::size_of::<[i8; 13]>() as u64)
        .wrapping_sub(1 as i32 as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];