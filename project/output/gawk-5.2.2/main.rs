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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    pub type instruction_block;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn pma_init(verbose: i32, file: *const i8) -> i32;
    static mut pma_errno: i32;
    static pma_version: [i8; 0];
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn abort() -> !;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn localeconv() -> *mut lconv;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strsignal(__sig: i32) -> *mut i8;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    static mut environ: *mut *mut i8;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getpgrp() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: i32, __list: *mut __gid_t) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    static mut opterr: i32;
    static mut optopt: i32;
    fn pma_save_free_lists();
    fn pma_mpfr_check();
    fn push_context(ctxt: *mut AWK_CONTEXT);
    fn new_context() -> *mut AWK_CONTEXT;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut errcount: i32;
    static mut interpret: Option<unsafe extern "C" fn(*mut INSTRUCTION) -> i32>;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static str_array_func: array_funcs_t;
    static mut nextfree: [block_header; 2];
    fn release_all_vars();
    fn install_symbol(name: *const i8, type_0: NODETYPE) -> *mut NODE;
    fn init_symbol_table();
    static quote: i8;
    static mut defpath: *const i8;
    static mut deflibpath: *const i8;
    static awk_namespace: [i8; 0];
    static mut current_namespace: *const i8;
    fn r_unref(tmp: *mut NODE);
    fn make_array() -> *mut NODE;
    fn null_array(symbol: *mut NODE);
    fn array_init();
    fn set_SUBSEP();
    fn init_env_array(env_node: *mut NODE);
    fn init_argv_array(argv_node: *mut NODE, shadow_node: *mut NODE);
    fn variable(location: i32, name: *mut i8, type_0: NODETYPE) -> *mut NODE;
    fn parse_program(pcode: *mut *mut INSTRUCTION, from_eval: bool) -> i32;
    fn dump_funcs();
    fn dump_vars(fname: *const i8);
    fn shadow_funcs();
    fn check_special(name: *const i8) -> i32;
    fn add_srcfile(
        stype: srctype,
        src: *mut i8,
        curr: *mut SRCFILE,
        already_included: *mut bool,
        errcode: *mut i32,
    ) -> *mut SRCFILE;
    fn install_builtins();
    fn is_letter(c: i32) -> bool;
    fn is_identchar(c: i32) -> bool;
    fn validate_qualified_name(token: *mut i8) -> bool;
    fn init_debug();
    fn debug_prog(pc: *mut INSTRUCTION) -> i32;
    fn init_interpret();
    fn set_IGNORECASE();
    fn set_OFS();
    fn set_ORS();
    fn set_OFMT();
    fn set_CONVFMT();
    fn set_BINMODE();
    fn set_LINT();
    fn set_TEXTDOMAIN();
    fn update_NR();
    fn update_NF();
    fn update_FNR();
    fn load_casetable();
    fn r_get_lhs(n: *mut NODE, reference: bool) -> *mut *mut NODE;
    fn load_ext(lib_name: *const i8);
    fn init_fields();
    fn set_NF();
    fn set_FS();
    fn set_RS();
    fn set_FIELDWIDTHS();
    fn set_FPAT();
    fn update_PROCINFO_str(subscript: *const i8, str: *const i8);
    fn update_PROCINFO_num(subscript: *const i8, val: libc::c_double);
    fn current_field_sep_str() -> *const i8;
    fn init_ext_api();
    fn print_ext_versions();
    fn gawk_name(filespec: *const i8) -> *const i8;
    fn os_arg_fixup(argcp: *mut i32, argvp: *mut *mut *mut i8);
    fn os_isatty(fd: i32) -> i32;
    fn os_is_setuid() -> i32;
    fn os_setbinmode(fd: i32, mode: i32) -> i32;
    fn os_maybe_set_errno();
    fn init_io();
    fn set_FNR();
    fn set_NR();
    fn devopen(name: *const i8, mode: *const i8) -> i32;
    fn load_symbols();
    fn final_exit(status: i32) -> !;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn make_typed_regex(re: *const i8, len: size_t) -> *mut NODE;
    fn lookup(name: *const i8) -> *mut NODE;
    fn r_warning(mesg: *const i8, _: ...);
    fn resetup();
    fn set_ROUNDMODE();
    fn set_PREC();
    fn init_btowc_cache();
    fn msg(mesg: *const i8, _: ...);
    fn init_profiling_signals();
    fn set_prof_file(filename: *const i8);
    fn dump_prog(code: *mut INSTRUCTION);
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn mtrace();
    static mut version_string: *const i8;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type gid_t = __gid_t;
pub type time_t = __time_t;
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
pub struct lconv {
    pub decimal_point: *mut i8,
    pub thousands_sep: *mut i8,
    pub grouping: *mut i8,
    pub int_curr_symbol: *mut i8,
    pub currency_symbol: *mut i8,
    pub mon_decimal_point: *mut i8,
    pub mon_thousands_sep: *mut i8,
    pub mon_grouping: *mut i8,
    pub positive_sign: *mut i8,
    pub negative_sign: *mut i8,
    pub int_frac_digits: i8,
    pub frac_digits: i8,
    pub p_cs_precedes: i8,
    pub p_sep_by_space: i8,
    pub n_cs_precedes: i8,
    pub n_sep_by_space: i8,
    pub p_sign_posn: i8,
    pub n_sign_posn: i8,
    pub int_p_cs_precedes: i8,
    pub int_p_sep_by_space: i8,
    pub int_n_cs_precedes: i8,
    pub int_n_sep_by_space: i8,
    pub int_p_sign_posn: i8,
    pub int_n_sign_posn: i8,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    GAWK_API_MINOR_VERSION = 2,
    GAWK_API_MAJOR_VERSION = 3,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::GAWK_API_MINOR_VERSION => 2,
            C2RustUnnamed_0::GAWK_API_MAJOR_VERSION => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            2 => C2RustUnnamed_0::GAWK_API_MINOR_VERSION,
            3 => C2RustUnnamed_0::GAWK_API_MAJOR_VERSION,
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
pub type INSTRUCTION = exp_instruction;
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum binmode_values {
    TEXT_TRANSLATE = 0,
    BINMODE_INPUT = 1,
    BINMODE_OUTPUT = 2,
    BINMODE_BOTH = 3,
}
impl binmode_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            binmode_values::TEXT_TRANSLATE => 0,
            binmode_values::BINMODE_INPUT => 1,
            binmode_values::BINMODE_OUTPUT => 2,
            binmode_values::BINMODE_BOTH => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> binmode_values {
        match value {
            0 => binmode_values::TEXT_TRANSLATE,
            1 => binmode_values::BINMODE_INPUT,
            2 => binmode_values::BINMODE_OUTPUT,
            3 => binmode_values::BINMODE_BOTH,
            _ => panic!("Invalid value for binmode_values: {}", value),
        }
    }
}
impl AddAssign<u32> for binmode_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for binmode_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for binmode_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for binmode_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for binmode_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for binmode_values {
    type Output = binmode_values;
    fn add(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for binmode_values {
    type Output = binmode_values;
    fn sub(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for binmode_values {
    type Output = binmode_values;
    fn mul(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for binmode_values {
    type Output = binmode_values;
    fn div(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for binmode_values {
    type Output = binmode_values;
    fn rem(self, rhs: u32) -> binmode_values {
        binmode_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
pub struct instruction_pool {
    pub pool: [instruction_mem_pool; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_mem_pool {
    pub block_list: *mut instruction_block,
    pub free_space: *mut INSTRUCTION,
    pub free_list: *mut INSTRUCTION,
}
pub type INSTRUCTION_POOL = instruction_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub pools: INSTRUCTION_POOL,
    pub symbols: NODE,
    pub rule_list: INSTRUCTION,
    pub srcfiles: SRCFILE,
    pub sourceline: i32,
    pub source: *mut i8,
    pub install_func: Option<unsafe extern "C" fn(*mut NODE) -> ()>,
    pub prev: *mut context,
}
pub type AWK_CONTEXT = context;
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
pub struct varinit {
    pub spec: *mut *mut NODE,
    pub name: *const i8,
    pub strval: *const i8,
    pub numval: libc::c_double,
    pub update: Func_ptr,
    pub assign: Func_ptr,
    pub do_assign: bool,
    pub flags: i32,
}
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
pub struct pre_assign {
    pub type_0: assign_type,
    pub val: *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum assign_type {
    PRE_ASSIGN = 1,
    PRE_ASSIGN_FS,
}
impl assign_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            assign_type::PRE_ASSIGN => 1,
            assign_type::PRE_ASSIGN_FS => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> assign_type {
        match value {
            1 => assign_type::PRE_ASSIGN,
            2 => assign_type::PRE_ASSIGN_FS,
            _ => panic!("Invalid value for assign_type: {}", value),
        }
    }
}
impl AddAssign<u32> for assign_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = assign_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for assign_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = assign_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for assign_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = assign_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for assign_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = assign_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for assign_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = assign_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for assign_type {
    type Output = assign_type;
    fn add(self, rhs: u32) -> assign_type {
        assign_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for assign_type {
    type Output = assign_type;
    fn sub(self, rhs: u32) -> assign_type {
        assign_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for assign_type {
    type Output = assign_type;
    fn mul(self, rhs: u32) -> assign_type {
        assign_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for assign_type {
    type Output = assign_type;
    fn div(self, rhs: u32) -> assign_type {
        assign_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for assign_type {
    type Output = assign_type;
    fn rem(self, rhs: u32) -> assign_type {
        assign_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
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
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
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
static mut varfile: *const i8 = b"awkvars.out\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut command_file: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut ARGC_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ARGIND_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ARGV_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut BINMODE_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut CONVFMT_node: *mut NODE = 0 as *const NODE as *mut NODE;
static mut ENVIRON_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ERRNO_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut FIELDWIDTHS_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut FILENAME_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut FNR_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut FPAT_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut FS_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut IGNORECASE_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut LINT_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut NF_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut NR_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut OFMT_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut OFS_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ORS_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut PROCINFO_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut RLENGTH_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut RSTART_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut RS_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut RT_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut SUBSEP_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut PREC_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut ROUNDMODE_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut TEXTDOMAIN_node: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut NF: i64 = 0;
#[no_mangle]
pub static mut NR: i64 = 0;
#[no_mangle]
pub static mut FNR: i64 = 0;
#[no_mangle]
pub static mut BINMODE: i32 = 0;
#[no_mangle]
pub static mut IGNORECASE: bool = false;
#[no_mangle]
pub static mut OFS: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut ORS: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut OFMT: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut TEXTDOMAIN: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut CONVFMT: *const i8 = b"%.6g\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut Nnull_string: *mut NODE = 0 as *const NODE as *mut NODE;
#[no_mangle]
pub static mut loc: lconv = lconv {
    decimal_point: 0 as *const i8 as *mut i8,
    thousands_sep: 0 as *const i8 as *mut i8,
    grouping: 0 as *const i8 as *mut i8,
    int_curr_symbol: 0 as *const i8 as *mut i8,
    currency_symbol: 0 as *const i8 as *mut i8,
    mon_decimal_point: 0 as *const i8 as *mut i8,
    mon_thousands_sep: 0 as *const i8 as *mut i8,
    mon_grouping: 0 as *const i8 as *mut i8,
    positive_sign: 0 as *const i8 as *mut i8,
    negative_sign: 0 as *const i8 as *mut i8,
    int_frac_digits: 0,
    frac_digits: 0,
    p_cs_precedes: 0,
    p_sep_by_space: 0,
    n_cs_precedes: 0,
    n_sep_by_space: 0,
    p_sign_posn: 0,
    n_sign_posn: 0,
    int_p_cs_precedes: 0,
    int_p_sep_by_space: 0,
    int_n_cs_precedes: 0,
    int_n_sep_by_space: 0,
    int_p_sign_posn: 0,
    int_n_sign_posn: 0,
};
#[no_mangle]
pub static mut myname: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut code_block: *mut INSTRUCTION = 0 as *const INSTRUCTION
    as *mut INSTRUCTION;
#[no_mangle]
pub static mut d_argv: *mut *mut i8 = 0 as *const *mut i8 as *mut *mut i8;
#[no_mangle]
pub static mut rule_list: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
#[no_mangle]
pub static mut exit_val: i32 = 0 as i32;
#[no_mangle]
pub static mut srcfiles: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut preassigns: *mut pre_assign = 0 as *const pre_assign as *mut pre_assign;
static mut numassigns: i64 = -(1 as i32) as i64;
static mut disallow_var_assigns: bool = 0 as i32 != 0;
static mut stopped_early: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut using_persistent_malloc: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut do_flags: do_flag_values = do_flag_values::DO_FLAG_NONE;
#[no_mangle]
pub static mut do_itrace: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut do_optimize: bool = 1 as i32 != 0;
static mut do_nostalgia: i32 = 0 as i32;
static mut do_binary: i32 = 0 as i32;
static mut do_version: i32 = 0 as i32;
static mut locale: *const i8 = b"\0" as *const u8 as *const i8;
static mut locale_dir: *const i8 = b"/usr/local/share/locale\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut use_lc_numeric: i32 = 0 as i32;
#[no_mangle]
pub static mut gawk_mb_cur_max: i32 = 0;
#[no_mangle]
pub static mut output_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut output_is_tty: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut def_strftime_format: [i8; 24] = unsafe {
    *::core::mem::transmute::<&[u8; 24], &[i8; 24]>(b"%a %b %e %H:%M:%S %Z %Y\0")
};
#[no_mangle]
pub static mut groupset: *mut gid_t = 0 as *const gid_t as *mut gid_t;
#[no_mangle]
pub static mut ngroups: i32 = 0;
#[no_mangle]
pub static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()> = unsafe {
    Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ())
};
static mut optab: [option; 31] = unsafe {
    [
        {
            let mut init = option {
                name: b"assign\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"bignum\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"characters-as-bytes\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &do_binary as *const i32 as *mut i32,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"copyright\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debug\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"dump-variables\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"exec\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"field-separator\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"gen-pot\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"include\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lint\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lint-old\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"load\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"non-decimal-data\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-optimize\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nostalgia\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &do_nostalgia as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"optimize\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"persist\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"posix\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'P' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pretty-print\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'o' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"profile\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"re-interval\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"sandbox\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"source\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"trace\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"traditional\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"use-lc-numeric\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &use_lc_numeric as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &do_version as *const i32 as *mut i32,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: '\0' as i32,
            };
            init
        },
    ]
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut extra_stack: *mut i8 = 0 as *mut i8;
    let mut have_srcfile: bool = 0 as i32 != 0;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut persist_file: *const i8 = getenv(
        b"GAWK_PERSIST_FILE\0" as *const u8 as *const i8,
    );
    myname = gawk_name(*argv.offset(0 as i32 as isize));
    check_pma_security(persist_file);
    let mut pma_result: i32 = pma_init(1 as i32, persist_file);
    if pma_result != 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: fatal: persistent memory allocator failed to initialize: return value %d, pma.c line: %d.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            myname,
            pma_result,
            pma_errno,
        );
        exit(2 as i32);
    }
    using_persistent_malloc = !persist_file.is_null();
    if !(getenv(b"TIDYMEM\0" as *const u8 as *const i8)).is_null() {
        do_flags = ::core::mem::transmute::<
            u32,
            do_flag_values,
        >(do_flags as u32 | do_flag_values::DO_TIDY_MEM as i32 as u32);
    }
    if !using_persistent_malloc
        && do_flags as u32 & do_flag_values::DO_TIDY_MEM as i32 as u32 != 0
    {
        mtrace();
    }
    os_arg_fixup(&mut argc, &mut argv);
    if argc < 2 as i32 {
        usage(1 as i32, stderr);
    }
    cp = getenv(b"GAWK_LOCALE_DIR\0" as *const u8 as *const i8);
    if !cp.is_null() {
        locale_dir = cp;
    }
    let mut flags: i32 = fcntl(fileno(stderr), 3 as i32, 0 as *mut libc::c_void);
    if flags >= 0 as i32 && flags & 0o2000 as i32 == 0 as i32 {
        flags |= 0o2000 as i32;
        fcntl(fileno(stderr), 4 as i32, flags);
    }
    set_locale_stuff();
    signal(8 as i32, Some(catchsig as unsafe extern "C" fn(i32) -> ()));
    signal(7 as i32, Some(catchsig as unsafe extern "C" fn(i32) -> ()));
    signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    signal(11 as i32, Some(catchsig as unsafe extern "C" fn(i32) -> ()));
    extra_stack = emalloc_real(
        (16 as i32 * 1024 as i32) as size_t,
        b"main\0" as *const u8 as *const i8,
        b"extra_stack\0" as *const u8 as *const i8,
        b"main.c\0" as *const u8 as *const i8,
        316 as i32,
    ) as *mut i8;
    Nnull_string = make_str_node(
        b"\0" as *const u8 as *const i8,
        0 as i32 as size_t,
        0 as i32,
    );
    init_fds();
    array_init();
    init_symbol_table();
    output_fp = stdout;
    push_context(new_context());
    parse_args(argc, argv);
    gawk_mb_cur_max = __ctype_get_mb_cur_max() as i32;
    init_btowc_cache();
    if gawk_mb_cur_max == 1 as i32 {
        load_casetable();
    }
    if do_nostalgia != 0 {
        nostalgia();
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
        && !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null()
    {
        do_flags = ::core::mem::transmute::<
            u32,
            do_flag_values,
        >(do_flags as u32 | do_flag_values::DO_POSIX as i32 as u32);
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 365 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"environment variable `POSIXLY_CORRECT' set: turning on `--posix'\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
        use_lc_numeric = 1 as i32;
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 373 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`--posix' overrides `--traditional'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            do_flags = ::core::mem::transmute::<
                u32,
                do_flag_values,
            >(do_flags as u32 | do_flag_values::DO_TRADITIONAL as i32 as u32);
        }
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
        && do_flags as u32 & do_flag_values::DO_NON_DEC_DATA as i32 as u32 != 0
    {
        do_flags = ::core::mem::transmute::<
            u32,
            do_flag_values,
        >(do_flags as u32 & !(do_flag_values::DO_NON_DEC_DATA as i32) as u32);
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"main.c\0" as *const u8 as *const i8, 384 as i32);
        (Some(
            (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"`--posix'/`--traditional' overrides `--non-decimal-data'\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if do_binary != 0 {
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 389 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`--posix' overrides `--characters-as-bytes'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        } else {
            gawk_mb_cur_max = 1 as i32;
            setlocale(6 as i32, b"C\0" as *const u8 as *const i8);
        }
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        if os_is_setuid() != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 400 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"running %s setuid root may be a security problem\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                myname,
            );
        }
        if do_flags as u32 & do_flag_values::DO_INTERVALS as i32 as u32 != 0 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 402 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"The -r/--re-interval options no longer have any effect\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    if do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0 {
        init_debug();
    }
    init_groupset();
    (*Nnull_string).sub.val.fltnum = 0.0f64;
    (*Nnull_string).flags = flagvals::from_libc_c_uint(
        (flagvals::MALLOC as i32 | flagvals::STRCUR as i32 | flagvals::STRING as i32
            | flagvals::NUMCUR as i32 | flagvals::NUMBER as i32) as u32,
    );
    resetup();
    init_vars();
    init_fields();
    let mut dash_v_errs: i32 = 0 as i32;
    i = 0 as i32;
    while i as i64 <= numassigns {
        if (*preassigns.offset(i as isize)).type_0 as u32
            == assign_type::PRE_ASSIGN as i32 as u32
        {
            dash_v_errs
                += (arg_assign((*preassigns.offset(i as isize)).val, 1 as i32 != 0)
                    == 0 as i32) as i32;
        } else {
            cmdline_fs((*preassigns.offset(i as isize)).val);
        }
        pma_free((*preassigns.offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    if !preassigns.is_null() {
        pma_free(preassigns as *mut libc::c_void);
    }
    if BINMODE & binmode_values::BINMODE_INPUT as i32 != 0 as i32 {
        if os_setbinmode(fileno(stdin), 0 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 456 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot set binary mode on stdin: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if BINMODE & binmode_values::BINMODE_OUTPUT as i32 != 0 as i32 {
        if os_setbinmode(fileno(stdout), 0 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 459 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot set binary mode on stdout: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        if os_setbinmode(fileno(stderr), 0 as i32) == -(1 as i32) {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 461 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot set binary mode on stderr: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if os_isatty(fileno(stdout)) != 0 {
        output_is_tty = 1 as i32 != 0;
    }
    atexit(Some(pma_save_free_lists as unsafe extern "C" fn() -> ()));
    init_ext_api();
    s = (*srcfiles).next;
    while s != srcfiles {
        if (*s).stype as u32 == srctype::SRC_EXTLIB as i32 as u32 {
            load_ext((*s).fullpath);
        } else if (*s).stype as u32 != srctype::SRC_INC as i32 as u32 {
            have_srcfile = 1 as i32 != 0;
        }
        s = (*s).next;
    }
    if do_version != 0 {
        version();
    }
    if !have_srcfile {
        if optind > argc - 1 as i32 || stopped_early as i32 != 0 {
            usage(1 as i32, stderr);
        }
        add_srcfile(
            srctype::SRC_CMDLINE,
            *argv.offset(optind as isize),
            srcfiles,
            0 as *mut bool,
            0 as *mut i32,
        );
        optind += 1;
        optind;
    }
    init_interpret();
    init_args(
        optind,
        argc,
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
            *argv.offset(0 as i32 as isize)
        } else {
            myname
        },
        argv,
    );
    setlocale(1 as i32, b"C\0" as *const u8 as *const i8);
    if parse_program(&mut code_block, 0 as i32 != 0) != 0 as i32
        || dash_v_errs > 0 as i32
    {
        exit(1 as i32);
    }
    if do_flags as u32 & do_flag_values::DO_INTL as i32 as u32 != 0 {
        exit(0 as i32);
    }
    set_current_namespace(awk_namespace.as_ptr());
    install_builtins();
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
    {
        shadow_funcs();
    }
    if do_flags as u32
        & (do_flag_values::DO_LINT_INVALID as i32 | do_flag_values::DO_LINT_ALL as i32)
            as u32 != 0
        && (*(*code_block).nexti).opcode as u32 == opcodeval::Op_atexit as i32 as u32
    {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"main.c\0" as *const u8 as *const i8, 526 as i32);
        (Some(lintfunc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"no program text at all!\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    load_symbols();
    if do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0 {
        init_profiling_signals();
    }
    if use_lc_numeric != 0 {
        setlocale(1 as i32, locale);
    }
    init_io();
    output_fp = stdout;
    if do_flags as u32 & do_flag_values::DO_DEBUG as i32 as u32 != 0 {
        debug_prog(code_block);
    } else if !(do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0
        && do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 == 0)
    {
        interpret.expect("non-null function pointer")(code_block);
    }
    if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
        set_current_namespace(awk_namespace.as_ptr());
        dump_prog(code_block);
        dump_funcs();
    }
    if do_flags as u32 & do_flag_values::DO_DUMP_VARS as i32 as u32 != 0 {
        dump_vars(varfile);
    }
    if do_flags as u32 & do_flag_values::DO_TIDY_MEM as i32 as u32 != 0 {
        release_all_vars();
    }
    if !extra_stack.is_null() {
        pma_free(extra_stack as *mut libc::c_void);
    }
    final_exit(exit_val);
}
unsafe extern "C" fn add_preassign(mut type_0: assign_type, mut val: *mut i8) {
    static mut alloc_assigns: i64 = 0;
    numassigns += 1;
    numassigns;
    if preassigns.is_null() {
        preassigns = emalloc_real(
            (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<pre_assign>() as u64),
            b"add_preassign\0" as *const u8 as *const i8,
            b"preassigns\0" as *const u8 as *const i8,
            b"main.c\0" as *const u8 as *const i8,
            601 as i32,
        ) as *mut pre_assign;
        alloc_assigns = 4 as i32 as i64;
    } else if numassigns >= alloc_assigns {
        alloc_assigns *= 2 as i32 as i64;
        preassigns = erealloc_real(
            preassigns as *mut libc::c_void,
            (alloc_assigns as u64)
                .wrapping_mul(::core::mem::size_of::<pre_assign>() as u64),
            b"add_preassigns\0" as *const u8 as *const i8,
            b"preassigns\0" as *const u8 as *const i8,
            b"main.c\0" as *const u8 as *const i8,
            606 as i32,
        ) as *mut pre_assign;
    }
    (*preassigns.offset(numassigns as isize)).type_0 = type_0;
    let ref mut fresh0 = (*preassigns.offset(numassigns as isize)).val;
    *fresh0 = estrdup(val, strlen(val));
}
unsafe extern "C" fn usage(mut exitval: i32, mut fp: *mut FILE) -> ! {
    static mut gnu_url: [i8; 29] = unsafe {
        *::core::mem::transmute::<
            &[u8; 29],
            &[i8; 29],
        >(b"https://ftp.gnu.org/gnu/gawk\0")
    };
    static mut beta_url: [i8; 28] = unsafe {
        *::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"https://www.skeeve.com/gawk\0")
    };
    let mut url: *const i8 = 0 as *const i8;
    let mut major_version: i32 = 0;
    let mut minor_version: i32 = 0;
    let mut patchlevel: i32 = 0;
    patchlevel = 0 as i32;
    minor_version = patchlevel;
    major_version = minor_version;
    sscanf(
        b"5.2.2\0" as *const u8 as *const i8,
        b"%d.%d.%d\0" as *const u8 as *const i8,
        &mut major_version as *mut i32,
        &mut minor_version as *mut i32,
        &mut patchlevel as *mut i32,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [POSIX or GNU style options] -f progfile [--] file ...\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        myname,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [POSIX or GNU style options] [--] %cprogram%c file ...\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        myname,
        quote as i32,
        quote as i32,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"POSIX options:\t\tGNU long options: (standard)\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-f progfile\t\t--file=progfile\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-F fs\t\t\t--field-separator=fs\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-v var=val\t\t--assign=var=val\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"Short options:\t\tGNU long options: (extensions)\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-b\t\t\t--characters-as-bytes\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-c\t\t\t--traditional\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-C\t\t\t--copyright\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-d[file]\t\t--dump-variables[=file]\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-D[file]\t\t--debug[=file]\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-e 'program-text'\t--source='program-text'\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-E file\t\t\t--exec=file\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-g\t\t\t--gen-pot\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-h\t\t\t--help\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-i includefile\t\t--include=includefile\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-I\t\t\t--trace\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-l library\t\t--load=library\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-L[fatal|invalid|no-ext]\t--lint[=fatal|invalid|no-ext]\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-M\t\t\t--bignum\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-N\t\t\t--use-lc-numeric\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-n\t\t\t--non-decimal-data\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-o[file]\t\t--pretty-print[=file]\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-O\t\t\t--optimize\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-p[file]\t\t--profile[=file]\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-P\t\t\t--posix\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-r\t\t\t--re-interval\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-s\t\t\t--no-optimize\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-S\t\t\t--sandbox\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-t\t\t\t--lint-old\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\t-V\t\t\t--version\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"\nTo report bugs, use the `gawkbug' program.\nFor full instructions, see the node `Bugs' in `gawk.info'\nwhich is section `Reporting Problems and Bugs' in the\nprinted version.  This same information may be found at\nhttps://www.gnu.org/software/gawk/manual/html_node/Bugs.html.\nPLEASE do NOT try to report bugs by posting in comp.lang.awk,\nor by using a web forum such as Stack Overflow.\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    if patchlevel >= 60 as i32
        || *(*__ctype_b_loc())
            .offset(
                (*::core::mem::transmute::<
                    &[u8; 6],
                    &[i8; 6],
                >(
                    b"5.2.2\0",
                ))[(strlen(b"5.2.2\0" as *const u8 as *const i8))
                    .wrapping_sub(1 as i32 as u64) as usize] as i32 as isize,
            ) as i32 & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
    {
        url = beta_url.as_ptr();
    } else {
        url = gnu_url.as_ptr();
    }
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Source code for gawk may be obtained from\n%s/gawk-%s.tar.gz\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        url,
        b"5.2.2\0" as *const u8 as *const i8,
    );
    fputs(
        dcgettext(
            0 as *const i8,
            b"gawk is a pattern scanning and processing language.\nBy default it reads standard input and writes standard output.\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        fp,
    );
    fprintf(
        fp,
        dcgettext(
            0 as *const i8,
            b"Examples:\n\t%s '{ sum += $1 }; END { print sum }' file\n\t%s -F: '{ print $1 }' /etc/passwd\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        myname,
        myname,
    );
    fflush(fp);
    if ferror(fp) != 0 {
        os_maybe_set_errno();
        if *__errno_location() == 32 as i32 {
            signal(13 as i32, None);
            kill(getpid(), 13 as i32);
        }
        if fp == stdout {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 719 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"error writing standard output: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        } else if fp == stderr {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 721 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"error writing standard error: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(1 as i32);
    }
    exit(exitval);
}
unsafe extern "C" fn copyleft() -> ! {
    static mut blurb_part1: [i8; 297] = unsafe {
        *::core::mem::transmute::<
            &[u8; 297],
            &[i8; 297],
        >(
            b"Copyright (C) 1989, 1991-%d Free Software Foundation.\n\nThis program is free software; you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation; either version 3 of the License, or\n(at your option) any later version.\n\n\0",
        )
    };
    static mut blurb_part2: [i8; 236] = unsafe {
        *::core::mem::transmute::<
            &[u8; 236],
            &[i8; 236],
        >(
            b"This program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\n\0",
        )
    };
    static mut blurb_part3: [i8; 134] = unsafe {
        *::core::mem::transmute::<
            &[u8; 134],
            &[i8; 134],
        >(
            b"You should have received a copy of the GNU General Public License\nalong with this program. If not, see http://www.gnu.org/licenses/.\n\0",
        )
    };
    printf(dcgettext(0 as *const i8, blurb_part1.as_ptr(), 5 as i32), 2023 as i32);
    fputs(dcgettext(0 as *const i8, blurb_part2.as_ptr(), 5 as i32), stdout);
    fputs(dcgettext(0 as *const i8, blurb_part3.as_ptr(), 5 as i32), stdout);
    fflush(stdout);
    if ferror(stdout) != 0 {
        os_maybe_set_errno();
        if *__errno_location() != 32 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 764 as i32);
            (Some(
                (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"error writing standard output: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(1 as i32);
    }
    exit(0 as i32);
}
unsafe extern "C" fn cmdline_fs(mut str: *mut i8) {
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    tmp = &mut (*FS_node).sub.nodep.l.lptr;
    unref(*tmp);
    if *str.offset(0 as i32 as isize) as i32 == 't' as i32
        && *str.offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 789 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"-Ft does not set FS to tab in POSIX awk\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
            && do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
        {
            *str.offset(0 as i32 as isize) = '\t' as i32 as i8;
        }
    }
    *tmp = make_str_node(str, strlen(str), 1 as i32);
    set_FS();
}
unsafe extern "C" fn init_args(
    mut argc0: i32,
    mut argc: i32,
    mut argv0: *const i8,
    mut argv: *mut *mut i8,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut shadow_node: *mut NODE = 0 as *mut NODE;
    ARGV_node = install_symbol(
        estrdup(b"ARGV\0" as *const u8 as *const i8, 4 as i32 as size_t),
        nodevals::Node_var_array,
    );
    sub = make_number.expect("non-null function pointer")(0.0f64);
    val = make_str_node(argv0, strlen(argv0), 0 as i32);
    (*val).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >((*val).flags as u32 | flagvals::USER_INPUT as i32 as u32);
    assoc_set(ARGV_node, sub, val);
    if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 != 0 {
        shadow_node = make_array();
        sub = make_str_node(argv0, strlen(argv0), 0 as i32);
        val = make_number.expect("non-null function pointer")(0.0f64);
        assoc_set(shadow_node, sub, val);
    }
    i = argc0;
    j = 1 as i32;
    while i < argc {
        sub = make_number.expect("non-null function pointer")(j as libc::c_double);
        val = make_str_node(
            *argv.offset(i as isize),
            strlen(*argv.offset(i as isize)),
            0 as i32,
        );
        (*val).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*val).flags as u32 | flagvals::USER_INPUT as i32 as u32);
        assoc_set(ARGV_node, sub, val);
        if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 != 0 {
            sub = make_str_node(
                *argv.offset(i as isize),
                strlen(*argv.offset(i as isize)),
                0 as i32,
            );
            val = make_number.expect("non-null function pointer")(0.0f64);
            assoc_set(shadow_node, sub, val);
        }
        i += 1;
        i;
        j += 1;
        j;
    }
    ARGC_node = install_symbol(
        estrdup(b"ARGC\0" as *const u8 as *const i8, 4 as i32 as size_t),
        nodevals::Node_var,
    );
    (*ARGC_node).sub.nodep.l.lptr = make_number
        .expect("non-null function pointer")(j as libc::c_double);
    if do_flags as u32 & do_flag_values::DO_SANDBOX as i32 as u32 != 0 {
        init_argv_array(ARGV_node, shadow_node);
    }
}
static mut varinit: [varinit; 29] = unsafe {
    [
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ARGC\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ARGIND_node as *const *mut NODE as *mut *mut NODE,
                name: b"ARGIND\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ARGV\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &BINMODE_node as *const *mut NODE as *mut *mut NODE,
                name: b"BINMODE\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_BINMODE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &CONVFMT_node as *const *mut NODE as *mut *mut NODE,
                name: b"CONVFMT\0" as *const u8 as *const i8,
                strval: b"%.6g\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_CONVFMT as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"ENVIRON\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x1 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ERRNO_node as *const *mut NODE as *mut *mut NODE,
                name: b"ERRNO\0" as *const u8 as *const i8,
                strval: b"\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FIELDWIDTHS_node as *const *mut NODE as *mut *mut NODE,
                name: b"FIELDWIDTHS\0" as *const u8 as *const i8,
                strval: b"\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_FIELDWIDTHS as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FILENAME_node as *const *mut NODE as *mut *mut NODE,
                name: b"FILENAME\0" as *const u8 as *const i8,
                strval: b"\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FNR_node as *const *mut NODE as *mut *mut NODE,
                name: b"FNR\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: Some(update_FNR as unsafe extern "C" fn() -> ()),
                assign: Some(set_FNR as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FS_node as *const *mut NODE as *mut *mut NODE,
                name: b"FS\0" as *const u8 as *const i8,
                strval: b" \0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_FS as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &FPAT_node as *const *mut NODE as *mut *mut NODE,
                name: b"FPAT\0" as *const u8 as *const i8,
                strval: b"[^[:space:]]+\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_FPAT as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &IGNORECASE_node as *const *mut NODE as *mut *mut NODE,
                name: b"IGNORECASE\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_IGNORECASE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &LINT_node as *const *mut NODE as *mut *mut NODE,
                name: b"LINT\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_LINT as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &PREC_node as *const *mut NODE as *mut *mut NODE,
                name: b"PREC\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 53 as i32 as libc::c_double,
                update: None,
                assign: Some(set_PREC as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &NF_node as *const *mut NODE as *mut *mut NODE,
                name: b"NF\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: -(1 as i32) as libc::c_double,
                update: Some(update_NF as unsafe extern "C" fn() -> ()),
                assign: Some(set_NF as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &NR_node as *const *mut NODE as *mut *mut NODE,
                name: b"NR\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: Some(update_NR as unsafe extern "C" fn() -> ()),
                assign: Some(set_NR as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &OFMT_node as *const *mut NODE as *mut *mut NODE,
                name: b"OFMT\0" as *const u8 as *const i8,
                strval: b"%.6g\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_OFMT as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &OFS_node as *const *mut NODE as *mut *mut NODE,
                name: b"OFS\0" as *const u8 as *const i8,
                strval: b" \0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_OFS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ORS_node as *const *mut NODE as *mut *mut NODE,
                name: b"ORS\0" as *const u8 as *const i8,
                strval: b"\n\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_ORS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: b"PROCINFO\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x1 as i32 | 0x2 as i32 | 0x4 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RLENGTH_node as *const *mut NODE as *mut *mut NODE,
                name: b"RLENGTH\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &ROUNDMODE_node as *const *mut NODE as *mut *mut NODE,
                name: b"ROUNDMODE\0" as *const u8 as *const i8,
                strval: b"N\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_ROUNDMODE as unsafe extern "C" fn() -> ()),
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RS_node as *const *mut NODE as *mut *mut NODE,
                name: b"RS\0" as *const u8 as *const i8,
                strval: b"\n\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_RS as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RSTART_node as *const *mut NODE as *mut *mut NODE,
                name: b"RSTART\0" as *const u8 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &RT_node as *const *mut NODE as *mut *mut NODE,
                name: b"RT\0" as *const u8 as *const i8,
                strval: b"\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &SUBSEP_node as *const *mut NODE as *mut *mut NODE,
                name: b"SUBSEP\0" as *const u8 as *const i8,
                strval: b"\x1C\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_SUBSEP as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: &TEXTDOMAIN_node as *const *mut NODE as *mut *mut NODE,
                name: b"TEXTDOMAIN\0" as *const u8 as *const i8,
                strval: b"messages\0" as *const u8 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: Some(set_TEXTDOMAIN as unsafe extern "C" fn() -> ()),
                do_assign: 1 as i32 != 0,
                flags: 0x2 as i32,
            };
            init
        },
        {
            let mut init = varinit {
                spec: 0 as *const *mut NODE as *mut *mut NODE,
                name: 0 as *const i8,
                strval: 0 as *const i8,
                numval: 0 as i32 as libc::c_double,
                update: None,
                assign: None,
                do_assign: 0 as i32 != 0,
                flags: 0 as i32,
            };
            init
        },
    ]
};
unsafe extern "C" fn init_vars() {
    let mut vp: *const varinit = 0 as *const varinit;
    let mut n: *mut NODE = 0 as *mut NODE;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if !((*vp).flags & 0x1 as i32 != 0 as i32) {
            *(*vp).spec = install_symbol(
                estrdup((*vp).name, strlen((*vp).name)),
                nodevals::Node_var,
            );
            n = *(*vp).spec;
            if !((*vp).strval).is_null() {
                (*n).sub.nodep.l.lptr = make_str_node(
                    (*vp).strval,
                    strlen((*vp).strval),
                    0 as i32,
                );
            } else {
                (*n).sub.nodep.l.lptr = make_number
                    .expect("non-null function pointer")((*vp).numval);
            }
            (*n).sub.nodep.x.aptr = (*vp).assign;
            (*n).sub.nodep.r.uptr = (*vp).update;
            if (*vp).do_assign {
                (Some(((*vp).assign).expect("non-null function pointer")))
                    .expect("non-null function pointer")();
            }
        }
        vp = vp.offset(1);
        vp;
    }
    if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0 {
        load_procinfo();
    }
    load_environ();
}
unsafe extern "C" fn path_environ(mut pname: *const i8, mut dflt: *const i8) {
    let mut val: *const i8 = 0 as *const i8;
    let mut aptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    tmp = make_str_node(pname, strlen(pname), 0 as i32);
    val = getenv(pname);
    if val.is_null() || *val as i32 == '\0' as i32 {
        val = dflt;
    }
    aptr = ((*(*ENVIRON_node).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(ENVIRON_node, tmp);
    if (**aptr).sub.val.slen == 0 as i32 as u64 {
        unref(*aptr);
        *aptr = make_str_node(val, strlen(val), 0 as i32);
    }
    unref(tmp);
}
unsafe extern "C" fn load_environ() -> *mut NODE {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut i8;
    }
    let mut var: *mut i8 = 0 as *mut i8;
    let mut val: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut newval: *mut NODE = 0 as *mut NODE;
    static mut been_here: bool = 0 as i32 != 0;
    if been_here {
        return ENVIRON_node;
    }
    been_here = 1 as i32 != 0;
    ENVIRON_node = install_symbol(
        estrdup(b"ENVIRON\0" as *const u8 as *const i8, 7 as i32 as size_t),
        nodevals::Node_var_array,
    );
    (*ENVIRON_node).sub.nodep.l.lp = &str_array_func;
    i = 0 as i32;
    while !(*environ.offset(i as isize)).is_null() {
        static mut nullstr: [i8; 1] = unsafe {
            *::core::mem::transmute::<&[u8; 1], &mut [i8; 1]>(b"\0")
        };
        var = *environ.offset(i as isize);
        val = strchr(var, '=' as i32);
        if !val.is_null() {
            let fresh1 = val;
            val = val.offset(1);
            *fresh1 = '\0' as i32 as i8;
        } else {
            val = nullstr.as_mut_ptr();
        }
        sub = make_str_node(var, strlen(var), 0 as i32);
        newval = make_str_node(val, strlen(val), 0 as i32);
        (*newval).flags = ::core::mem::transmute::<
            u32,
            flagvals,
        >((*newval).flags as u32 | flagvals::USER_INPUT as i32 as u32);
        assoc_set(ENVIRON_node, sub, newval);
        if val != nullstr.as_mut_ptr() {
            val = val.offset(-1);
            *val = '=' as i32 as i8;
        }
        i += 1;
        i;
    }
    path_environ(b"AWKPATH\0" as *const u8 as *const i8, defpath);
    path_environ(b"AWKLIBPATH\0" as *const u8 as *const i8, deflibpath);
    init_env_array(ENVIRON_node);
    return ENVIRON_node;
}
unsafe extern "C" fn load_procinfo_argv() {
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut argv_array: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    argv_array = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !argv_array.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(argv_array
            as *mut block_item))
            .freep;
    } else {
        argv_array = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(
        argv_array as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    null_array(argv_array);
    (*argv_array).sub.nodep.x.extra = PROCINFO_node;
    (*argv_array).sub.nodep.name = estrdup(
        b"argv\0" as *const u8 as *const i8,
        4 as i32 as size_t,
    );
    i = 0 as i32;
    while !(*d_argv.offset(i as isize)).is_null() {
        sub = make_number.expect("non-null function pointer")(i as libc::c_double);
        val = make_str_node(
            *d_argv.offset(i as isize),
            strlen(*d_argv.offset(i as isize)),
            0 as i32,
        );
        assoc_set(argv_array, sub, val);
        i += 1;
        i;
    }
    sub = make_str_node(
        b"argv\0" as *const u8 as *const i8,
        4 as i32 as size_t,
        0 as i32,
    );
    assoc_set(PROCINFO_node, sub, argv_array);
}
unsafe extern "C" fn load_procinfo() -> *mut NODE {
    let mut i: i32 = 0;
    let mut name: [i8; 100] = [0; 100];
    let mut value: libc::c_double = 0.;
    static mut been_here: bool = 0 as i32 != 0;
    if been_here {
        return PROCINFO_node;
    }
    been_here = 1 as i32 != 0;
    PROCINFO_node = install_symbol(
        estrdup(b"PROCINFO\0" as *const u8 as *const i8, 8 as i32 as size_t),
        nodevals::Node_var_array,
    );
    update_PROCINFO_str(
        b"version\0" as *const u8 as *const i8,
        b"5.2.2\0" as *const u8 as *const i8,
    );
    update_PROCINFO_str(
        b"strftime\0" as *const u8 as *const i8,
        def_strftime_format.as_ptr(),
    );
    update_PROCINFO_str(b"platform\0" as *const u8 as *const i8, platform_name());
    update_PROCINFO_num(
        b"api_major\0" as *const u8 as *const i8,
        C2RustUnnamed_0::GAWK_API_MAJOR_VERSION as i32 as libc::c_double,
    );
    update_PROCINFO_num(
        b"api_minor\0" as *const u8 as *const i8,
        C2RustUnnamed_0::GAWK_API_MINOR_VERSION as i32 as libc::c_double,
    );
    value = getpgrp() as libc::c_double;
    update_PROCINFO_num(b"pgrpid\0" as *const u8 as *const i8, value);
    value = getpid() as libc::c_double;
    update_PROCINFO_num(b"pid\0" as *const u8 as *const i8, value);
    value = getppid() as libc::c_double;
    update_PROCINFO_num(b"ppid\0" as *const u8 as *const i8, value);
    value = getuid() as libc::c_double;
    update_PROCINFO_num(b"uid\0" as *const u8 as *const i8, value);
    value = geteuid() as libc::c_double;
    update_PROCINFO_num(b"euid\0" as *const u8 as *const i8, value);
    value = getgid() as libc::c_double;
    update_PROCINFO_num(b"gid\0" as *const u8 as *const i8, value);
    value = getegid() as libc::c_double;
    update_PROCINFO_num(b"egid\0" as *const u8 as *const i8, value);
    update_PROCINFO_str(b"FS\0" as *const u8 as *const i8, current_field_sep_str());
    i = 0 as i32;
    while i < ngroups {
        sprintf(name.as_mut_ptr(), b"group%d\0" as *const u8 as *const i8, i + 1 as i32);
        value = *groupset.offset(i as isize) as libc::c_double;
        update_PROCINFO_num(name.as_mut_ptr(), value);
        i += 1;
        i;
    }
    if !groupset.is_null() {
        pma_free(groupset as *mut libc::c_void);
        groupset = 0 as *mut gid_t;
    }
    update_PROCINFO_str(b"pma\0" as *const u8 as *const i8, get_pma_version());
    load_procinfo_argv();
    return PROCINFO_node;
}
#[no_mangle]
pub unsafe extern "C" fn is_std_var(mut var: *const i8) -> i32 {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if strcmp((*vp).name, var) == 0 as i32 {
            if (do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 != 0
                || do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0)
                && (*vp).flags & 0x2 as i32 != 0 as i32
            {
                return 0 as i32;
            }
            return 1 as i32;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn is_off_limits_var(mut var: *const i8) -> i32 {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if strcmp((*vp).name, var) == 0 as i32 {
            return ((*vp).flags & 0x4 as i32 == 0 as i32) as i32;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_spec_varname(mut fptr: Func_ptr) -> *const i8 {
    let mut vp: *const varinit = 0 as *const varinit;
    if fptr.is_none() {
        return 0 as *const i8;
    }
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if (*vp).assign == fptr || (*vp).update == fptr {
            return (*vp).name;
        }
        vp = vp.offset(1);
        vp;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn arg_assign(mut arg: *mut i8, mut initing: bool) -> i32 {
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut badvar: bool = false;
    let mut var: *mut NODE = 0 as *mut NODE;
    let mut it: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut save_FNR: i64 = 0;
    if !initing && disallow_var_assigns as i32 != 0 {
        return 0 as i32;
    }
    cp = strchr(arg, '=' as i32);
    if cp.is_null() {
        if !initing {
            return 0 as i32;
        }
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: `%s' argument to `-v' not in `var=value' form\n\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            myname,
            arg,
        );
        usage(1 as i32, stderr);
    }
    let fresh2 = cp;
    cp = cp.offset(1);
    *fresh2 = '\0' as i32 as i8;
    source = 0 as *mut i8;
    sourceline = 0 as i32;
    save_FNR = FNR;
    FNR = 0 as i32 as i64;
    badvar = 0 as i32 != 0;
    if !is_letter(*arg.offset(0 as i32 as isize) as u8 as i32) {
        badvar = 1 as i32 != 0;
    } else {
        cp2 = arg.offset(1 as i32 as isize);
        while *cp2 != 0 {
            if !is_identchar(*cp2 as u8 as i32) && *cp2 as i32 != ':' as i32 {
                badvar = 1 as i32 != 0;
                break;
            } else {
                cp2 = cp2.offset(1);
                cp2;
            }
        }
    }
    if badvar {
        if initing {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 1240 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`%s' is not a legal variable name\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                arg,
            );
        }
        if do_flags as u32
            & (do_flag_values::DO_LINT_INVALID as i32
                | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
        {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 1243 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`%s' is not a variable name, looking for file `%s=%s'\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                arg,
                arg,
                cp,
            );
        }
    } else if !validate_qualified_name(arg) {
        badvar = 1 as i32 != 0;
    } else {
        if check_special(arg) >= 0 as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"main.c\0" as *const u8 as *const i8, 1257 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"cannot use gawk builtin `%s' as variable name\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                arg,
            );
        }
        if !initing {
            var = lookup(arg);
            if !var.is_null()
                && (*var).type_0 as u32 == nodevals::Node_func as i32 as u32
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"main.c\0" as *const u8 as *const i8, 1262 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"cannot use function `%s' as variable name\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    arg,
                );
            }
        }
        cp2 = cp.offset(strlen(cp) as isize).offset(-(1 as i32 as isize));
        if do_flags as u32 & do_flag_values::DO_TRADITIONAL as i32 as u32 == 0
            && strlen(cp) >= 3 as i32 as u64
            && *cp.offset(0 as i32 as isize) as i32 == '@' as i32
            && *cp.offset(1 as i32 as isize) as i32 == '/' as i32
            && *cp2 as i32 == '/' as i32
        {
            let mut len: size_t = (strlen(cp)).wrapping_sub(3 as i32 as u64);
            cp2 = ezalloc_real(
                len.wrapping_add(1 as i32 as u64),
                b"arg_assign\0" as *const u8 as *const i8,
                b"cp2\0" as *const u8 as *const i8,
                b"main.c\0" as *const u8 as *const i8,
                1272 as i32,
            ) as *mut i8;
            memcpy(
                cp2 as *mut libc::c_void,
                cp.offset(2 as i32 as isize) as *const libc::c_void,
                len,
            );
            it = make_typed_regex(cp2, len);
        } else {
            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0
                && !(strchr(cp, '\n' as i32)).is_null()
            {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"main.c\0" as *const u8 as *const i8, 1284 as i32);
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
            }
            it = make_str_node(cp, strlen(cp), 1 as i32 | 4 as i32);
            (*it).flags = ::core::mem::transmute::<
                u32,
                flagvals,
            >((*it).flags as u32 | flagvals::USER_INPUT as i32 as u32);
            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
                setlocale(1 as i32, b"C\0" as *const u8 as *const i8);
            }
            force_number(it);
            if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
                setlocale(1 as i32, locale);
            }
        }
        cp2 = estrdup(arg, cp.offset_from(arg) as i64 as size_t);
        var = variable(0 as i32, cp2, nodevals::Node_var);
        if var.is_null() {
            final_exit(2 as i32);
        }
        if (*var).type_0 as u32 == nodevals::Node_var as i32 as u32
            && ((*var).sub.nodep.r.uptr).is_some()
        {
            ((*var).sub.nodep.r.uptr).expect("non-null function pointer")();
        }
        lhs = if (*var).type_0 as u32 == nodevals::Node_var as i32 as u32
            && !((*var).sub.nodep.l.lptr == Nnull_string)
        {
            &mut (*var).sub.nodep.l.lptr
        } else {
            r_get_lhs(var, 0 as i32 != 0)
        };
        unref(*lhs);
        *lhs = it;
        if (*var).type_0 as u32 == nodevals::Node_var as i32 as u32
            && ((*var).sub.nodep.x.aptr).is_some()
        {
            ((*var).sub.nodep.x.aptr).expect("non-null function pointer")();
        }
    }
    if !initing {
        cp = cp.offset(-1);
        *cp = '=' as i32 as i8;
    }
    FNR = save_FNR;
    return !badvar as i32;
}
unsafe extern "C" fn catchsig(mut sig: i32) {
    if sig == 8 as i32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"main.c\0" as *const u8 as *const i8, 1341 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"floating point exception\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else if sig == 11 as i32 || sig == 7 as i32 {
        if errcount > 0 as i32 {
            exit(2 as i32);
        }
        set_loc(b"main.c\0" as *const u8 as *const i8, 1350 as i32);
        msg(
            dcgettext(
                0 as *const i8,
                b"fatal error: internal error\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fflush(0 as *mut FILE);
        abort();
    } else {
        r_fatal(
            b"internal error: file %s, line %d: unexpected signal, number %d (%s)\0"
                as *const u8 as *const i8,
            b"main.c\0" as *const u8 as *const i8,
            1357 as i32,
            sig,
            strsignal(sig),
        );
    };
}
unsafe extern "C" fn nostalgia() -> ! {
    fprintf(stderr, b"awk: bailing out near line 1\n\0" as *const u8 as *const i8);
    fflush(stderr);
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn get_pma_version() -> *const i8 {
    static mut buf: [i8; 200] = [0; 200];
    let mut open: *const i8 = 0 as *const i8;
    let mut close: *const i8 = 0 as *const i8;
    let mut out: *mut i8 = 0 as *mut i8;
    let mut in_0: *const i8 = 0 as *const i8;
    open = strchr(pma_version.as_ptr(), '(' as i32);
    if open.is_null() {
        return pma_version.as_ptr();
    }
    open = open.offset(1);
    open;
    close = strchr(open, ')' as i32);
    if close.is_null() {
        return pma_version.as_ptr();
    }
    out = buf.as_mut_ptr();
    in_0 = open;
    while in_0 < close {
        let fresh3 = in_0;
        in_0 = in_0.offset(1);
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 = *fresh3;
    }
    let fresh5 = out;
    out = out.offset(1);
    *fresh5 = '\0' as i32 as i8;
    return buf.as_mut_ptr();
}
unsafe extern "C" fn version() -> ! {
    printf(b"%s\0" as *const u8 as *const i8, version_string);
    printf(
        b", API %d.%d\0" as *const u8 as *const i8,
        C2RustUnnamed_0::GAWK_API_MAJOR_VERSION as i32,
        C2RustUnnamed_0::GAWK_API_MINOR_VERSION as i32,
    );
    printf(b", PMA %s\0" as *const u8 as *const i8, get_pma_version());
    printf(b"\n\0" as *const u8 as *const i8);
    print_ext_versions();
    copyleft();
}
unsafe extern "C" fn init_fds() {
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
    let mut newfd: i32 = 0;
    let opposite_mode: [*const i8; 3] = [
        b"w\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
    ];
    fd = 0 as i32;
    while fd <= 2 as i32 {
        if fstat(fd, &mut sbuf) < 0 as i32 {
            newfd = devopen(
                b"/dev/null\0" as *const u8 as *const i8,
                opposite_mode[fd as usize],
            );
            newfd += 0 as i32;
        }
        fd += 1;
        fd;
    }
}
unsafe extern "C" fn init_groupset() {
    ngroups = getgroups(0 as i32, 0 as *mut __gid_t);
    if ngroups <= 0 as i32 {
        return;
    }
    groupset = emalloc_real(
        (ngroups as u64).wrapping_mul(::core::mem::size_of::<gid_t>() as u64),
        b"init_groupset\0" as *const u8 as *const i8,
        b"groupset\0" as *const u8 as *const i8,
        b"main.c\0" as *const u8 as *const i8,
        1517 as i32,
    ) as *mut gid_t;
    ngroups = getgroups(ngroups, groupset);
    if ngroups == -(1 as i32) {
        pma_free(groupset as *mut libc::c_void);
        ngroups = 0 as i32;
        groupset = 0 as *mut gid_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn estrdup(mut str: *const i8, mut len: size_t) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    s = emalloc_real(
        len.wrapping_add(1 as i32 as u64),
        b"estrdup\0" as *const u8 as *const i8,
        b"s\0" as *const u8 as *const i8,
        b"main.c\0" as *const u8 as *const i8,
        1535 as i32,
    ) as *mut i8;
    memcpy(s as *mut libc::c_void, str as *const libc::c_void, len);
    *s.offset(len as isize) = '\0' as i32 as i8;
    return s;
}
unsafe extern "C" fn init_locale(mut l: *mut lconv) {
    let mut t: *mut lconv = 0 as *mut lconv;
    t = localeconv();
    *l = *t;
    (*l).thousands_sep = estrdup((*t).thousands_sep, strlen((*t).thousands_sep));
    (*l).decimal_point = estrdup((*t).decimal_point, strlen((*t).decimal_point));
    (*l).grouping = estrdup((*t).grouping, strlen((*t).grouping));
    (*l).int_curr_symbol = estrdup((*t).int_curr_symbol, strlen((*t).int_curr_symbol));
    (*l).currency_symbol = estrdup((*t).currency_symbol, strlen((*t).currency_symbol));
    (*l).mon_decimal_point = estrdup(
        (*t).mon_decimal_point,
        strlen((*t).mon_decimal_point),
    );
    (*l).mon_thousands_sep = estrdup(
        (*t).mon_thousands_sep,
        strlen((*t).mon_thousands_sep),
    );
    (*l).mon_grouping = estrdup((*t).mon_grouping, strlen((*t).mon_grouping));
    (*l).positive_sign = estrdup((*t).positive_sign, strlen((*t).positive_sign));
    (*l).negative_sign = estrdup((*t).negative_sign, strlen((*t).negative_sign));
}
unsafe extern "C" fn save_argv(mut argc: i32, mut argv: *mut *mut i8) {
    let mut i: i32 = 0;
    d_argv = emalloc_real(
        ((argc + 1 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
        b"save_argv\0" as *const u8 as *const i8,
        b"d_argv\0" as *const u8 as *const i8,
        b"main.c\0" as *const u8 as *const i8,
        1580 as i32,
    ) as *mut *mut i8;
    i = 0 as i32;
    while i < argc {
        let ref mut fresh6 = *d_argv.offset(i as isize);
        *fresh6 = estrdup(*argv.offset(i as isize), strlen(*argv.offset(i as isize)));
        i += 1;
        i;
    }
    let ref mut fresh7 = *d_argv.offset(argc as isize);
    *fresh7 = 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn update_global_values() {
    let mut vp: *const varinit = 0 as *const varinit;
    vp = varinit.as_ptr();
    while !((*vp).name).is_null() {
        if ((*vp).update).is_some() {
            ((*vp).update).expect("non-null function pointer")();
        }
        vp = vp.offset(1);
        vp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn getenv_long(mut name: *const i8) -> i64 {
    let mut val: *const i8 = 0 as *const i8;
    let mut newval: i64 = 0;
    val = getenv(name);
    if !val.is_null()
        && *(*__ctype_b_loc()).offset(*val as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        newval = 0 as i32 as i64;
        while *val as i32 != 0
            && *(*__ctype_b_loc()).offset(*val as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            newval = newval * 10 as i32 as i64 + *val as i64 - '0' as i32 as i64;
            val = val.offset(1);
            val;
        }
        return newval;
    }
    return -(1 as i32) as i64;
}
unsafe extern "C" fn parse_args(mut argc: i32, mut argv: *mut *mut i8) {
    let mut current_block: u64;
    let mut optlist: *const i8 = b"+F:f:v:W;bcCd::D::e:E:ghi:Il:L::nNo::Op::MPrSstVYZ:\0"
        as *const u8 as *const i8;
    let mut old_optind: i32 = 0;
    let mut c: i32 = 0;
    let mut scan: *mut i8 = 0 as *mut i8;
    let mut src: *mut i8 = 0 as *mut i8;
    opterr = 0 as i32;
    save_argv(argc, argv);
    optopt = 0 as i32;
    old_optind = 1 as i32;
    loop {
        c = getopt_long(argc, argv, optlist, optab.as_ptr(), 0 as *mut i32);
        if !(c != -(1 as i32)) {
            break;
        }
        if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 != 0 {
            opterr = 1 as i32;
        }
        match c {
            70 => {
                add_preassign(assign_type::PRE_ASSIGN_FS, optarg);
                current_block = 576355610076403033;
            }
            69 => {
                disallow_var_assigns = 1 as i32 != 0;
                current_block = 17458185132258431322;
            }
            102 => {
                current_block = 17458185132258431322;
            }
            118 => {
                add_preassign(assign_type::PRE_ASSIGN, optarg);
                current_block = 576355610076403033;
            }
            98 => {
                do_binary = 1 as i32;
                current_block = 576355610076403033;
            }
            99 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_TRADITIONAL as i32 as u32);
                current_block = 576355610076403033;
            }
            67 => {
                copyleft();
            }
            100 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_DUMP_VARS as i32 as u32);
                if !optarg.is_null()
                    && *optarg.offset(0 as i32 as isize) as i32 != '\0' as i32
                {
                    varfile = optarg;
                }
                current_block = 576355610076403033;
            }
            68 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_DEBUG as i32 as u32);
                if !optarg.is_null()
                    && *optarg.offset(0 as i32 as isize) as i32 != '\0' as i32
                {
                    command_file = optarg;
                }
                current_block = 576355610076403033;
            }
            101 => {
                if *optarg.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"main.c\0" as *const u8 as *const i8, 1704 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"empty argument to `-e/--source' ignored\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                } else {
                    add_srcfile(
                        srctype::SRC_CMDLINE,
                        optarg,
                        srcfiles,
                        0 as *mut bool,
                        0 as *mut i32,
                    );
                }
                current_block = 576355610076403033;
            }
            103 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_INTL as i32 as u32);
                current_block = 576355610076403033;
            }
            104 => {
                usage(0 as i32, stdout);
            }
            105 => {
                add_srcfile(
                    srctype::SRC_INC,
                    optarg,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut i32,
                );
                current_block = 576355610076403033;
            }
            73 => {
                do_itrace = 1 as i32 != 0;
                current_block = 576355610076403033;
            }
            108 => {
                add_srcfile(
                    srctype::SRC_EXTLIB,
                    optarg,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut i32,
                );
                current_block = 576355610076403033;
            }
            76 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(
                    do_flags as u32
                        | (do_flag_values::DO_LINT_ALL as i32
                            | do_flag_values::DO_LINT_EXTENSIONS as i32) as u32,
                );
                if !optarg.is_null() {
                    if strcmp(optarg, b"fatal\0" as *const u8 as *const i8) == 0 as i32 {
                        lintfunc = Some(
                            r_fatal as unsafe extern "C" fn(*const i8, ...) -> (),
                        );
                    } else if strcmp(optarg, b"invalid\0" as *const u8 as *const i8)
                        == 0 as i32
                    {
                        do_flags = ::core::mem::transmute::<
                            u32,
                            do_flag_values,
                        >(
                            do_flags as u32
                                & !(do_flag_values::DO_LINT_ALL as i32) as u32,
                        );
                        do_flags = ::core::mem::transmute::<
                            u32,
                            do_flag_values,
                        >(
                            do_flags as u32
                                | do_flag_values::DO_LINT_INVALID as i32 as u32,
                        );
                    } else if strcmp(optarg, b"no-ext\0" as *const u8 as *const i8)
                        == 0 as i32
                    {
                        do_flags = ::core::mem::transmute::<
                            u32,
                            do_flag_values,
                        >(
                            do_flags as u32
                                & !(do_flag_values::DO_LINT_EXTENSIONS as i32) as u32,
                        );
                    }
                }
                current_block = 576355610076403033;
            }
            116 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_LINT_OLD as i32 as u32);
                current_block = 576355610076403033;
            }
            110 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_NON_DEC_DATA as i32 as u32);
                current_block = 576355610076403033;
            }
            78 => {
                use_lc_numeric = 1 as i32;
                current_block = 576355610076403033;
            }
            79 => {
                do_optimize = 1 as i32 != 0;
                current_block = 576355610076403033;
            }
            112 => {
                if do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 != 0 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"main.c\0" as *const u8 as *const i8, 1769 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`--profile' overrides `--pretty-print'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_PROFILE as i32 as u32);
                current_block = 3800893665139888790;
            }
            111 => {
                current_block = 3800893665139888790;
            }
            77 => {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"main.c\0" as *const u8 as *const i8, 1786 as i32);
                (Some(
                    (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"-M ignored: MPFR/GMP support not compiled in\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                current_block = 576355610076403033;
            }
            80 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_POSIX as i32 as u32);
                current_block = 576355610076403033;
            }
            114 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_INTERVALS as i32 as u32);
                current_block = 576355610076403033;
            }
            115 => {
                do_optimize = 0 as i32 != 0;
                current_block = 576355610076403033;
            }
            83 => {
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_SANDBOX as i32 as u32);
                current_block = 576355610076403033;
            }
            84 => {
                if optarg.is_null() {
                    optarg = b"/some/file\0" as *const u8 as *const i8 as *mut i8;
                }
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"main.c\0" as *const u8 as *const i8, 1812 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"Use `GAWK_PERSIST_FILE=%s gawk ...' instead of --persist.\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    optarg,
                );
                current_block = 576355610076403033;
            }
            86 => {
                do_version = 1 as i32;
                current_block = 576355610076403033;
            }
            87 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: option `-W %s' unrecognized, ignored\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    *argv.offset(0 as i32 as isize),
                    optarg,
                );
                current_block = 576355610076403033;
            }
            0 => {
                current_block = 576355610076403033;
            }
            89 | 90 => {
                current_block = 4200263936204880529;
            }
            63 | _ => {
                current_block = 4200263936204880529;
            }
        }
        match current_block {
            4200263936204880529 => {
                if do_flags as u32 & do_flag_values::DO_POSIX as i32 as u32 == 0
                    && (optopt == '\0' as i32 || (strchr(optlist, optopt)).is_null())
                {
                    optind = old_optind;
                    stopped_early = 1 as i32 != 0;
                    break;
                } else if optopt != '\0' as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: option requires an argument -- %c\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        myname,
                        optopt,
                    );
                    usage(1 as i32, stderr);
                }
            }
            3800893665139888790 => {
                if c == 'o' as i32
                    && do_flags as u32 & do_flag_values::DO_PROFILE as i32 as u32 != 0
                {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"main.c\0" as *const u8 as *const i8, 1774 as i32);
                    (Some(
                        (Some(r_warning as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"`--profile' overrides `--pretty-print'\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                do_flags = ::core::mem::transmute::<
                    u32,
                    do_flag_values,
                >(do_flags as u32 | do_flag_values::DO_PRETTY_PRINT as i32 as u32);
                if !optarg.is_null() {
                    set_prof_file(optarg);
                } else {
                    set_prof_file(b"awkprof.out\0" as *const u8 as *const i8);
                }
            }
            17458185132258431322 => {
                scan = optarg;
                if *argv.offset((optind - 1 as i32) as isize) != optarg {
                    while *(*__ctype_b_loc()).offset(*scan as u8 as i32 as isize) as i32
                        & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
                    {
                        scan = scan.offset(1);
                        scan;
                    }
                }
                src = if *scan as i32 == '\0' as i32 {
                    let fresh8 = optind;
                    optind = optind + 1;
                    *argv.offset(fresh8 as isize)
                } else {
                    optarg
                };
                add_srcfile(
                    srctype::from_libc_c_uint(
                        (if !src.is_null()
                            && *src.offset(0 as i32 as isize) as i32 == '-' as i32
                            && *src.offset(1 as i32 as isize) as i32 == '\0' as i32
                        {
                            srctype::SRC_STDIN as i32
                        } else {
                            srctype::SRC_FILE as i32
                        }) as u32,
                    ),
                    src,
                    srcfiles,
                    0 as *mut bool,
                    0 as *mut i32,
                );
            }
            _ => {}
        }
        if c == 'E' as i32 {
            break;
        }
        optopt = 0 as i32;
        old_optind = optind;
    }
    do_optimize = do_optimize as i32 != 0
        && do_flags as u32 & do_flag_values::DO_PRETTY_PRINT as i32 as u32 == 0;
    pma_mpfr_check();
}
unsafe extern "C" fn set_locale_stuff() {
    setlocale(0 as i32, locale);
    setlocale(3 as i32, locale);
    setlocale(5 as i32, locale);
    setlocale(1 as i32, locale);
    init_locale(&mut loc);
    setlocale(1 as i32, b"C\0" as *const u8 as *const i8);
    setlocale(2 as i32, locale);
    bindtextdomain(b"gawk\0" as *const u8 as *const i8, locale_dir);
    textdomain(b"gawk\0" as *const u8 as *const i8);
}
unsafe extern "C" fn platform_name() -> *const i8 {
    return b"posix\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn set_current_namespace(mut new_namespace: *const i8) {
    if current_namespace != awk_namespace.as_ptr() {
        pma_free(current_namespace as *mut libc::c_void);
    }
    current_namespace = new_namespace;
}
unsafe extern "C" fn check_pma_security(mut pma_file: *const i8) {
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
    let mut euid: i32 = geteuid() as i32;
    if pma_file.is_null() {
        return
    } else if stat(pma_file, &mut sbuf) < 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: fatal: cannot stat %s: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            myname,
            pma_file,
            strerror(*__errno_location()),
        );
        exit(2 as i32);
    } else if euid == 0 as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: fatal: using persistent memory is not allowed when running as root.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            myname,
        );
        exit(2 as i32);
    } else if sbuf.st_uid != euid as u32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: warning: %s is not owned by euid %d.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            myname,
            pma_file,
            euid,
        );
    }
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