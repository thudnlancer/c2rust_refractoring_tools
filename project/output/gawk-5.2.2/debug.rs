#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type instruction_block;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn setbuf(__stream: *mut FILE, __buf: *mut i8);
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn unsetenv(__name: *const i8) -> i32;
    fn setenv(__name: *const i8, __value: *const i8, __replace: i32) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn execvp(__file: *const i8, __argv: *const *mut i8) -> i32;
    static ruletab: [*const i8; 0];
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut Nnull_string: *mut NODE;
    static mut Null_field: *mut NODE;
    static mut sourceline: i32;
    static mut source: *mut i8;
    static mut interpret: Option<unsafe extern "C" fn(*mut INSTRUCTION) -> i32>;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut srcfiles: *mut SRCFILE;
    static mut do_flags: do_flag_values;
    static mut exit_val: i32;
    static mut stack_ptr: *mut STACK_ITEM;
    static mut frame_ptr: *mut NODE;
    static mut stack_top: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    static mut fatal_tag: jmp_buf;
    static mut fatal_tag_valid: i32;
    fn make_array() -> *mut NODE;
    fn array_vname(symbol: *const NODE) -> *const i8;
    fn assoc_list(
        symbol: *mut NODE,
        sort_str: *const i8,
        sort_ctxt: sort_context_t,
    ) -> *mut *mut NODE;
    fn parse_program(pcode: *mut *mut INSTRUCTION, from_eval: bool) -> i32;
    fn getfname(
        _: Option<unsafe extern "C" fn(i32) -> *mut NODE>,
        prepend_awk: bool,
    ) -> *const i8;
    fn add_srcfile(
        stype: srctype,
        src: *mut i8,
        curr: *mut SRCFILE,
        already_included: *mut bool,
        errcode: *mut i32,
    ) -> *mut SRCFILE;
    fn free_srcfile(thisfile: *mut SRCFILE);
    fn files_are_same(path: *mut i8, src: *mut SRCFILE) -> i32;
    fn valinfo(n: *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn format_tree(_: *const i8, _: size_t, _: *mut *mut NODE, _: i64) -> *mut NODE;
    fn op2str(type_0: OPCODE) -> *const i8;
    fn in_main_context() -> i32;
    fn print_vars(table: *mut *mut NODE, print_func: Func_print, fp: *mut FILE);
    fn function_list(sort: bool) -> *mut *mut NODE;
    fn variable_list() -> *mut *mut NODE;
    fn foreach_func(
        table: *mut *mut NODE,
        _: Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
        _: *mut libc::c_void,
    ) -> i32;
    fn free_context(ctxt: *mut AWK_CONTEXT, keep_globals: bool);
    static mut func_table: *mut NODE;
    fn find_source(
        src: *const i8,
        stb: *mut stat,
        errcode: *mut i32,
        is_extlib: i32,
    ) -> *mut i8;
    fn get_field(num: i64, assign: *mut Func_ptr) -> *mut *mut NODE;
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn os_setbinmode(fd: i32, mode: i32) -> i32;
    fn srcopen(s: *mut SRCFILE) -> i32;
    fn pop_context();
    fn os_isatty(fd: i32) -> i32;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn os_devopen(name: *const i8, flag: i32) -> i32;
    fn close_extensions();
    fn close_io(stdio_problem: *mut bool, got_EPIPE: *mut bool) -> i32;
    fn nextfile(curfile_0: *mut *mut IOBUF, skipping: bool) -> i32;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn push_context(ctxt: *mut AWK_CONTEXT);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn cmp_nodes(t1: *mut NODE, t2: *mut NODE, use_strcmp: bool) -> i32;
    fn nodetype2str(type_0: NODETYPE) -> *const i8;
    fn pp_string_fp(
        print_func: Func_print,
        fp: *mut FILE,
        str: *const i8,
        namelen: size_t,
        delim: i32,
        breaklines: bool,
    );
    fn flags2str(_: i32) -> *const i8;
    fn genflags2str(flagval: i32, tab: *const flagtab) -> *const i8;
    fn reset_record();
    fn opcode2str(type_0: OPCODE) -> *const i8;
    fn register_exec_hook(preh: Func_pre_exec, posth: Func_post_exec) -> i32;
    fn os_isdir(fd: i32) -> i32;
    fn bcalloc(op: OPCODE, size: i32, srcline: i32) -> *mut INSTRUCTION;
    fn remove_params(func: *mut NODE);
    fn append_symbol(r: *mut NODE);
    fn new_context() -> *mut AWK_CONTEXT;
    fn install_params(func: *mut NODE);
    fn grow_stack() -> *mut STACK_ITEM;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn lookup(name: *const i8) -> *mut NODE;
    static mut output_is_tty: bool;
    fn free_cmdarg(list: *mut CMDARG);
    fn get_command(ctype: i32) -> Func_cmd;
    static mut exiting: bool;
    static mut rule_list: *mut INSTRUCTION;
    static mut code_block: *mut INSTRUCTION;
    static mut fcall_list: *mut *mut NODE;
    static mut fcall_count: i64;
    static mut output_fp: *mut FILE;
    static mut curfile: *mut IOBUF;
    static mut command_file: *const i8;
    fn get_spec_varname(fptr: Func_ptr) -> *const i8;
    fn zzparse() -> i32;
    fn redir2str(redirtype: i32) -> *const i8;
    static mut d_argv: *mut *mut i8;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
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
pub type va_list = __builtin_va_list;
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub sub: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub nodep: C2RustUnnamed_2,
    pub val: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_4,
    pub x: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
    pub d: C2RustUnnamed_7,
    pub x: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
    pub xl: i64,
    pub xn: *mut NODE,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct break_point {
    pub next: *mut break_point,
    pub prev: *mut break_point,
    pub number: i32,
    pub ignore_count: i64,
    pub hit_count: i64,
    pub src: *mut i8,
    pub bpi: *mut INSTRUCTION,
    pub commands: commands_item,
    pub silent: bool,
    pub cndn: condition,
    pub flags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct condition {
    pub code: *mut INSTRUCTION,
    pub ctxt: *mut AWK_CONTEXT,
    pub expr: *mut i8,
}
pub type AWK_CONTEXT = context;
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
pub type NODE = exp_node;
pub type SRCFILE = srcfile;
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
pub type INSTRUCTION = exp_instruction;
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
pub type INSTRUCTION_POOL = instruction_pool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commands_item {
    pub next: *mut commands_item,
    pub prev: *mut commands_item,
    pub cmd: i32,
    pub cmd_string: *mut i8,
    pub arg: *mut CMDARG,
}
pub type CMDARG = cmd_argument;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_argument {
    pub next: *mut cmd_argument,
    pub type_0: argtype,
    pub value: C2RustUnnamed_6,
    pub a_count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub lval: i64,
    pub sval: *mut i8,
    pub nodeval: *mut NODE,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum argtype {
    D_illegal,
    D_backtrace,
    D_break,
    D_clear,
    D_commands,
    D_condition,
    D_continue,
    D_delete,
    D_disable,
    D_display,
    D_down,
    D_dump,
    D_enable,
    D_end,
    D_eval,
    D_finish,
    D_frame,
    D_help,
    D_ignore,
    D_info,
    D_list,
    D_next,
    D_nexti,
    D_option,
    D_print,
    D_printf,
    D_quit,
    D_return,
    D_run,
    D_save,
    D_set,
    D_silent,
    D_source,
    D_step,
    D_stepi,
    D_tbreak,
    D_trace,
    D_undisplay,
    D_until,
    D_unwatch,
    D_up,
    D_watch,
    D_argument,
    D_int,
    D_string,
    D_variable,
    D_node,
    D_field,
    D_array,
    D_subscript,
    D_func,
    D_range,
}
impl argtype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            argtype::D_illegal => 0,
            argtype::D_backtrace => 1,
            argtype::D_break => 2,
            argtype::D_clear => 3,
            argtype::D_commands => 4,
            argtype::D_condition => 5,
            argtype::D_continue => 6,
            argtype::D_delete => 7,
            argtype::D_disable => 8,
            argtype::D_display => 9,
            argtype::D_down => 10,
            argtype::D_dump => 11,
            argtype::D_enable => 12,
            argtype::D_end => 13,
            argtype::D_eval => 14,
            argtype::D_finish => 15,
            argtype::D_frame => 16,
            argtype::D_help => 17,
            argtype::D_ignore => 18,
            argtype::D_info => 19,
            argtype::D_list => 20,
            argtype::D_next => 21,
            argtype::D_nexti => 22,
            argtype::D_option => 23,
            argtype::D_print => 24,
            argtype::D_printf => 25,
            argtype::D_quit => 26,
            argtype::D_return => 27,
            argtype::D_run => 28,
            argtype::D_save => 29,
            argtype::D_set => 30,
            argtype::D_silent => 31,
            argtype::D_source => 32,
            argtype::D_step => 33,
            argtype::D_stepi => 34,
            argtype::D_tbreak => 35,
            argtype::D_trace => 36,
            argtype::D_undisplay => 37,
            argtype::D_until => 38,
            argtype::D_unwatch => 39,
            argtype::D_up => 40,
            argtype::D_watch => 41,
            argtype::D_argument => 42,
            argtype::D_int => 43,
            argtype::D_string => 44,
            argtype::D_variable => 45,
            argtype::D_node => 46,
            argtype::D_field => 47,
            argtype::D_array => 48,
            argtype::D_subscript => 49,
            argtype::D_func => 50,
            argtype::D_range => 51,
        }
    }
    fn from_libc_c_uint(value: u32) -> argtype {
        match value {
            0 => argtype::D_illegal,
            1 => argtype::D_backtrace,
            2 => argtype::D_break,
            3 => argtype::D_clear,
            4 => argtype::D_commands,
            5 => argtype::D_condition,
            6 => argtype::D_continue,
            7 => argtype::D_delete,
            8 => argtype::D_disable,
            9 => argtype::D_display,
            10 => argtype::D_down,
            11 => argtype::D_dump,
            12 => argtype::D_enable,
            13 => argtype::D_end,
            14 => argtype::D_eval,
            15 => argtype::D_finish,
            16 => argtype::D_frame,
            17 => argtype::D_help,
            18 => argtype::D_ignore,
            19 => argtype::D_info,
            20 => argtype::D_list,
            21 => argtype::D_next,
            22 => argtype::D_nexti,
            23 => argtype::D_option,
            24 => argtype::D_print,
            25 => argtype::D_printf,
            26 => argtype::D_quit,
            27 => argtype::D_return,
            28 => argtype::D_run,
            29 => argtype::D_save,
            30 => argtype::D_set,
            31 => argtype::D_silent,
            32 => argtype::D_source,
            33 => argtype::D_step,
            34 => argtype::D_stepi,
            35 => argtype::D_tbreak,
            36 => argtype::D_trace,
            37 => argtype::D_undisplay,
            38 => argtype::D_until,
            39 => argtype::D_unwatch,
            40 => argtype::D_up,
            41 => argtype::D_watch,
            42 => argtype::D_argument,
            43 => argtype::D_int,
            44 => argtype::D_string,
            45 => argtype::D_variable,
            46 => argtype::D_node,
            47 => argtype::D_field,
            48 => argtype::D_array,
            49 => argtype::D_subscript,
            50 => argtype::D_func,
            51 => argtype::D_range,
            _ => panic!("Invalid value for argtype: {}", value),
        }
    }
}
impl AddAssign<u32> for argtype {
    fn add_assign(&mut self, rhs: u32) {
        *self = argtype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for argtype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = argtype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for argtype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = argtype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for argtype {
    fn div_assign(&mut self, rhs: u32) {
        *self = argtype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for argtype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = argtype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for argtype {
    type Output = argtype;
    fn add(self, rhs: u32) -> argtype {
        argtype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for argtype {
    type Output = argtype;
    fn sub(self, rhs: u32) -> argtype {
        argtype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for argtype {
    type Output = argtype;
    fn mul(self, rhs: u32) -> argtype {
        argtype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for argtype {
    type Output = argtype;
    fn div(self, rhs: u32) -> argtype {
        argtype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for argtype {
    type Output = argtype;
    fn rem(self, rhs: u32) -> argtype {
        argtype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub hs: C2RustUnnamed_9,
    pub hi: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub next: *mut bucket_item,
    pub li: [i64; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub next: *mut bucket_item,
    pub str_0: *mut i8,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iobuf {
    pub public: awk_input_buf_t,
    pub buf: *mut i8,
    pub off: *mut i8,
    pub dataend: *mut i8,
    pub end: *mut i8,
    pub readsize: size_t,
    pub size: size_t,
    pub count: ssize_t,
    pub scanoff: size_t,
    pub valid: bool,
    pub errcode: i32,
    pub flag: iobuf_flags,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum iobuf_flags {
    IOP_IS_TTY = 1,
    IOP_AT_EOF = 2,
    IOP_CLOSED = 4,
    IOP_AT_START = 8,
}
impl iobuf_flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            iobuf_flags::IOP_IS_TTY => 1,
            iobuf_flags::IOP_AT_EOF => 2,
            iobuf_flags::IOP_CLOSED => 4,
            iobuf_flags::IOP_AT_START => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> iobuf_flags {
        match value {
            1 => iobuf_flags::IOP_IS_TTY,
            2 => iobuf_flags::IOP_AT_EOF,
            4 => iobuf_flags::IOP_CLOSED,
            8 => iobuf_flags::IOP_AT_START,
            _ => panic!("Invalid value for iobuf_flags: {}", value),
        }
    }
}
impl AddAssign<u32> for iobuf_flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for iobuf_flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for iobuf_flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for iobuf_flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for iobuf_flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn add(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn sub(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn mul(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn div(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for iobuf_flags {
    type Output = iobuf_flags;
    fn rem(self, rhs: u32) -> iobuf_flags {
        iobuf_flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type IOBUF = iobuf;
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagtab {
    pub val: i32,
    pub name: *const i8,
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
pub type Func_pre_exec = Option<unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32>;
pub type Func_post_exec = Option<unsafe extern "C" fn(*mut INSTRUCTION) -> ()>;
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
pub union stack_item {
    pub rptr: *mut NODE,
    pub lptr: *mut *mut NODE,
}
pub type STACK_ITEM = stack_item;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sort_context_t {
    SORTED_IN = 1,
    ASORT,
    ASORTI,
}
impl sort_context_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sort_context_t::SORTED_IN => 1,
            sort_context_t::ASORT => 2,
            sort_context_t::ASORTI => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> sort_context_t {
        match value {
            1 => sort_context_t::SORTED_IN,
            2 => sort_context_t::ASORT,
            3 => sort_context_t::ASORTI,
            _ => panic!("Invalid value for sort_context_t: {}", value),
        }
    }
}
impl AddAssign<u32> for sort_context_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sort_context_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sort_context_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sort_context_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sort_context_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sort_context_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sort_context_t {
    type Output = sort_context_t;
    fn add(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sort_context_t {
    type Output = sort_context_t;
    fn sub(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sort_context_t {
    type Output = sort_context_t;
    fn mul(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sort_context_t {
    type Output = sort_context_t;
    fn div(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sort_context_t {
    type Output = sort_context_t;
    fn rem(self, rhs: u32) -> sort_context_t {
        sort_context_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub fcall_count: i64,
    pub sourceline: i32,
    pub source: *mut i8,
    pub pc: *mut INSTRUCTION,
    pub repeat_count: i32,
    pub print_frame: bool,
    pub print_ret: bool,
    pub break_point: i32,
    pub watch_point: i32,
    pub check_func: Option<unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32>,
    pub command: argtype,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item {
    pub next: *mut list_item,
    pub prev: *mut list_item,
    pub number: i32,
    pub symbol: *mut NODE,
    pub subs: *mut *mut NODE,
    pub num_subs: i32,
    pub sname: *mut i8,
    pub fcall_count: i64,
    pub commands: commands_item,
    pub silent: i32,
    pub cndn: condition,
    pub value: [C2RustUnnamed_12; 2],
    pub flags: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub n: *mut NODE,
    pub l: i64,
}
pub type Func_cmd = Option<unsafe extern "C" fn(*mut CMDARG, i32) -> i32>;
pub type BREAKPOINT = break_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pf_data {
    pub print_func: Func_print,
    pub defn: bool,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command_source {
    pub fd: i32,
    pub is_tty: i32,
    pub read_func: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    pub close_func: Option<unsafe extern "C" fn(i32) -> i32>,
    pub eof_status: i32,
    pub cmd: i32,
    pub str_0: *mut i8,
    pub next: *mut command_source,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_13 {
    BREAK = 1,
    WATCH,
    DISPLAY,
    HISTORY,
    OPTION,
}
impl C2RustUnnamed_13 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_13::BREAK => 1,
            C2RustUnnamed_13::WATCH => 2,
            C2RustUnnamed_13::DISPLAY => 3,
            C2RustUnnamed_13::HISTORY => 4,
            C2RustUnnamed_13::OPTION => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_13 {
        match value {
            1 => C2RustUnnamed_13::BREAK,
            2 => C2RustUnnamed_13::WATCH,
            3 => C2RustUnnamed_13::DISPLAY,
            4 => C2RustUnnamed_13::HISTORY,
            5 => C2RustUnnamed_13::OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_13: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_13 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_13 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_13 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_13 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_13 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn add(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn sub(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn mul(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn div(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_13 {
    type Output = C2RustUnnamed_13;
    fn rem(self, rhs: u32) -> C2RustUnnamed_13 {
        C2RustUnnamed_13::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbg_option {
    pub name: *const i8,
    pub num_val: *mut i32,
    pub str_val: *mut *const i8,
    pub assign: Option<unsafe extern "C" fn(*const i8) -> ()>,
    pub help_txt: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nametypeval {
    A_NONE = 0,
    A_ARGS,
    A_BREAK,
    A_DEL,
    A_DISPLAY,
    A_FRAME,
    A_FUNCTIONS,
    A_LOCALS,
    A_ONCE,
    A_SOURCE,
    A_SOURCES,
    A_TRACE_ON,
    A_TRACE_OFF,
    A_VARIABLES,
    A_WATCH,
}
impl nametypeval {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            nametypeval::A_NONE => 0,
            nametypeval::A_ARGS => 1,
            nametypeval::A_BREAK => 2,
            nametypeval::A_DEL => 3,
            nametypeval::A_DISPLAY => 4,
            nametypeval::A_FRAME => 5,
            nametypeval::A_FUNCTIONS => 6,
            nametypeval::A_LOCALS => 7,
            nametypeval::A_ONCE => 8,
            nametypeval::A_SOURCE => 9,
            nametypeval::A_SOURCES => 10,
            nametypeval::A_TRACE_ON => 11,
            nametypeval::A_TRACE_OFF => 12,
            nametypeval::A_VARIABLES => 13,
            nametypeval::A_WATCH => 14,
        }
    }
    fn from_libc_c_uint(value: u32) -> nametypeval {
        match value {
            0 => nametypeval::A_NONE,
            1 => nametypeval::A_ARGS,
            2 => nametypeval::A_BREAK,
            3 => nametypeval::A_DEL,
            4 => nametypeval::A_DISPLAY,
            5 => nametypeval::A_FRAME,
            6 => nametypeval::A_FUNCTIONS,
            7 => nametypeval::A_LOCALS,
            8 => nametypeval::A_ONCE,
            9 => nametypeval::A_SOURCE,
            10 => nametypeval::A_SOURCES,
            11 => nametypeval::A_TRACE_ON,
            12 => nametypeval::A_TRACE_OFF,
            13 => nametypeval::A_VARIABLES,
            14 => nametypeval::A_WATCH,
            _ => panic!("Invalid value for nametypeval: {}", value),
        }
    }
}
impl AddAssign<u32> for nametypeval {
    fn add_assign(&mut self, rhs: u32) {
        *self = nametypeval::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for nametypeval {
    fn sub_assign(&mut self, rhs: u32) {
        *self = nametypeval::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for nametypeval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = nametypeval::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for nametypeval {
    fn div_assign(&mut self, rhs: u32) {
        *self = nametypeval::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for nametypeval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = nametypeval::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for nametypeval {
    type Output = nametypeval;
    fn add(self, rhs: u32) -> nametypeval {
        nametypeval::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for nametypeval {
    type Output = nametypeval;
    fn sub(self, rhs: u32) -> nametypeval {
        nametypeval::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for nametypeval {
    type Output = nametypeval;
    fn mul(self, rhs: u32) -> nametypeval {
        nametypeval::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for nametypeval {
    type Output = nametypeval;
    fn div(self, rhs: u32) -> nametypeval {
        nametypeval::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for nametypeval {
    type Output = nametypeval;
    fn rem(self, rhs: u32) -> nametypeval {
        nametypeval::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn DEREF(mut r: *mut NODE) {
    (*r).valref -= 1;
    if (*r).valref > 0 as i32 as i64 {
        return;
    }
    r_unref(r);
}
#[inline]
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
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
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn POP_SCALAR() -> *mut NODE {
    let fresh0 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh0).rptr;
    if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"./awk.h\0" as *const u8 as *const i8, 1881 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"attempt to use array `%s' in a scalar context\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            array_vname(t),
        );
    } else if (*t).type_0 as u32 == nodevals::Node_elem_new as i32 as u32 {
        t = elem_new_to_scalar(t);
    }
    return t;
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
static mut linebuf: *mut i8 = 0 as *const i8 as *mut i8;
static mut linebuf_len: size_t = 0;
#[no_mangle]
pub static mut out_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut dbg_prompt: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut commands_prompt: *const i8 = b"> \0" as *const u8 as *const i8;
#[no_mangle]
pub static mut eval_prompt: *const i8 = b"@> \0" as *const u8 as *const i8;
#[no_mangle]
pub static mut input_from_tty: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut input_fd: i32 = 0;
static mut cur_srcfile: *mut SRCFILE = 0 as *const SRCFILE as *mut SRCFILE;
static mut cur_frame: i64 = 0 as i32 as i64;
static mut cur_pc: *mut INSTRUCTION = 0 as *const INSTRUCTION as *mut INSTRUCTION;
static mut cur_rule: i32 = 0 as i32;
static mut prog_running: bool = 0 as i32 != 0;
static mut breakpoints: BREAKPOINT = unsafe {
    {
        let mut init = break_point {
            next: &breakpoints as *const BREAKPOINT as *mut BREAKPOINT,
            prev: &breakpoints as *const BREAKPOINT as *mut BREAKPOINT,
            number: 0 as i32,
            ignore_count: 0,
            hit_count: 0,
            src: 0 as *const i8 as *mut i8,
            bpi: 0 as *const INSTRUCTION as *mut INSTRUCTION,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const i8 as *mut i8,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: false,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const i8 as *mut i8,
            },
            flags: 0,
        };
        init
    }
};
static mut last_printed_line: i32 = 0 as i32;
static mut last_print_count: i32 = 0;
static mut display_list: list_item = unsafe {
    {
        let mut init = list_item {
            next: &display_list as *const list_item as *mut list_item,
            prev: &display_list as *const list_item as *mut list_item,
            number: 0 as i32,
            symbol: 0 as *const NODE as *mut NODE,
            subs: 0 as *const *mut NODE as *mut *mut NODE,
            num_subs: 0,
            sname: 0 as *const i8 as *mut i8,
            fcall_count: 0,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const i8 as *mut i8,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: 0,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const i8 as *mut i8,
            },
            value: [C2RustUnnamed_12 {
                n: 0 as *const NODE as *mut NODE,
            }; 2],
            flags: 0,
        };
        init
    }
};
static mut watch_list: list_item = unsafe {
    {
        let mut init = list_item {
            next: &watch_list as *const list_item as *mut list_item,
            prev: &watch_list as *const list_item as *mut list_item,
            number: 0 as i32,
            symbol: 0 as *const NODE as *mut NODE,
            subs: 0 as *const *mut NODE as *mut *mut NODE,
            num_subs: 0,
            sname: 0 as *const i8 as *mut i8,
            fcall_count: 0,
            commands: commands_item {
                next: 0 as *const commands_item as *mut commands_item,
                prev: 0 as *const commands_item as *mut commands_item,
                cmd: 0,
                cmd_string: 0 as *const i8 as *mut i8,
                arg: 0 as *const CMDARG as *mut CMDARG,
            },
            silent: 0,
            cndn: condition {
                code: 0 as *const INSTRUCTION as *mut INSTRUCTION,
                ctxt: 0 as *const AWK_CONTEXT as *mut AWK_CONTEXT,
                expr: 0 as *const i8 as *mut i8,
            },
            value: [C2RustUnnamed_12 {
                n: 0 as *const NODE as *mut NODE,
            }; 2],
            flags: 0,
        };
        init
    }
};
static mut stop: C2RustUnnamed_11 = C2RustUnnamed_11 {
    fcall_count: 0,
    sourceline: 0,
    source: 0 as *const i8 as *mut i8,
    pc: 0 as *const INSTRUCTION as *mut INSTRUCTION,
    repeat_count: 0,
    print_frame: false,
    print_ret: false,
    break_point: 0,
    watch_point: 0,
    check_func: None,
    command: argtype::D_illegal,
};
static mut need_restart: bool = 0 as i32 != 0;
static mut env_variable: [*const i8; 6] = [
    b"\0" as *const u8 as *const i8,
    b"DGAWK_BREAK\0" as *const u8 as *const i8,
    b"DGAWK_WATCH\0" as *const u8 as *const i8,
    b"DGAWK_DISPLAY\0" as *const u8 as *const i8,
    b"DGAWK_HISTORY\0" as *const u8 as *const i8,
    b"DGAWK_OPTION\0" as *const u8 as *const i8,
];
static mut commands_string: *const i8 = 0 as *const i8;
static mut commands_string_len: i32 = 0 as i32;
static mut line_sep: i8 = 0;
static mut options_file: *const i8 = b"./.gawkrc\0" as *const u8 as *const i8;
static mut output_file: *const i8 = b"/dev/stdout\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut dgawk_prompt: *const i8 = 0 as *const i8;
static mut list_size: i32 = 15 as i32;
static mut do_trace: i32 = 0 as i32;
static mut do_save_history: i32 = 1 as i32;
static mut do_save_options: i32 = 1 as i32;
static mut history_size: i32 = 100 as i32;
static mut option_list: [dbg_option; 8] = unsafe {
    [
        {
            let mut init = dbg_option {
                name: b"history_size\0" as *const u8 as *const i8,
                num_val: &history_size as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: Some(set_history_size as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"set or show the number of lines to keep in history file\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"listsize\0" as *const u8 as *const i8,
                num_val: &list_size as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: Some(set_listsize as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"set or show the list command window size\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"outfile\0" as *const u8 as *const i8,
                num_val: 0 as *const i32 as *mut i32,
                str_val: &output_file as *const *const i8 as *mut *const i8,
                assign: Some(set_gawk_output as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"set or show gawk output file\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"prompt\0" as *const u8 as *const i8,
                num_val: 0 as *const i32 as *mut i32,
                str_val: &dgawk_prompt as *const *const i8 as *mut *const i8,
                assign: Some(set_prompt as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"set or show debugger prompt\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"save_history\0" as *const u8 as *const i8,
                num_val: &do_save_history as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: Some(set_save_history as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"(un)set or show saving of command history (value=on|off)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"save_options\0" as *const u8 as *const i8,
                num_val: &do_save_options as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: Some(set_save_options as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"(un)set or show saving of options (value=on|off)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: b"trace\0" as *const u8 as *const i8,
                num_val: &do_trace as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: Some(set_trace as unsafe extern "C" fn(*const i8) -> ()),
                help_txt: b"(un)set or show instruction tracing (value=on|off)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = dbg_option {
                name: 0 as *const i8,
                num_val: 0 as *const i32 as *mut i32,
                str_val: 0 as *const *const i8 as *mut *const i8,
                assign: None,
                help_txt: 0 as *const i8,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut pager_quit_tag: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut pager_quit_tag_valid: i32 = 0 as i32;
static mut screen_width: i32 = 2147483647 as i32;
static mut screen_height: i32 = 2147483647 as i32;
static mut pager_lines_printed: i32 = 0 as i32;
static mut pf_data: pf_data = pf_data {
    print_func: None,
    defn: false,
    fp: 0 as *const FILE as *mut FILE,
};
#[no_mangle]
pub static mut read_a_line: Option<unsafe extern "C" fn(*const i8) -> *mut i8> = None;
static mut cmd_src: *mut command_source = 0 as *const command_source
    as *mut command_source;
unsafe extern "C" fn g_readline(mut prompt: *const i8) -> *mut i8 {
    let mut line: *mut i8 = 0 as *mut i8;
    let mut line_size: size_t = 100 as i32 as size_t;
    static mut buf: [i8; 2] = [0; 2];
    let mut p: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut n: i32 = 0;
    if input_from_tty as i32 != 0 && !prompt.is_null() && *prompt as i32 != 0 {
        fprintf(out_fp, b"%s\0" as *const u8 as *const i8, prompt);
    }
    line = emalloc_real(
        line_size.wrapping_add(1 as i32 as u64),
        b"g_readline\0" as *const u8 as *const i8,
        b"line\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        390 as i32,
    ) as *mut i8;
    p = line;
    end = line.offset(line_size as isize);
    loop {
        n = read(input_fd, buf.as_mut_ptr() as *mut libc::c_void, 1 as i32 as size_t)
            as i32;
        if !(n > 0 as i32) {
            break;
        }
        if buf[0 as i32 as usize] as i32 == '\n' as i32 {
            if p > line && *p.offset(-(1 as i32) as isize) as i32 == '\r' as i32 {
                p = p.offset(-1);
                p;
            }
            break;
        } else {
            if p == end {
                line = erealloc_real(
                    line as *mut libc::c_void,
                    (2 as i32 as u64)
                        .wrapping_mul(line_size)
                        .wrapping_add(1 as i32 as u64),
                    b"g_readline\0" as *const u8 as *const i8,
                    b"line\0" as *const u8 as *const i8,
                    b"debug.c\0" as *const u8 as *const i8,
                    400 as i32,
                ) as *mut i8;
                p = line.offset(line_size as isize);
                line_size = (line_size as u64).wrapping_mul(2 as i32 as u64) as size_t
                    as size_t;
                end = line.offset(line_size as isize);
            }
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = buf[0 as i32 as usize];
        }
    }
    if n == -(1 as i32) || n == 0 as i32 && p == line {
        pma_free(line as *mut libc::c_void);
        return 0 as *mut i8;
    }
    *p = '\0' as i32 as i8;
    return line;
}
#[no_mangle]
pub unsafe extern "C" fn d_error(mut mesg: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(
        out_fp,
        dcgettext(0 as *const i8, b"error: \0" as *const u8 as *const i8, 5 as i32),
    );
    vfprintf(out_fp, mesg, args_0.as_va_list());
    fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
}
unsafe extern "C" fn find_lines(mut s: *mut SRCFILE) -> i32 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut n: i32 = 0;
    let mut ofs: i32 = 0 as i32;
    let mut pos: *mut i32 = 0 as *mut i32;
    let mut pos_size: i32 = 0;
    let mut maxlen: i32 = 0 as i32;
    let mut numlines: i32 = 0 as i32;
    let mut lastchar: i8 = '\0' as i32 as i8;
    buf = emalloc_real(
        (*s).bufsize,
        b"find_lines\0" as *const u8 as *const i8,
        b"buf\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        443 as i32,
    ) as *mut i8;
    pos_size = (*s).srclines;
    (*s).line_offset = emalloc_real(
        ((pos_size + 2 as i32) as u64)
            .wrapping_mul(::core::mem::size_of::<i32>() as u64),
        b"find_lines\0" as *const u8 as *const i8,
        b"s->line_offset\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        445 as i32,
    ) as *mut i32;
    pos = (*s).line_offset;
    *pos.offset(0 as i32 as isize) = 0 as i32;
    loop {
        n = read((*s).fd, buf as *mut libc::c_void, (*s).bufsize) as i32;
        if !(n > 0 as i32) {
            break;
        }
        end = buf.offset(n as isize);
        lastchar = *buf.offset((n - 1 as i32) as isize);
        p = buf;
        while p < end {
            let fresh2 = p;
            p = p.offset(1);
            if *fresh2 as i32 == '\n' as i32 {
                numlines += 1;
                if numlines > pos_size {
                    (*s).line_offset = erealloc_real(
                        (*s).line_offset as *mut libc::c_void,
                        ((2 as i32 * pos_size + 2 as i32) as u64)
                            .wrapping_mul(::core::mem::size_of::<i32>() as u64),
                        b"find_lines\0" as *const u8 as *const i8,
                        b"s->line_offset\0" as *const u8 as *const i8,
                        b"debug.c\0" as *const u8 as *const i8,
                        456 as i32,
                    ) as *mut i32;
                    pos = ((*s).line_offset).offset(pos_size as isize);
                    pos_size *= 2 as i32;
                }
                pos = pos.offset(1);
                *pos = (ofs as i64 + p.offset_from(buf) as i64) as i32;
                if *pos.offset(0 as i32 as isize) - *pos.offset(-(1 as i32) as isize)
                    > maxlen
                {
                    maxlen = *pos.offset(0 as i32 as isize)
                        - *pos.offset(-(1 as i32) as isize);
                }
            }
        }
        ofs += n;
    }
    pma_free(buf as *mut libc::c_void);
    if n == -(1 as i32) {
        d_error(
            dcgettext(
                0 as *const i8,
                b"cannot read source file `%s': %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*s).src,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    if ofs <= 0 as i32 {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"source file `%s' is empty.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*s).src,
        );
        return -(1 as i32);
    }
    if lastchar as i32 != '\n' as i32 {
        pos = pos.offset(1);
        *pos = ofs + 1 as i32;
        numlines += 1;
        numlines;
        if *pos.offset(0 as i32 as isize) - *pos.offset(-(1 as i32) as isize) > maxlen {
            maxlen = *pos.offset(0 as i32 as isize) - *pos.offset(-(1 as i32) as isize);
        }
    }
    (*s).maxlen = maxlen;
    (*s).srclines = numlines;
    return 0 as i32;
}
unsafe extern "C" fn source_find(mut src: *mut i8) -> *mut SRCFILE {
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
    if src.is_null() || *src as i32 == '\0' as i32 {
        d_error(
            dcgettext(
                0 as *const i8,
                b"no current source file\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as *mut SRCFILE;
    }
    if (*cur_srcfile).src == src {
        return cur_srcfile;
    }
    s = (*srcfiles).next;
    while s != srcfiles {
        if ((*s).stype as u32 == srctype::SRC_FILE as i32 as u32
            || (*s).stype as u32 == srctype::SRC_INC as i32 as u32)
            && strcmp((*s).src, src) == 0 as i32
        {
            return s;
        }
        s = (*s).next;
    }
    path = find_source(src, &mut sbuf, &mut errno_val, 0 as i32);
    if !path.is_null() {
        s = (*srcfiles).next;
        while s != srcfiles {
            if ((*s).stype as u32 == srctype::SRC_FILE as i32 as u32
                || (*s).stype as u32 == srctype::SRC_INC as i32 as u32)
                && files_are_same(path, s) != 0
            {
                pma_free(path as *mut libc::c_void);
                return s;
            }
            s = (*s).next;
        }
        pma_free(path as *mut libc::c_void);
    }
    d_error(
        dcgettext(
            0 as *const i8,
            b"cannot find source file named `%s': %s\0" as *const u8 as *const i8,
            5 as i32,
        ),
        src,
        strerror(errno_val),
    );
    return 0 as *mut SRCFILE;
}
unsafe extern "C" fn print_lines(
    mut src: *mut i8,
    mut start_line: i32,
    mut nlines: i32,
) -> i32 {
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    let mut pos: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
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
    s = source_find(src);
    if s.is_null() {
        return -(1 as i32);
    }
    if (*s).fd <= -(1 as i32)
        && {
            (*s).fd = srcopen(s);
            (*s).fd <= -(1 as i32)
        }
    {
        d_error(
            dcgettext(
                0 as *const i8,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            src,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    if fstat((*s).fd, &mut sbuf) == 0 as i32 && (*s).mtime < sbuf.st_mtim.tv_sec {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"warning: source file `%s' modified since program compilation.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            src,
        );
        pma_free((*s).line_offset as *mut libc::c_void);
        (*s).line_offset = 0 as *mut i32;
        (*s).mtime = sbuf.st_mtim.tv_sec;
        close((*s).fd);
        (*s).fd = -(1 as i32);
        (*s).fd = srcopen(s);
        if (*s).fd <= -(1 as i32) {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                src,
                strerror(*__errno_location()),
            );
            return -(1 as i32);
        }
    }
    os_setbinmode((*s).fd, 0 as i32);
    if ((*s).line_offset).is_null() && find_lines(s) != 0 as i32 {
        return -(1 as i32);
    }
    if start_line < 1 as i32 || start_line > (*s).srclines {
        d_error(
            dcgettext(
                0 as *const i8,
                b"line number %d out of range; `%s' has %d lines\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            start_line,
            src,
            (*s).srclines,
        );
        return -(1 as i32);
    }
    if start_line + nlines - 1 as i32 > (*s).srclines {
        nlines = (*s).srclines - start_line + 1 as i32;
    }
    pos = (*s).line_offset;
    if lseek((*s).fd, *pos.offset((start_line - 1 as i32) as isize) as off_t, 0 as i32)
        < 0 as i32 as i64
    {
        d_error(
            b"%s: %s\0" as *const u8 as *const i8,
            src,
            strerror(*__errno_location()),
        );
        return -(1 as i32);
    }
    if linebuf.is_null() {
        linebuf = emalloc_real(
            ((*s).maxlen + 20 as i32) as size_t,
            b"print_lines\0" as *const u8 as *const i8,
            b"linebuf\0" as *const u8 as *const i8,
            b"debug.c\0" as *const u8 as *const i8,
            589 as i32,
        ) as *mut i8;
        linebuf_len = (*s).maxlen as size_t;
    } else if linebuf_len < (*s).maxlen as u64 {
        linebuf = erealloc_real(
            linebuf as *mut libc::c_void,
            ((*s).maxlen + 20 as i32) as size_t,
            b"print_lines\0" as *const u8 as *const i8,
            b"linebuf\0" as *const u8 as *const i8,
            b"debug.c\0" as *const u8 as *const i8,
            592 as i32,
        ) as *mut i8;
        linebuf_len = (*s).maxlen as size_t;
    }
    i = start_line;
    while i < start_line + nlines {
        let mut supposed_len: i32 = 0;
        let mut len: i32 = 0;
        let mut p: *mut i8 = 0 as *mut i8;
        sprintf(linebuf, b"%-8d\0" as *const u8 as *const i8, i);
        if nlines > 1 as i32 {
            let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
            let mut has_bpt: bool = 0 as i32 != 0;
            b = breakpoints.prev;
            while b != &mut breakpoints as *mut BREAKPOINT {
                if src == (*b).src && i == (*(*b).bpi).source_line as i32 {
                    has_bpt = 1 as i32 != 0;
                    break;
                } else {
                    b = (*b).prev;
                }
            }
            if prog_running as i32 != 0 && src == source && i == sourceline {
                if has_bpt {
                    sprintf(linebuf, b"%-4d:b=>\0" as *const u8 as *const i8, i);
                } else {
                    sprintf(linebuf, b"%-4d  =>\0" as *const u8 as *const i8, i);
                }
            } else if has_bpt {
                sprintf(linebuf, b"%-4d:b  \0" as *const u8 as *const i8, i);
            }
        }
        p = linebuf.offset(strlen(linebuf) as isize);
        supposed_len = *pos.offset(i as isize) - *pos.offset((i - 1 as i32) as isize);
        len = read((*s).fd, p as *mut libc::c_void, supposed_len as size_t) as i32;
        match len {
            -1 => {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"cannot read source file `%s': %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    src,
                    strerror(*__errno_location()),
                );
                return -(1 as i32);
            }
            0 => {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"unexpected eof while reading file `%s', line %d\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    src,
                    i,
                );
                return -(1 as i32);
            }
            _ => {
                if i == (*s).srclines
                    && *p.offset((len - 1 as i32) as isize) as i32 != '\n' as i32
                {
                    let fresh3 = len;
                    len = len + 1;
                    *p.offset(fresh3 as isize) = '\n' as i32 as i8;
                }
                len = (len as i64 + p.offset_from(linebuf) as i64) as i32;
                if (if 0 != 0 && 0 != 0
                    && (::core::mem::size_of::<i8>() as u64).wrapping_mul(len as size_t)
                        <= 8 as i32 as u64
                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                {
                    ({
                        let mut __ptr: *const i8 = linebuf as *const i8;
                        let mut __stream: *mut FILE = out_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::core::mem::size_of::<i8>() as u64)
                            .wrapping_mul(len as size_t);
                        while __cnt > 0 as i32 as u64 {
                            if (if ((*__stream)._IO_write_ptr
                                >= (*__stream)._IO_write_end) as i32 as i64 != 0
                            {
                                let fresh4 = __ptr;
                                __ptr = __ptr.offset(1);
                                __overflow(__stream, *fresh4 as u8 as i32)
                            } else {
                                let fresh5 = __ptr;
                                __ptr = __ptr.offset(1);
                                let fresh6 = (*__stream)._IO_write_ptr;
                                (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                    .offset(1);
                                *fresh6 = *fresh5;
                                *fresh6 as u8 as i32
                            }) == -(1 as i32)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        (::core::mem::size_of::<i8>() as u64)
                            .wrapping_mul(len as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    })
                } else {
                    (if 0 != 0 && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                        || 0 != 0 && len as size_t == 0 as i32 as u64
                    {
                        0 as i32 as size_t
                    } else {
                        fwrite_unlocked(
                            linebuf as *const libc::c_void,
                            ::core::mem::size_of::<i8>() as u64,
                            len as size_t,
                            out_fp,
                        )
                    })
                }) != len as u64
                {
                    return -(1 as i32);
                }
            }
        }
        i += 1;
        i;
    }
    if cur_srcfile != s {
        if (*cur_srcfile).fd != -(1 as i32) {
            close((*cur_srcfile).fd);
            (*cur_srcfile).fd = -(1 as i32);
        }
        cur_srcfile = s;
    }
    return i - 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_list(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut line_first: i64 = 0;
    let mut line_last: i64 = 0;
    let mut count: i64 = list_size as i64;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut src: *mut i8 = (*cur_srcfile).src;
    line_first = (last_printed_line + 1 as i32) as i64;
    if !arg.is_null() {
        let mut current_block_24: u64;
        match (*arg).type_0 as u32 {
            43 => {
                if (*arg).value.lval < 0 as i32 as i64 {
                    line_first = (last_printed_line - last_print_count - list_size
                        + 1 as i32) as i64;
                    if line_first < 1 as i32 as i64 {
                        if last_printed_line != last_print_count {
                            line_first = 1 as i32 as i64;
                        } else {
                            return 0 as i32
                        }
                    }
                    current_block_24 = 2891135413264362348;
                } else {
                    current_block_24 = 5795995263004476895;
                }
            }
            51 => {
                current_block_24 = 11989267069235126506;
            }
            44 => {
                src = (*arg).value.sval;
                if !((*arg).next).is_null() {
                    arg = (*arg).next;
                    if (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
                        current_block_24 = 5795995263004476895;
                    } else if (*arg).type_0 as u32 == argtype::D_range as i32 as u32 {
                        current_block_24 = 11989267069235126506;
                    } else if (*arg).type_0 as u32 == argtype::D_func as i32 as u32 {
                        current_block_24 = 4158637261959553548;
                    } else {
                        line_first = 1 as i32 as i64;
                        current_block_24 = 2891135413264362348;
                    }
                } else {
                    line_first = 1 as i32 as i64;
                    current_block_24 = 2891135413264362348;
                }
            }
            50 => {
                current_block_24 = 4158637261959553548;
            }
            _ => {
                current_block_24 = 2891135413264362348;
            }
        }
        match current_block_24 {
            4158637261959553548 => {
                rp = (*(*arg).value.nodeval).sub.nodep.r.iptr;
                src = (*rp).d.name;
                line_first = ((*rp).source_line as i32 - list_size / 2 as i32) as i64;
                if line_first < 1 as i32 as i64 {
                    line_first = 1 as i32 as i64;
                }
            }
            11989267069235126506 => {
                line_first = (*arg).value.lval;
                arg = (*arg).next;
                count = (*arg).value.lval - line_first + 1 as i32 as i64;
            }
            5795995263004476895 => {
                line_first = (*arg).value.lval - (list_size / 2 as i32) as i64;
                if line_first < 1 as i32 as i64 {
                    line_first = 1 as i32 as i64;
                }
            }
            _ => {}
        }
    }
    line_last = print_lines(src, line_first as i32, count as i32) as i64;
    if line_last != -(1 as i32) as i64 {
        last_printed_line = line_last as i32;
        last_print_count = (line_last - line_first + 1 as i32 as i64) as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_info(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut table: *mut *mut NODE = 0 as *mut *mut NODE;
    if arg.is_null() || (*arg).type_0 as u32 != argtype::D_argument as i32 as u32 {
        return 0 as i32;
    }
    match (*arg).value.lval {
        9 => {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Current source file: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*cur_srcfile).src,
            );
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Number of lines: %d\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*cur_srcfile).srclines,
            );
        }
        10 => {
            let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
            s = (*srcfiles).next;
            while s != srcfiles {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Source file (lines): %s (%d)\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    if (*s).stype as u32 == srctype::SRC_FILE as i32 as u32
                        || (*s).stype as u32 == srctype::SRC_INC as i32 as u32
                    {
                        (*s).src
                    } else {
                        b"cmd. line\0" as *const u8 as *const i8
                    },
                    (*s).srclines,
                );
                s = (*s).next;
            }
        }
        2 => {
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
                let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
                let mut c: *mut commands_item = 0 as *mut commands_item;
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Number  Disp  Enabled  Location\n\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                b = breakpoints.prev;
                while b != &mut breakpoints as *mut BREAKPOINT {
                    let mut disp: *const i8 = b"keep\0" as *const u8 as *const i8;
                    if (*b).flags as i32 & 2 as i32 != 0 as i32 {
                        disp = b"dis\0" as *const u8 as *const i8;
                    } else if (*b).flags as i32 & 4 as i32 != 0 as i32 {
                        disp = b"del\0" as *const u8 as *const i8;
                    }
                    gprintf(
                        out_fp,
                        b"%-6d  %-4.4s  %-7.7s  file %s, line #%d\n\0" as *const u8
                            as *const i8,
                        (*b).number,
                        disp,
                        if (*b).flags as i32 & 1 as i32 != 0 as i32 {
                            b"yes\0" as *const u8 as *const i8
                        } else {
                            b"no\0" as *const u8 as *const i8
                        },
                        (*b).src,
                        (*(*b).bpi).source_line as i32,
                    );
                    if (*b).hit_count > 0 as i32 as i64 {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tnumber of hits = %ld\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*b).hit_count,
                        );
                    }
                    if (*b).flags as i32 & 8 as i32 != 0 as i32 {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tignore next %ld hit(s)\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*b).ignore_count,
                        );
                    }
                    if !((*b).cndn.code).is_null() {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tstop condition: %s\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*b).cndn.expr,
                        );
                    }
                    if (*b).commands.next != &mut (*b).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tcommands:\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    c = (*b).commands.next;
                    while c != &mut (*b).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            b"\t%s\n\0" as *const u8 as *const i8,
                            (*c).cmd_string,
                        );
                        if (*c).cmd == argtype::D_eval as i32 {
                            let mut start: *mut i8 = 0 as *mut i8;
                            let mut end: *mut i8 = 0 as *mut i8;
                            let mut a: *mut CMDARG = (*c).arg;
                            start = strchr((*a).value.sval, '{' as i32);
                            end = strrchr((*a).value.sval, '}' as i32);
                            if !(start.is_null() || end.is_null()) {
                                start = start.offset(1);
                                start;
                                *end = '\0' as i32 as i8;
                                gprintf(out_fp, b"%s\0" as *const u8 as *const i8, start);
                                *end = '}' as i32 as i8;
                            }
                        }
                        c = (*c).next;
                    }
                    b = (*b).prev;
                }
            }
        }
        5 => {
            if !prog_running {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"program not running\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as i32;
            }
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Current frame: \0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            print_numbered_frame(cur_frame);
            if cur_frame < fcall_count {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Called by frame: \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                print_numbered_frame(cur_frame + 1 as i32 as i64);
            }
            if cur_frame > 0 as i32 as i64 {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Caller of frame: \0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                print_numbered_frame(cur_frame - 1 as i32 as i64);
            }
        }
        1 | 7 => {
            let mut f: *mut NODE = 0 as *mut NODE;
            let mut func: *mut NODE = 0 as *mut NODE;
            let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            let mut arg_count: i32 = 0;
            let mut pcount: i32 = 0;
            let mut i: i32 = 0;
            let mut from: i32 = 0;
            let mut to: i32 = 0;
            if !prog_running {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"program not running\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as i32;
            }
            f = find_frame(cur_frame);
            func = (*f).sub.nodep.x.extra;
            if func.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"None in main().\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as i32;
            }
            pcount = (*func).sub.nodep.l.ll as i32;
            pc = (*f).sub.nodep.l.li as *mut INSTRUCTION;
            arg_count = (*pc.offset(1 as i32 as isize)).x.xl as i32;
            if arg_count > pcount {
                arg_count = pcount;
            }
            if (*arg).value.lval == nametypeval::A_ARGS as i32 as i64 {
                from = 0 as i32;
                to = arg_count - 1 as i32;
            } else {
                from = arg_count;
                to = pcount - 1 as i32;
            }
            i = from;
            while i <= to {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = *((*f).sub.nodep.r.av).offset(i as isize);
                if (*r).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
                    r = (*r).sub.nodep.l.lptr;
                }
                fprintf(
                    out_fp,
                    b"%s = \0" as *const u8 as *const i8,
                    (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name,
                );
                print_symbol(r, 1 as i32 != 0);
                i += 1;
                i;
            }
            if to < from {
                fprintf(
                    out_fp,
                    b"%s\0" as *const u8 as *const i8,
                    if (*arg).value.lval == nametypeval::A_ARGS as i32 as i64 {
                        dcgettext(
                            0 as *const i8,
                            b"No arguments.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        )
                    } else {
                        dcgettext(
                            0 as *const i8,
                            b"No locals.\n\0" as *const u8 as *const i8,
                            5 as i32,
                        )
                    },
                );
            }
        }
        13 => {
            table = variable_list();
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"All defined variables:\n\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                print_vars(
                    table,
                    Some(
                        gprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                    ),
                    out_fp,
                );
            }
            pma_free(table as *mut libc::c_void);
        }
        6 => {
            table = function_list(1 as i32 != 0);
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
                gprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"All defined functions:\n\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                pf_data.print_func = Some(
                    gprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                );
                pf_data.fp = out_fp;
                pf_data.defn = 1 as i32 != 0;
                foreach_func(
                    table,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *mut INSTRUCTION,
                                *mut libc::c_void,
                            ) -> i32,
                        >,
                        Option<
                            unsafe extern "C" fn(
                                *mut INSTRUCTION,
                                *mut libc::c_void,
                            ) -> i32,
                        >,
                    >(
                        Some(
                            print_function
                                as unsafe extern "C" fn(
                                    *mut INSTRUCTION,
                                    *mut libc::c_void,
                                ) -> i32,
                        ),
                    ),
                    &mut pf_data as *mut pf_data as *mut libc::c_void,
                );
            }
            pma_free(table as *mut libc::c_void);
        }
        4 | 14 => {
            if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
                let mut d: *mut list_item = 0 as *mut list_item;
                let mut list: *mut list_item = 0 as *mut list_item;
                if (*arg).value.lval == nametypeval::A_DISPLAY as i32 as i64 {
                    list = &mut display_list;
                    gprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"Auto-display variables:\n\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                } else {
                    list = &mut watch_list;
                    gprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"Watch variables:\n\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                d = (*list).prev;
                while d != list {
                    let mut i_0: i32 = 0;
                    let mut c_0: *mut commands_item = 0 as *mut commands_item;
                    let mut symbol: *mut NODE = (*d).symbol;
                    if (*d).flags & 2 as i32 != 0 as i32 {
                        gprintf(
                            out_fp,
                            b"%d:\t%s\0" as *const u8 as *const i8,
                            (*d).number,
                            (*d).sname,
                        );
                        i_0 = 0 as i32;
                        while i_0 < (*d).num_subs {
                            let mut sub: *mut NODE = 0 as *mut NODE;
                            sub = *((*d).subs).offset(i_0 as isize);
                            gprintf(
                                out_fp,
                                b"[\"%.*s\"]\0" as *const u8 as *const i8,
                                (*sub).sub.val.slen as i32,
                                (*sub).sub.val.sp,
                            );
                            i_0 += 1;
                            i_0;
                        }
                        gprintf(out_fp, b"\n\0" as *const u8 as *const i8);
                    } else if (*d).flags & 4 as i32 != 0 as i32 {
                        gprintf(
                            out_fp,
                            b"%d:\t$%ld\n\0" as *const u8 as *const i8,
                            (*d).number,
                            (*symbol).sub.val.fltnum as i64,
                        );
                    } else {
                        gprintf(
                            out_fp,
                            b"%d:\t%s\n\0" as *const u8 as *const i8,
                            (*d).number,
                            (*d).sname,
                        );
                    }
                    if !((*d).cndn.code).is_null() {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tstop condition: %s\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*d).cndn.expr,
                        );
                    }
                    if (*d).commands.next != &mut (*d).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"\tcommands:\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                    }
                    c_0 = (*d).commands.next;
                    while c_0 != &mut (*d).commands as *mut commands_item {
                        gprintf(
                            out_fp,
                            b"\t%s\n\0" as *const u8 as *const i8,
                            (*c_0).cmd_string,
                        );
                        if (*c_0).cmd == argtype::D_eval as i32 {
                            let mut start_0: *mut i8 = 0 as *mut i8;
                            let mut end_0: *mut i8 = 0 as *mut i8;
                            let mut a_0: *mut CMDARG = (*c_0).arg;
                            start_0 = strchr((*a_0).value.sval, '{' as i32);
                            end_0 = strrchr((*a_0).value.sval, '}' as i32);
                            if !(start_0.is_null() || end_0.is_null()) {
                                start_0 = start_0.offset(1);
                                start_0;
                                *end_0 = '\0' as i32 as i8;
                                gprintf(out_fp, b"%s\0" as *const u8 as *const i8, start_0);
                                *end_0 = '}' as i32 as i8;
                            }
                        }
                        c_0 = (*c_0).next;
                    }
                    d = (*d).prev;
                }
            }
        }
        _ => {}
    }
    return 0 as i32;
}
unsafe extern "C" fn print_symbol(mut r: *mut NODE, mut isparam: bool) {
    match (*r).type_0 as u32 {
        6 => {
            fprintf(out_fp, b"untyped variable\n\0" as *const u8 as *const i8);
        }
        7 => {
            fprintf(out_fp, b"untyped element\n\0" as *const u8 as *const i8);
        }
        4 => {
            if !isparam && ((*r).sub.nodep.r.uptr).is_some() {
                ((*r).sub.nodep.r.uptr).expect("non-null function pointer")();
            }
            valinfo(
                (*r).sub.nodep.l.lptr,
                Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
                out_fp,
            );
        }
        5 => {
            fprintf(
                out_fp,
                b"array, %ld elements\n\0" as *const u8 as *const i8,
                (*r).sub.nodep.reflags as i64,
            );
        }
        9 => {
            fprintf(out_fp, b"`function'\n\0" as *const u8 as *const i8);
        }
        _ => {}
    };
}
unsafe extern "C" fn find_frame(mut num: i64) -> *mut NODE {
    if num == 0 as i32 as i64 {
        return frame_ptr;
    }
    return *fcall_list.offset(num as isize);
}
unsafe extern "C" fn find_param(
    mut name: *const i8,
    mut num: i64,
    mut pname: *mut *mut i8,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut fparam: *mut i8 = 0 as *mut i8;
    if !pname.is_null() {
        *pname = 0 as *mut i8;
    }
    if num < 0 as i32 as i64 || num > fcall_count || name.is_null() {
        return 0 as *mut NODE;
    }
    f = find_frame(num);
    if !((*f).sub.nodep.x.extra).is_null() {
        let mut func: *mut NODE = 0 as *mut NODE;
        let mut i: i32 = 0;
        let mut pcount: i32 = 0;
        func = (*f).sub.nodep.x.extra;
        pcount = (*func).sub.nodep.l.ll as i32;
        i = 0 as i32;
        while i < pcount {
            fparam = (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name;
            if strcmp(name, fparam) == 0 as i32 {
                r = *((*f).sub.nodep.r.av).offset(i as isize);
                if (*r).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
                    r = (*r).sub.nodep.l.lptr;
                }
                if !pname.is_null() {
                    *pname = fparam;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    return r;
}
unsafe extern "C" fn find_symbol(
    mut name: *const i8,
    mut pname: *mut *mut i8,
) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    if !pname.is_null() {
        *pname = 0 as *mut i8;
    }
    if prog_running {
        r = find_param(name, cur_frame, pname);
    }
    if r.is_null() {
        r = lookup(name);
    }
    if r.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"no symbol `%s' in current context\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    return r;
}
unsafe extern "C" fn find_array(mut name: *const i8) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    r = find_symbol(name, 0 as *mut *mut i8);
    if !r.is_null() && (*r).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"`%s' is not an array\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
        return 0 as *mut NODE;
    }
    return r;
}
unsafe extern "C" fn print_field(mut field_num: i64) {
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    lhs = get_field(field_num, 0 as *mut Func_ptr);
    if *lhs == Null_field || *lhs == Nnull_string {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"$%ld = uninitialized field\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            field_num,
        );
    } else {
        fprintf(out_fp, b"$%ld = \0" as *const u8 as *const i8, field_num);
        valinfo(
            *lhs,
            Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
            out_fp,
        );
    };
}
unsafe extern "C" fn print_array(mut arr: *mut NODE, mut arr_name: *mut i8) -> i32 {
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut i: i32 = 0;
    let mut num_elems: size_t = 0 as i32 as size_t;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: i32 = 0 as i32;
    let mut pager_quit_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    if (*(arr as *mut NODE)).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        gprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"array `%s' is empty\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            arr_name,
        );
        return 0 as i32;
    }
    num_elems = (*(arr as *mut NODE)).sub.nodep.reflags as size_t;
    list = assoc_list(
        arr as *mut NODE,
        b"@ind_str_asc\0" as *const u8 as *const i8,
        sort_context_t::SORTED_IN,
    );
    let fresh7 = pager_quit_tag_valid;
    pager_quit_tag_valid = pager_quit_tag_valid + 1;
    if fresh7 != 0 {
        memcpy(
            pager_quit_tag_stack.as_mut_ptr() as *mut i8 as *mut libc::c_void,
            pager_quit_tag.as_mut_ptr() as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<jmp_buf>() as u64,
        );
    }
    if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
        i = 0 as i32;
        while ret == 0 as i32 && (i as u64) < num_elems {
            subs = *list.offset(i as isize);
            r = *((*(*(arr as *mut NODE)).sub.nodep.l.lp).lookup)
                .expect("non-null function pointer")(arr as *mut NODE, subs)
                as *mut NODE;
            if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                ::core::ptr::write_volatile(
                    &mut ret as *mut i32,
                    print_array(r, (*r).sub.nodep.name),
                );
            } else {
                gprintf(
                    out_fp,
                    b"%s[\"%.*s\"] = \0" as *const u8 as *const i8,
                    arr_name,
                    (*subs).sub.val.slen as i32,
                    (*subs).sub.val.sp,
                );
                valinfo(
                    r as *mut NODE,
                    Some(
                        gprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                    ),
                    out_fp,
                );
            }
            i += 1;
            i;
        }
    } else {
        ::core::ptr::write_volatile(&mut ret as *mut i32, 1 as i32);
    }
    pager_quit_tag_valid -= 1;
    if pager_quit_tag_valid != 0 {
        memcpy(
            pager_quit_tag.as_mut_ptr() as *mut i8 as *mut libc::c_void,
            pager_quit_tag_stack.as_mut_ptr() as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<jmp_buf>() as u64,
        );
    }
    i = 0 as i32;
    while (i as u64) < num_elems {
        unref(*list.offset(i as isize));
        i += 1;
        i;
    }
    pma_free(list as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn print_subscript(
    mut arr: *mut NODE,
    mut arr_name: *mut i8,
    mut a: *mut CMDARG,
    mut count: i32,
) {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    subs = (*a).value.nodeval;
    r = in_array(arr, subs);
    if r.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"subscript \"%.*s\" is not in array `%s'\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*subs).sub.val.slen as i32,
            (*subs).sub.val.sp,
            arr_name,
        );
    } else if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        if count > 1 as i32 {
            print_subscript(r, (*r).sub.nodep.name, (*a).next, count - 1 as i32);
        } else {
            fprintf(out_fp, b"%s = \0" as *const u8 as *const i8, (*r).sub.nodep.name);
            print_symbol(r, 0 as i32 != 0);
        }
    } else {
        fprintf(
            out_fp,
            b"%s[\"%.*s\"] = \0" as *const u8 as *const i8,
            arr_name,
            (*subs).sub.val.slen as i32,
            (*subs).sub.val.sp,
        );
        valinfo(
            r,
            Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
            out_fp,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_print_var(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut pname: *mut i8 = 0 as *mut i8;
    a = arg;
    while !a.is_null() {
        match (*a).type_0 as u32 {
            45 => {
                name = (*a).value.sval;
                r = find_symbol(name, &mut pname);
                if !r.is_null() {
                    fprintf(out_fp, b"%s = \0" as *const u8 as *const i8, name);
                    print_symbol(r, !pname.is_null());
                }
            }
            49 => {
                name = (*a).value.sval;
                r = find_array(name);
                if !r.is_null() {
                    print_subscript(r, name, (*a).next, (*a).a_count);
                }
            }
            48 => {
                name = (*a).value.sval;
                r = find_array(name);
                if !r.is_null() {
                    let mut count: i32 = (*a).a_count;
                    while count > 0 as i32 {
                        let mut value: *mut NODE = 0 as *mut NODE;
                        let mut subs: *mut NODE = 0 as *mut NODE;
                        a = (*a).next;
                        subs = (*a).value.nodeval;
                        value = in_array(r, subs);
                        if value.is_null() {
                            fprintf(
                                out_fp,
                                dcgettext(
                                    0 as *const i8,
                                    b"subscript \"%.*s\" is not in array `%s'\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (*subs).sub.val.slen as i32,
                                (*subs).sub.val.sp,
                                name,
                            );
                            break;
                        } else if (*value).type_0 as u32
                            != nodevals::Node_var_array as i32 as u32
                        {
                            fprintf(
                                out_fp,
                                dcgettext(
                                    0 as *const i8,
                                    b"`%s[\"%.*s\"]' is not an array\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                name,
                                (*subs).sub.val.slen as i32,
                                (*subs).sub.val.sp,
                            );
                            break;
                        } else {
                            r = value;
                            name = (*r).sub.nodep.name;
                            count -= 1;
                            count;
                        }
                    }
                    if count == 0 as i32 {
                        print_array(r as *mut NODE, name);
                    }
                }
            }
            47 => {
                print_field((*(*a).value.nodeval).sub.val.fltnum as i64);
            }
            _ => {}
        }
        a = (*a).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_set_var(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut lhs: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut pname: *mut i8 = 0 as *mut i8;
    match (*arg).type_0 as u32 {
        45 => {
            name = (*arg).value.sval;
            arg = (*arg).next;
            val = (*arg).value.nodeval;
            r = find_symbol(name, &mut pname);
            if !r.is_null() {
                let mut current_block_12: u64;
                match (*r).type_0 as u32 {
                    6 | 7 => {
                        (*r).type_0 = nodevals::Node_var;
                        (*r).sub.nodep.l.lptr = dupnode(Nnull_string);
                        current_block_12 = 9359659890357492930;
                    }
                    4 => {
                        current_block_12 = 9359659890357492930;
                    }
                    _ => {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"`%s' is not a scalar variable\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            name,
                        );
                        current_block_12 = 12349973810996921269;
                    }
                }
                match current_block_12 {
                    9359659890357492930 => {
                        lhs = &mut (*r).sub.nodep.l.lptr;
                        unref(*lhs);
                        *lhs = dupnode(val);
                        if pname.is_null() && ((*r).sub.nodep.x.aptr).is_some() {
                            ((*r).sub.nodep.x.aptr)
                                .expect("non-null function pointer")();
                        }
                        fprintf(out_fp, b"%s = \0" as *const u8 as *const i8, name);
                        print_symbol(r, !pname.is_null());
                    }
                    _ => {}
                }
            }
        }
        49 => {
            let mut subs: *mut NODE = 0 as *mut NODE;
            let mut value: *mut NODE = 0 as *mut NODE;
            let mut count: i32 = (*arg).a_count;
            let mut newval: *mut NODE = 0 as *mut NODE;
            name = (*arg).value.sval;
            r = find_array(name);
            if !r.is_null() {
                while count > 0 as i32 {
                    arg = (*arg).next;
                    subs = (*arg).value.nodeval;
                    value = in_array(r, subs);
                    if count == 1 as i32 {
                        if !value.is_null()
                            && (*value).type_0 as u32
                                == nodevals::Node_var_array as i32 as u32
                        {
                            d_error(
                                dcgettext(
                                    0 as *const i8,
                                    b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                name,
                                (*subs).sub.val.slen as i32,
                                (*subs).sub.val.sp,
                            );
                        } else {
                            arg = (*arg).next;
                            val = (*arg).value.nodeval;
                            newval = dupnode(val);
                            assoc_set(r, dupnode(subs), newval);
                            fprintf(
                                out_fp,
                                b"%s[\"%.*s\"] = \0" as *const u8 as *const i8,
                                name,
                                (*subs).sub.val.slen as i32,
                                (*subs).sub.val.sp,
                            );
                            valinfo(
                                newval,
                                Some(
                                    fprintf
                                        as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                                ),
                                out_fp,
                            );
                        }
                    } else if value.is_null() {
                        let mut array: *mut NODE = 0 as *mut NODE;
                        array = make_array();
                        (*array).sub.nodep.name = estrdup(
                            (*subs).sub.val.sp,
                            (*subs).sub.val.slen,
                        );
                        (*array).sub.nodep.x.extra = r;
                        assoc_set(r, dupnode(subs), array);
                        r = array;
                    } else if (*value).type_0 as u32
                        != nodevals::Node_var_array as i32 as u32
                    {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"attempt to use scalar `%s[\"%.*s\"]' as array\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            name,
                            (*subs).sub.val.slen as i32,
                            (*subs).sub.val.sp,
                        );
                        break;
                    } else {
                        r = value;
                        name = (*r).sub.nodep.name;
                    }
                    count -= 1;
                    count;
                }
            }
        }
        47 => {
            let mut field_num: i64 = 0;
            let mut assign: Func_ptr = None;
            field_num = (*(*arg).value.nodeval).sub.val.fltnum as i64;
            arg = (*arg).next;
            val = (*arg).value.nodeval;
            lhs = get_field(field_num, &mut assign);
            if assign.is_some() {
                assign.expect("non-null function pointer")();
            }
            unref(*lhs);
            *lhs = dupnode(val);
            print_field(field_num);
        }
        _ => {}
    }
    return 0 as i32;
}
unsafe extern "C" fn find_item(
    mut list: *mut list_item,
    mut num: i64,
) -> *mut list_item {
    let mut d: *mut list_item = 0 as *mut list_item;
    if num <= 0 as i32 as i64 {
        return 0 as *mut list_item;
    }
    d = (*list).next;
    while d != list {
        if (*d).number as i64 == num {
            return d;
        }
        d = (*d).next;
    }
    return 0 as *mut list_item;
}
unsafe extern "C" fn delete_item(mut d: *mut list_item) {
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut i: i32 = 0;
    if (*d).flags & 2 as i32 != 0 as i32 {
        i = 0 as i32;
        while i < (*d).num_subs {
            unref(*((*d).subs).offset(i as isize));
            i += 1;
            i;
        }
        pma_free((*d).subs as *mut libc::c_void);
    } else if (*d).flags & 4 as i32 != 0 as i32 {
        unref((*d).symbol);
    }
    if (*d).flags & 16 as i32 == 0 as i32 {
        unref((*d).value[0 as i32 as usize].n);
    }
    if (*d).flags & 8 as i32 == 0 as i32 {
        unref((*d).value[1 as i32 as usize].n);
    }
    c = (*d).commands.next;
    while c != &mut (*d).commands as *mut commands_item {
        c = (*c).prev;
        delete_commands_item((*c).next);
        c = (*c).next;
    }
    free_context((*d).cndn.ctxt, 0 as i32 != 0);
    if !((*d).cndn.expr).is_null() {
        pma_free((*d).cndn.expr as *mut libc::c_void);
    }
    (*(*d).next).prev = (*d).prev;
    (*(*d).prev).next = (*d).next;
    pma_free(d as *mut libc::c_void);
}
unsafe extern "C" fn add_item(
    mut list: *mut list_item,
    mut type_0: i32,
    mut symbol: *mut NODE,
    mut pname: *mut i8,
) -> *mut list_item {
    let mut d: *mut list_item = 0 as *mut list_item;
    d = ezalloc_real(
        ::core::mem::size_of::<list_item>() as u64,
        b"add_item\0" as *const u8 as *const i8,
        b"d\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        1403 as i32,
    ) as *mut list_item;
    (*d).commands.prev = &mut (*d).commands;
    (*d).commands.next = (*d).commands.prev;
    (*list).number += 1;
    (*d).number = (*list).number;
    (*d).sname = (*symbol).sub.nodep.name;
    if !pname.is_null() {
        (*d).sname = pname;
        (*d).flags |= 1 as i32;
        (*d).fcall_count = fcall_count - cur_frame;
    }
    if type_0 == argtype::D_field as i32 {
        (*d).symbol = symbol;
        (*d).flags |= 4 as i32;
    } else if type_0 == argtype::D_subscript as i32 {
        (*d).symbol = symbol;
        (*d).flags |= 2 as i32;
    } else {
        (*d).symbol = symbol;
    }
    (*d).next = (*list).next;
    (*d).prev = list;
    (*list).next = d;
    (*(*d).next).prev = d;
    return d;
}
unsafe extern "C" fn do_add_item(
    mut list: *mut list_item,
    mut arg: *mut CMDARG,
) -> *mut list_item {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut pname: *mut i8 = 0 as *mut i8;
    let mut item: *mut list_item = 0 as *mut list_item;
    match (*arg).type_0 as u32 {
        49 | 45 => {
            name = (*arg).value.sval;
            symbol = find_symbol(name, &mut pname);
            if symbol.is_null() {
                return 0 as *mut list_item;
            }
            if (*symbol).type_0 as u32 == nodevals::Node_func as i32 as u32 {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"`%s' is a function\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
                return 0 as *mut list_item;
            }
            if (*arg).type_0 as u32 == argtype::D_subscript as i32 as u32
                && (*symbol).type_0 as u32 != nodevals::Node_var_array as i32 as u32
            {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"`%s' is not an array\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
                return 0 as *mut list_item;
            }
            item = add_item(list, (*arg).type_0 as i32, symbol, pname);
            if !item.is_null()
                && (*arg).type_0 as u32 == argtype::D_subscript as i32 as u32
            {
                let mut subs: *mut *mut NODE = 0 as *mut *mut NODE;
                let mut count: i32 = (*arg).a_count;
                let mut i: i32 = 0;
                subs = emalloc_real(
                    (count as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                    b"do_add_item\0" as *const u8 as *const i8,
                    b"subs\0" as *const u8 as *const i8,
                    b"debug.c\0" as *const u8 as *const i8,
                    1466 as i32,
                ) as *mut *mut NODE;
                i = 0 as i32;
                while i < count {
                    arg = (*arg).next;
                    let ref mut fresh8 = *subs.offset(i as isize);
                    *fresh8 = dupnode((*arg).value.nodeval);
                    let ref mut fresh9 = *subs.offset(i as isize);
                    *fresh9 = force_string_fmt(
                        *subs.offset(i as isize),
                        CONVFMT,
                        CONVFMTidx,
                    );
                    i += 1;
                    i;
                }
                (*item).subs = subs;
                (*item).num_subs = count;
            }
        }
        47 => {
            symbol = dupnode((*arg).value.nodeval);
            item = add_item(list, argtype::D_field as i32, symbol, 0 as *mut i8);
        }
        _ => {}
    }
    if list == &mut watch_list as *mut list_item {
        arg = (*arg).next;
        if !item.is_null() && !arg.is_null() {
            if parse_condition(
                argtype::D_watch as i32,
                (*item).number,
                (*arg).value.sval,
            ) == 0 as i32
            {
                (*arg).value.sval = 0 as *mut i8;
            } else {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"watchpoint %d is unconditional\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*item).number,
                );
            }
        }
    }
    return item;
}
unsafe extern "C" fn do_delete_item(mut list: *mut list_item, mut arg: *mut CMDARG) {
    if arg.is_null() {
        while (*list).next != list {
            delete_item((*list).next);
        }
    }
    while !arg.is_null() {
        let mut d: *mut list_item = 0 as *mut list_item;
        if (*arg).type_0 as u32 == argtype::D_range as i32 as u32 {
            let mut i: i64 = 0;
            let mut j: i64 = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > (*list).number as i64 {
                j = (*list).number as i64;
            }
            while i <= j {
                d = find_item(list, i);
                if !d.is_null() {
                    delete_item(d);
                }
                i += 1;
                i;
            }
        } else {
            d = find_item(list, (*arg).value.lval);
            if d.is_null() {
                if list == &mut display_list as *mut list_item {
                    d_error(
                        dcgettext(
                            0 as *const i8,
                            b"no display item numbered %ld\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*arg).value.lval,
                    );
                } else {
                    d_error(
                        dcgettext(
                            0 as *const i8,
                            b"no watch item numbered %ld\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*arg).value.lval,
                    );
                }
            } else {
                delete_item(d);
            }
        }
        arg = (*arg).next;
    }
}
unsafe extern "C" fn display(mut d: *mut list_item) {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    symbol = (*d).symbol;
    if (*d).flags & 1 as i32 != 0 as i32 && (*d).fcall_count != fcall_count - cur_frame {
        return;
    }
    let mut current_block_17: u64;
    if (*d).flags & 2 as i32 != 0 as i32 {
        let mut sub: *mut NODE = 0 as *mut NODE;
        let mut r: *mut NODE = 0 as *mut NODE;
        let mut i: i32 = 0 as i32;
        let mut count: i32 = (*d).num_subs;
        i = 0 as i32;
        loop {
            if !(i < count) {
                current_block_17 = 18317007320854588510;
                break;
            }
            sub = *((*d).subs).offset(i as isize);
            r = in_array(symbol, sub);
            if r.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"%d: subscript \"%.*s\" is not in array `%s'\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*d).number,
                    (*sub).sub.val.slen as i32,
                    (*sub).sub.val.sp,
                    (*d).sname,
                );
                current_block_17 = 18317007320854588510;
                break;
            } else {
                if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                    symbol = r;
                    if i == count - 1 as i32 {
                        current_block_17 = 4930609730660346180;
                        break;
                    }
                } else {
                    if i != count - 1 as i32 {
                        return;
                    }
                    fprintf(
                        out_fp,
                        b"%d: %s[\"%.*s\"] = \0" as *const u8 as *const i8,
                        (*d).number,
                        (*d).sname,
                        (*sub).sub.val.slen as i32,
                        (*sub).sub.val.sp,
                    );
                    valinfo(
                        r,
                        Some(
                            fprintf
                                as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                        ),
                        out_fp,
                    );
                }
                i += 1;
                i;
            }
        }
    } else if (*d).flags & 4 as i32 != 0 as i32 {
        let mut r_0: *mut NODE = (*d).symbol;
        fprintf(out_fp, b"%d: \0" as *const u8 as *const i8, (*d).number);
        print_field((*r_0).sub.val.fltnum as i64);
        current_block_17 = 18317007320854588510;
    } else {
        current_block_17 = 4930609730660346180;
    }
    match current_block_17 {
        4930609730660346180 => {
            fprintf(
                out_fp,
                b"%d: %s = \0" as *const u8 as *const i8,
                (*d).number,
                (*d).sname,
            );
            print_symbol(symbol, (*d).flags & 1 as i32 != 0 as i32);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_display(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut d: *mut list_item = 0 as *mut list_item;
    if arg.is_null() {
        d = display_list.prev;
        while d != &mut display_list as *mut list_item {
            display(d);
            d = (*d).prev;
        }
        return 0 as i32;
    }
    d = do_add_item(&mut display_list, arg);
    if !d.is_null() {
        display(d);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_undisplay(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    do_delete_item(&mut display_list, arg);
    return 0 as i32;
}
unsafe extern "C" fn condition_triggered(mut cndn: *mut condition) -> i32 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut di: i32 = 0;
    if ((*cndn).code).is_null() {
        return 1 as i32;
    }
    push_context((*cndn).ctxt);
    r = execute_code((*cndn).code as *mut INSTRUCTION);
    pop_context();
    if r.is_null() {
        return 0 as i32;
    }
    force_number(r);
    di = !((*r).sub.val.fltnum == 0.0f64) as i32;
    DEREF(r);
    return di;
}
unsafe extern "C" fn find_subscript(
    mut item: *mut list_item,
    mut ptr: *mut *mut NODE,
) -> i32 {
    let mut symbol: *mut NODE = (*item).symbol;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0 as i32;
    let mut count: i32 = (*item).num_subs;
    *ptr = 0 as *mut NODE;
    r = *ptr;
    i = 0 as i32;
    while i < count {
        sub = *((*item).subs).offset(i as isize);
        r = in_array(symbol, sub);
        if r.is_null() {
            return 0 as i32;
        }
        if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            symbol = r;
        } else if i < count - 1 as i32 {
            return -(1 as i32)
        }
        i += 1;
        i;
    }
    if !r.is_null() {
        *ptr = r;
    }
    return 0 as i32;
}
unsafe extern "C" fn cmp_val(
    mut w: *mut list_item,
    mut old: *mut NODE,
    mut new: *mut NODE,
) -> i32 {
    if (*w).flags & 16 as i32 != 0 as i32 {
        let mut size: i64 = 0 as i32 as i64;
        if new.is_null() {
            return 1 as i32;
        }
        if (*new).type_0 as u32 == nodevals::Node_val as i32 as u32 {
            return 1 as i32;
        }
        size = (*new).sub.nodep.reflags as i64;
        if (*w).value[0 as i32 as usize].l == size {
            return 0 as i32;
        }
        return 1 as i32;
    }
    if old.is_null() && new.is_null() {
        return 0 as i32;
    }
    if old.is_null() && !new.is_null() || !old.is_null() && new.is_null() {
        return 1 as i32;
    }
    if (*new).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        return 1 as i32;
    }
    return cmp_nodes(old, new, 1 as i32 != 0);
}
unsafe extern "C" fn watchpoint_triggered(mut w: *mut list_item) -> i32 {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    symbol = (*w).symbol;
    if (*w).flags & 1 as i32 != 0 as i32 && (*w).fcall_count != fcall_count - cur_frame {
        return 0 as i32;
    }
    if condition_triggered(&mut (*w).cndn) == 0 {
        return 0 as i32;
    }
    t1 = (*w).value[0 as i32 as usize].n;
    t2 = 0 as *mut NODE;
    if (*w).flags & 2 as i32 != 0 as i32 {
        find_subscript(w, &mut t2);
    } else if (*w).flags & 4 as i32 != 0 as i32 {
        let mut field_num: i64 = 0;
        field_num = (*(*w).symbol).sub.val.fltnum as i64;
        t2 = *get_field(field_num, 0 as *mut Func_ptr);
    } else {
        match (*symbol).type_0 as u32 {
            4 => {
                t2 = (*symbol).sub.nodep.l.lptr;
            }
            5 => {
                t2 = symbol;
            }
            6 | 7 => {}
            _ => {
                r_fatal(
                    b"internal error: file %s, line %d: unexpected symbol type %s\0"
                        as *const u8 as *const i8,
                    b"debug.c\0" as *const u8 as *const i8,
                    1740 as i32,
                    nodetype2str((*symbol).type_0),
                );
            }
        }
    }
    if cmp_val(w, t1, t2) == 0 {
        return 0 as i32;
    }
    if (*w).flags & 8 as i32 == 0 as i32 {
        unref((*w).value[1 as i32 as usize].n);
    }
    (*w).flags &= !(8 as i32);
    if (*w).flags & 16 as i32 != 0 as i32 {
        (*w).value[1 as i32 as usize].l = (*w).value[0 as i32 as usize].l;
        (*w).flags |= 8 as i32;
        if t2.is_null() {
            (*w).flags &= !(16 as i32);
            (*w).value[0 as i32 as usize].n = 0 as *mut NODE;
        } else if (*t2).type_0 as u32 == nodevals::Node_val as i32 as u32 {
            (*w).flags &= !(16 as i32);
            (*w).value[0 as i32 as usize].n = dupnode(t2);
        } else {
            (*w).value[0 as i32 as usize].l = (if (*t2).type_0 as u32
                == nodevals::Node_var_array as i32 as u32
            {
                (*t2).sub.nodep.reflags as u32
            } else {
                0 as i32 as u32
            }) as i64;
        }
    } else if t1.is_null() {
        (*w).value[1 as i32 as usize].n = 0 as *mut NODE;
        if (*t2).type_0 as u32 == nodevals::Node_val as i32 as u32 {
            (*w).value[0 as i32 as usize].n = dupnode(t2);
        } else {
            (*w).flags |= 16 as i32;
            (*w).value[0 as i32 as usize].l = (if (*t2).type_0 as u32
                == nodevals::Node_var_array as i32 as u32
            {
                (*t2).sub.nodep.reflags as u32
            } else {
                0 as i32 as u32
            }) as i64;
        }
    } else {
        (*w).value[1 as i32 as usize].n = (*w).value[0 as i32 as usize].n;
        if t2.is_null() {
            (*w).value[0 as i32 as usize].n = 0 as *mut NODE;
        } else if (*t2).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            (*w).flags |= 16 as i32;
            (*w).value[0 as i32 as usize].l = (*t2).sub.nodep.reflags as i64;
        } else {
            (*w).value[0 as i32 as usize].n = dupnode(t2);
        }
    }
    return (*w).number;
}
unsafe extern "C" fn initialize_watch_item(mut w: *mut list_item) -> i32 {
    let mut t: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut symbol: *mut NODE = (*w).symbol;
    if (*w).flags & 2 as i32 != 0 as i32 {
        if find_subscript(w, &mut r) == -(1 as i32) {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"attempt to use scalar value as array\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return -(1 as i32);
        }
        if r.is_null() {
            (*w).value[0 as i32 as usize].n = 0 as *mut NODE;
        } else if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            (*w).flags |= 16 as i32;
            (*w).value[0 as i32 as usize].l = (*r).sub.nodep.reflags as i64;
        } else {
            (*w).value[0 as i32 as usize].n = dupnode(r);
        }
    } else if (*w).flags & 4 as i32 != 0 as i32 {
        let mut field_num: i64 = 0;
        t = (*w).symbol;
        field_num = (*t).sub.val.fltnum as i64;
        r = *get_field(field_num, 0 as *mut Func_ptr);
        (*w).value[0 as i32 as usize].n = dupnode(r);
    } else if (*symbol).type_0 as u32 == nodevals::Node_var_new as i32 as u32
        || (*symbol).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
    {
        (*w).value[0 as i32 as usize].n = 0 as *mut NODE;
    } else if (*symbol).type_0 as u32 == nodevals::Node_var as i32 as u32 {
        r = (*symbol).sub.nodep.l.lptr;
        (*w).value[0 as i32 as usize].n = dupnode(r);
    } else if (*symbol).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        (*w).flags |= 16 as i32;
        (*w).value[0 as i32 as usize].l = (*symbol).sub.nodep.reflags as i64;
    } else if (*symbol).type_0 as u32 == nodevals::Node_val as i32 as u32
        && (*symbol).flags as u32 & flagvals::REGEX as i32 as u32 != 0 as i32 as u32
    {
        (*w).value[0 as i32 as usize].n = dupnode(symbol);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_watch(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    w = do_add_item(&mut watch_list, arg);
    if w.is_null() {
        return 0 as i32;
    }
    if initialize_watch_item(w) == -(1 as i32) {
        delete_item(w);
        return 0 as i32;
    }
    fprintf(out_fp, b"Watchpoint %d: \0" as *const u8 as *const i8, (*w).number);
    symbol = (*w).symbol;
    if (*w).flags & 2 as i32 != 0 as i32 {
        fprintf(out_fp, b"%s\0" as *const u8 as *const i8, (*w).sname);
        i = 0 as i32;
        while i < (*w).num_subs {
            sub = *((*w).subs).offset(i as isize);
            fprintf(
                out_fp,
                b"[\"%.*s\"]\0" as *const u8 as *const i8,
                (*sub).sub.val.slen as i32,
                (*sub).sub.val.sp,
            );
            i += 1;
            i;
        }
        fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
    } else if (*w).flags & 4 as i32 != 0 as i32 {
        fprintf(
            out_fp,
            b"$%ld\n\0" as *const u8 as *const i8,
            (*symbol).sub.val.fltnum as i64,
        );
    } else {
        fprintf(out_fp, b"%s\n\0" as *const u8 as *const i8, (*w).sname);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_unwatch(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    do_delete_item(&mut watch_list, arg);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn frame_popped() {
    let mut item: *mut list_item = 0 as *mut list_item;
    item = watch_list.next;
    while item != &mut watch_list as *mut list_item {
        if (*item).flags & 1 as i32 != 0 as i32 && (*item).fcall_count > fcall_count {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Watchpoint %d deleted because parameter is out of scope.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*item).number,
            );
            item = (*item).prev;
            delete_item((*item).next);
        }
        item = (*item).next;
    }
    item = display_list.next;
    while item != &mut display_list as *mut list_item {
        if (*item).flags & 1 as i32 != 0 as i32 && (*item).fcall_count > fcall_count {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Display %d deleted because parameter is out of scope.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*item).number,
            );
            item = (*item).prev;
            delete_item((*item).next);
        }
        item = (*item).next;
    }
}
unsafe extern "C" fn print_function(
    mut pc: *mut INSTRUCTION,
    mut x: *mut libc::c_void,
) -> i32 {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    let mut pcount: i32 = 0;
    let mut data: *mut pf_data = x as *mut pf_data;
    let mut defn: i32 = (*data).defn as i32;
    let mut print_func: Func_print = (*data).print_func;
    let mut fp: *mut FILE = (*data).fp;
    func = (*pc).x.xn;
    pcount = (*func).sub.nodep.l.ll as i32;
    print_func
        .expect(
            "non-null function pointer",
        )(fp, b"%s(\0" as *const u8 as *const i8, (*func).sub.nodep.name);
    i = 0 as i32;
    while i < pcount {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp,
            b"%s\0" as *const u8 as *const i8,
            (*((*func).sub.nodep.rn).offset(i as isize)).sub.nodep.name,
        );
        if i < pcount - 1 as i32 {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b", \0" as *const u8 as *const i8);
        }
        i += 1;
        i;
    }
    print_func.expect("non-null function pointer")(fp, b")\0" as *const u8 as *const i8);
    if defn != 0 {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp,
            dcgettext(
                0 as *const i8,
                b" in file `%s', line %d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*pc).d.name,
            (*pc).source_line as i32,
        );
    }
    return 0 as i32;
}
unsafe extern "C" fn print_frame(
    mut func: *mut NODE,
    mut src: *mut i8,
    mut srcline: i32,
) {
    if func.is_null() {
        fprintf(out_fp, b"main()\0" as *const u8 as *const i8);
    } else {
        pf_data.print_func = Some(
            fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
        );
        pf_data.fp = out_fp;
        pf_data.defn = 0 as i32 != 0;
        print_function(
            (*func).sub.nodep.r.iptr,
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
    }
    fprintf(
        out_fp,
        dcgettext(0 as *const i8, b" at `%s':%d\0" as *const u8 as *const i8, 5 as i32),
        src,
        srcline,
    );
}
unsafe extern "C" fn print_numbered_frame(mut num: i64) {
    let mut f: *mut NODE = 0 as *mut NODE;
    f = find_frame(num);
    if num == 0 as i32 as i64 {
        fprintf(out_fp, b"#%ld\t \0" as *const u8 as *const i8, num);
        print_frame((*f).sub.nodep.x.extra, source, sourceline);
    } else {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"#%ld\tin \0" as *const u8 as *const i8,
                5 as i32,
            ),
            num,
        );
        print_frame(
            (*f).sub.nodep.x.extra,
            (*f).sub.nodep.name,
            (*((*find_frame(num - 1 as i32 as i64)).sub.nodep.l.li as *mut INSTRUCTION))
                .source_line as i32,
        );
    }
    fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn do_backtrace(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut cur: i64 = 0 as i32 as i64;
    let mut last: i64 = fcall_count;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
        let mut count: i64 = (*arg).value.lval;
        if count >= 0 as i32 as i64 {
            last = count - 1 as i32 as i64;
            if last > fcall_count {
                last = fcall_count;
            }
        } else {
            cur = 1 as i32 as i64 + fcall_count + count;
            if cur < 0 as i32 as i64 {
                cur = 0 as i32 as i64;
            }
        }
    }
    while cur <= last {
        print_numbered_frame(cur);
        cur += 1;
        cur;
    }
    if cur <= fcall_count {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"More stack frames follow ...\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    return 0 as i32;
}
unsafe extern "C" fn print_cur_frame_and_sourceline() {
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut srcline: i32 = 0;
    let mut src: *mut i8 = 0 as *mut i8;
    f = find_frame(cur_frame);
    if cur_frame == 0 as i32 as i64 {
        src = source;
        srcline = sourceline;
    } else {
        f = find_frame(cur_frame);
        src = (*f).sub.nodep.name;
        srcline = (*((*find_frame(cur_frame - 1 as i32 as i64)).sub.nodep.l.li
            as *mut INSTRUCTION))
            .source_line as i32;
    }
    fprintf(
        out_fp,
        if cur_frame > 0 as i32 as i64 {
            dcgettext(0 as *const i8, b"#%ld\tin \0" as *const u8 as *const i8, 5 as i32)
        } else {
            b"#%ld\t \0" as *const u8 as *const i8
        },
        cur_frame,
    );
    print_frame((*f).sub.nodep.x.extra, src, srcline);
    fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
    print_lines(src, srcline, 1 as i32);
    last_printed_line = srcline - list_size / 2 as i32;
    if last_printed_line < 0 as i32 {
        last_printed_line = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_frame(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
        if (*arg).value.lval < 0 as i32 as i64 || (*arg).value.lval > fcall_count {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"invalid frame number\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return 0 as i32;
        }
        cur_frame = (*arg).value.lval;
    }
    print_cur_frame_and_sourceline();
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_up(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
        cur_frame += (*arg).value.lval;
    } else {
        cur_frame += 1;
        cur_frame;
    }
    if cur_frame < 0 as i32 as i64 {
        cur_frame = 0 as i32 as i64;
    } else if cur_frame > fcall_count {
        cur_frame = fcall_count;
    }
    print_cur_frame_and_sourceline();
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_down(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
        cur_frame -= (*arg).value.lval;
    } else {
        cur_frame -= 1;
        cur_frame;
    }
    if cur_frame < 0 as i32 as i64 {
        cur_frame = 0 as i32 as i64;
    } else if cur_frame > fcall_count {
        cur_frame = fcall_count;
    }
    print_cur_frame_and_sourceline();
    return 0 as i32;
}
unsafe extern "C" fn find_rule(mut src: *mut i8, mut lineno: i64) -> *mut INSTRUCTION {
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if lineno == 0 as i32 as i64 {
        rp = (*rule_list).nexti;
        while !rp.is_null() {
            if (*rp.offset(-(1 as i32 as isize))).d.name == src
                && (*rp.offset(-(1 as i32 as isize))).source_line as i32 > 0 as i32
            {
                return rp.offset(-(1 as i32 as isize));
            }
            rp = (*rp).nexti;
        }
    } else {
        rp = (*rule_list).nexti;
        while !rp.is_null() {
            if (*rp.offset(-(1 as i32 as isize))).d.name == src
                && lineno >= (*rp.offset(1 as i32 as isize)).source_line as i64
                && lineno <= (*rp.offset(1 as i32 as isize)).x.xl
            {
                return rp.offset(-(1 as i32 as isize));
            }
            rp = (*rp).nexti;
        }
    }
    return 0 as *mut INSTRUCTION;
}
unsafe extern "C" fn mk_breakpoint(
    mut src: *mut i8,
    mut srcline: i32,
) -> *mut INSTRUCTION {
    let mut bp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    bp = bcalloc(opcodeval::Op_breakpoint, 1 as i32, srcline);
    b = emalloc_real(
        ::core::mem::size_of::<BREAKPOINT>() as u64,
        b"mk_breakpoint\0" as *const u8 as *const i8,
        b"b\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        2134 as i32,
    ) as *mut BREAKPOINT;
    memset(
        &mut (*b).cndn as *mut condition as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<condition>() as u64,
    );
    (*b).commands.prev = &mut (*b).commands;
    (*b).commands.next = (*b).commands.prev;
    (*b).silent = 0 as i32 != 0;
    watch_list.number += 1;
    (*b).number = watch_list.number;
    (*b).ignore_count = 0 as i32 as i64;
    (*b).hit_count = 0 as i32 as i64;
    (*b).flags = 1 as i32 as libc::c_short;
    (*b).src = src;
    (*bp).x.bpt = b;
    (*b).bpi = bp;
    (*b).next = breakpoints.next;
    (*b).prev = &mut breakpoints;
    breakpoints.next = b;
    (*(*b).next).prev = b;
    return bp;
}
unsafe extern "C" fn delete_breakpoint(mut b: *mut BREAKPOINT) {
    let mut pc: *mut INSTRUCTION = (*b).bpi;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    (*pc).opcode = opcodeval::Op_no_op;
    (*pc).source_line = 0 as i32 as libc::c_short;
    (*pc).x.bpt = 0 as *mut break_point;
    c = (*b).commands.next;
    while c != &mut (*b).commands as *mut commands_item {
        c = (*c).prev;
        delete_commands_item((*c).next);
        c = (*c).next;
    }
    free_context((*b).cndn.ctxt, 0 as i32 != 0);
    if !((*b).cndn.expr).is_null() {
        pma_free((*b).cndn.expr as *mut libc::c_void);
    }
    (*(*b).next).prev = (*b).prev;
    (*(*b).prev).next = (*b).next;
    pma_free(b as *mut libc::c_void);
}
unsafe extern "C" fn find_breakpoint(mut num: i64) -> *mut BREAKPOINT {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if num <= 0 as i32 as i64 {
        return 0 as *mut BREAKPOINT;
    }
    b = breakpoints.next;
    while b != &mut breakpoints as *mut BREAKPOINT {
        if (*b).number as i64 == num {
            return b;
        }
        b = (*b).next;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn add_breakpoint(
    mut prevp: *mut INSTRUCTION,
    mut ip: *mut INSTRUCTION,
    mut src: *mut i8,
    mut silent: bool,
) -> *mut BREAKPOINT {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut bp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut lineno: i32 = (*ip).source_line as i32;
    while (*ip).opcode as u32 == opcodeval::Op_breakpoint as i32 as u32
        && (*ip).source_line as i32 == lineno
    {
        if !silent {
            b = (*ip).x.bpt;
            if (*b).flags as i32 & 1 as i32 != 0 as i32 {
                if (*b).flags as i32 & 8 as i32 != 0 as i32 {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"Note: breakpoint %d (enabled, ignore next %ld hits), also set at %s:%d\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*b).number,
                        (*b).ignore_count,
                        (*b).src,
                        lineno,
                    );
                } else {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"Note: breakpoint %d (enabled), also set at %s:%d\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*b).number,
                        (*b).src,
                        lineno,
                    );
                }
            } else if (*b).flags as i32 & 8 as i32 != 0 as i32 {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Note: breakpoint %d (disabled, ignore next %ld hits), also set at %s:%d\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*b).number,
                    (*b).ignore_count,
                    (*b).src,
                    lineno,
                );
            } else {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Note: breakpoint %d (disabled), also set at %s:%d\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*b).number,
                    (*b).src,
                    lineno,
                );
            }
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    bp = mk_breakpoint(src, lineno);
    (*prevp).nexti = bp;
    (*bp).nexti = ip;
    b = (*bp).x.bpt;
    if !silent {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"Breakpoint %d set at file `%s', line %d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            (*b).number,
            src,
            lineno,
        );
    }
    return b;
}
unsafe extern "C" fn set_breakpoint_at(
    mut rp: *mut INSTRUCTION,
    mut lineno: i32,
    mut silent: bool,
) -> *mut BREAKPOINT {
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut prevp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    prevp = rp;
    ip = (*rp).nexti;
    while !ip.is_null() {
        if (*ip).opcode as u32 == opcodeval::Op_K_case as i32 as u32 {
            let mut i1: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            let mut i2: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
            i2 = (*ip).d.di;
            i1 = (*i2).nexti;
            while i2 != (*ip).x.xi {
                if (*i1).source_line as i32 >= lineno {
                    return add_breakpoint(i2, i1, (*rp).d.name, silent);
                }
                if i1 == (*ip).x.xi {
                    break;
                }
                i2 = i1;
                i1 = (*i1).nexti;
            }
        }
        if (*ip).source_line as i32 >= lineno {
            return add_breakpoint(prevp, ip, (*rp).d.name, silent);
        }
        if ip == (*rp.offset(1 as i32 as isize)).d.di {
            break;
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn set_breakpoint_next(
    mut rp: *mut INSTRUCTION,
    mut ip: *mut INSTRUCTION,
) -> *mut BREAKPOINT {
    let mut prevp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    if ip == (*rp.offset(1 as i32 as isize)).d.di {
        return 0 as *mut BREAKPOINT;
    }
    prevp = ip;
    if (*ip).opcode as u32 != opcodeval::Op_breakpoint as i32 as u32 {
        ip = (*ip).nexti;
    }
    while !ip.is_null() {
        if (*ip).source_line as i32 > 0 as i32 {
            return add_breakpoint(prevp, ip, (*rp).d.name, 0 as i32 != 0);
        }
        if ip == (*rp.offset(1 as i32 as isize)).d.di {
            break;
        }
        prevp = ip;
        ip = (*ip).nexti;
    }
    return 0 as *mut BREAKPOINT;
}
unsafe extern "C" fn set_breakpoint(mut arg: *mut CMDARG, mut temporary: bool) -> i32 {
    let mut lineno: i32 = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut i8 = (*cur_srcfile).src;
    if arg.is_null() {
        if !prog_running {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"program not running\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return 0 as i32;
        }
        if cur_frame == 0 as i32 as i64 {
            src = source;
            ip = cur_pc;
        } else {
            let mut f: *mut NODE = 0 as *mut NODE;
            f = find_frame(cur_frame);
            src = (*f).sub.nodep.name;
            ip = (*find_frame(cur_frame - 1 as i32 as i64)).sub.nodep.l.li
                as *mut INSTRUCTION;
        }
        rp = find_rule(src, (*ip).source_line as i64);
        b = set_breakpoint_next(rp, ip);
        if b.is_null() {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"cannot set breakpoint in file `%s'\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                src,
            );
        } else {
            if cur_frame == 0 as i32 as i64 {
                (*b).flags = ((*b).flags as i32 | 8 as i32) as libc::c_short;
                (*b).ignore_count = 1 as i32 as i64;
            }
            if temporary {
                (*b).flags = ((*b).flags as i32 | 4 as i32) as libc::c_short;
            }
        }
        return 0 as i32;
    }
    let mut current_block_52: u64;
    match (*arg).type_0 as u32 {
        44 => {
            s = source_find((*arg).value.sval);
            arg = (*arg).next;
            if s.is_null() || arg.is_null()
                || (*arg).type_0 as u32 != argtype::D_int as i32 as u32
                    && (*arg).type_0 as u32 != argtype::D_func as i32 as u32
            {
                return 0 as i32;
            }
            src = (*s).src;
            if (*arg).type_0 as u32 == argtype::D_func as i32 as u32 {
                current_block_52 = 3224757918216980676;
            } else {
                current_block_52 = 15414074289446687308;
            }
        }
        43 => {
            current_block_52 = 15414074289446687308;
        }
        50 => {
            current_block_52 = 3224757918216980676;
        }
        _ => return 0 as i32,
    }
    match current_block_52 {
        15414074289446687308 => {
            lineno = (*arg).value.lval as i32;
            if lineno <= 0 as i32 || lineno > (*s).srclines {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"line number %d in file `%s' is out of range\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    lineno,
                    src,
                );
            } else {
                rp = find_rule(src, lineno as i64);
                if rp.is_null() {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"internal error: cannot find rule\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                if rp.is_null()
                    || {
                        b = set_breakpoint_at(rp, lineno, 0 as i32 != 0);
                        b.is_null()
                    }
                {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"cannot set breakpoint at `%s':%d\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        src,
                        lineno,
                    );
                }
                if !b.is_null() && temporary as i32 != 0 {
                    (*b).flags = ((*b).flags as i32 | 4 as i32) as libc::c_short;
                }
            }
        }
        _ => {
            func = (*arg).value.nodeval;
            rp = (*func).sub.nodep.r.iptr;
            b = set_breakpoint_at(rp, (*rp).source_line as i32, 0 as i32 != 0);
            if b.is_null() {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"cannot set breakpoint in function `%s'\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*func).sub.nodep.name,
                );
            } else {
                if temporary {
                    (*b).flags = ((*b).flags as i32 | 4 as i32) as libc::c_short;
                }
                lineno = (*(*b).bpi).source_line as i32;
            }
        }
    }
    arg = (*arg).next;
    if !b.is_null() && !arg.is_null() {
        if parse_condition(argtype::D_break as i32, (*b).number, (*arg).value.sval)
            == 0 as i32
        {
            (*arg).value.sval = 0 as *mut i8;
        } else {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"breakpoint %d set at file `%s', line %d is unconditional\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*b).number,
                src,
                lineno,
            );
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn breakpoint_triggered(mut b: *mut BREAKPOINT) -> i32 {
    if (*b).flags as i32 & 1 as i32 == 0 as i32 {
        return 0 as i32;
    }
    if (*b).flags as i32 & 8 as i32 != 0 as i32 {
        (*b).ignore_count -= 1;
        if (*b).ignore_count <= 0 as i32 as i64 {
            (*b).flags = ((*b).flags as i32 & !(8 as i32)) as libc::c_short;
        }
        return 0 as i32;
    }
    if condition_triggered(&mut (*b).cndn) == 0 {
        return 0 as i32;
    }
    (*b).hit_count += 1;
    (*b).hit_count;
    if (*b).flags as i32 & 2 as i32 != 0 as i32 {
        (*b).flags = ((*b).flags as i32 & !(2 as i32)) as libc::c_short;
        (*b).flags = ((*b).flags as i32 & !(1 as i32)) as libc::c_short;
    }
    return (*b).number;
}
#[no_mangle]
pub unsafe extern "C" fn do_breakpoint(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    return set_breakpoint(arg, 0 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn do_tmp_breakpoint(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    return set_breakpoint(arg, 1 as i32 != 0);
}
#[no_mangle]
pub unsafe extern "C" fn do_clear(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut lineno: i32 = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut i8 = (*cur_srcfile).src;
    let mut bp_found: i32 = 0 as i32;
    if arg.is_null() {
        if !prog_running {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"program not running\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return 0 as i32;
        }
        if cur_frame == 0 as i32 as i64 {
            lineno = sourceline;
            src = source;
        } else {
            let mut f: *mut NODE = 0 as *mut NODE;
            f = find_frame(cur_frame);
            src = (*f).sub.nodep.name;
            lineno = (*((*find_frame(cur_frame - 1 as i32 as i64)).sub.nodep.l.li
                as *mut INSTRUCTION))
                .source_line as i32;
        }
    } else {
        's_191: {
            let mut current_block_36: u64;
            match (*arg).type_0 as u32 {
                44 => {
                    s = source_find((*arg).value.sval);
                    arg = (*arg).next;
                    if s.is_null() || arg.is_null()
                        || (*arg).type_0 as u32 != argtype::D_int as i32 as u32
                            && (*arg).type_0 as u32 != argtype::D_func as i32 as u32
                    {
                        return 0 as i32;
                    }
                    src = (*s).src;
                    if (*arg).type_0 as u32 == argtype::D_func as i32 as u32 {
                        current_block_36 = 882999933025113155;
                    } else {
                        current_block_36 = 9786085573804384142;
                    }
                }
                43 => {
                    current_block_36 = 9786085573804384142;
                }
                50 => {
                    current_block_36 = 882999933025113155;
                }
                _ => {
                    current_block_36 = 3832808693628565118;
                }
            }
            match current_block_36 {
                882999933025113155 => {
                    func = (*arg).value.nodeval;
                    rp = (*func).sub.nodep.r.iptr;
                    ip = (*rp).nexti;
                    while !ip.is_null() {
                        if !((*ip).source_line as i32 <= 0 as i32) {
                            if (*ip).opcode as u32
                                != opcodeval::Op_breakpoint as i32 as u32
                            {
                                break;
                            }
                            b = (*ip).x.bpt;
                            bp_found += 1;
                            if bp_found == 1 as i32 {
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Deleted breakpoint %d\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    (*b).number,
                                );
                            } else {
                                fprintf(
                                    out_fp,
                                    b", %d\0" as *const u8 as *const i8,
                                    (*b).number,
                                );
                            }
                            delete_breakpoint(b);
                        }
                        ip = (*ip).nexti;
                    }
                    if bp_found == 0 as i32 {
                        fprintf(
                            out_fp,
                            dcgettext(
                                0 as *const i8,
                                b"No breakpoint(s) at entry to function `%s'\n\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*func).sub.nodep.name,
                        );
                    } else {
                        fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
                    }
                }
                9786085573804384142 => {
                    lineno = (*arg).value.lval as i32;
                    if lineno <= 0 as i32 || lineno > (*s).srclines {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"line number %d in file `%s' out of range\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            lineno,
                            src,
                        );
                        return 0 as i32;
                    }
                    break 's_191;
                }
                _ => {}
            }
            return 0 as i32;
        }
    }
    rp = find_rule(src, lineno as i64);
    if !rp.is_null() {
        ip = (*rp).nexti;
        while !ip.is_null() {
            if (*ip).opcode as u32 == opcodeval::Op_breakpoint as i32 as u32
                && (*ip).source_line as i32 == lineno
            {
                b = (*ip).x.bpt;
                bp_found += 1;
                if bp_found == 1 as i32 {
                    fprintf(
                        out_fp,
                        dcgettext(
                            0 as *const i8,
                            b"Deleted breakpoint %d\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*b).number,
                    );
                } else {
                    fprintf(out_fp, b", %d\0" as *const u8 as *const i8, (*b).number);
                }
                delete_breakpoint(b);
            }
            if ip == (*rp.offset(1 as i32 as isize)).d.di {
                break;
            }
            ip = (*ip).nexti;
        }
    }
    if bp_found == 0 as i32 {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"No breakpoint at file `%s', line #%d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            src,
            lineno,
        );
    } else {
        fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn enable_breakpoint(mut b: *mut BREAKPOINT, mut disp: libc::c_short) {
    (*b).flags = ((*b).flags as i32 & !(2 as i32 | 4 as i32)) as libc::c_short;
    (*b).flags = ((*b).flags as i32 | 1 as i32) as libc::c_short;
    if disp != 0 {
        (*b).flags = ((*b).flags as i32 | disp as i32) as libc::c_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_enable_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut disp: libc::c_short = 0 as i32 as libc::c_short;
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_argument as i32 as u32 {
        if (*arg).value.lval == nametypeval::A_DEL as i32 as i64 {
            disp = 4 as i32 as libc::c_short;
        } else {
            disp = 2 as i32 as libc::c_short;
        }
        arg = (*arg).next;
    }
    if arg.is_null() {
        b = breakpoints.next;
        while b != &mut breakpoints as *mut BREAKPOINT {
            enable_breakpoint(b, disp);
            b = (*b).next;
        }
    }
    while !arg.is_null() {
        if (*arg).type_0 as u32 == argtype::D_range as i32 as u32 {
            let mut i: i64 = 0;
            let mut j: i64 = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as i64 {
                j = breakpoints.number as i64;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    enable_breakpoint(b, disp);
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"invalid breakpoint number\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                enable_breakpoint(b, disp);
            }
        }
        arg = (*arg).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_delete_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    if arg.is_null() {
        let mut delete_all: bool = 1 as i32 != 0;
        delete_all = prompt_yes_no(
            dcgettext(
                0 as *const i8,
                b"Delete all breakpoints? (y or n) \0" as *const u8 as *const i8,
                5 as i32,
            ),
            *(dcgettext(0 as *const i8, b"y\0" as *const u8 as *const i8, 5 as i32))
                .offset(0 as i32 as isize),
            1 as i32,
            out_fp,
        ) != 0;
        if delete_all {
            while breakpoints.next != &mut breakpoints as *mut BREAKPOINT {
                delete_breakpoint(breakpoints.next);
            }
        }
    }
    while !arg.is_null() {
        let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
        if (*arg).type_0 as u32 == argtype::D_range as i32 as u32 {
            let mut i: i64 = 0;
            let mut j: i64 = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as i64 {
                j = breakpoints.number as i64;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    delete_breakpoint(b);
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"invalid breakpoint number\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                delete_breakpoint(b);
            }
        }
        arg = (*arg).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_ignore_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if arg.is_null() || (*arg).type_0 as u32 != argtype::D_int as i32 as u32
        || ((*arg).next).is_null()
        || (*(*arg).next).type_0 as u32 != argtype::D_int as i32 as u32
    {
        return 0 as i32;
    }
    b = find_breakpoint((*arg).value.lval);
    if b.is_null() {
        d_error(
            dcgettext(
                0 as *const i8,
                b"invalid breakpoint number\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        (*b).ignore_count = (*(*arg).next).value.lval;
        if (*b).ignore_count > 0 as i32 as i64 {
            (*b).flags = ((*b).flags as i32 | 8 as i32) as libc::c_short;
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Will ignore next %ld crossing(s) of breakpoint %d.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*b).ignore_count,
                (*b).number,
            );
        } else {
            (*b).flags = ((*b).flags as i32 & !(8 as i32)) as libc::c_short;
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"Will stop next time breakpoint %d is reached.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                (*b).number,
            );
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_disable_breakpoint(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if arg.is_null() {
        b = breakpoints.next;
        while b != &mut breakpoints as *mut BREAKPOINT {
            (*b).flags = ((*b).flags as i32 & !(1 as i32)) as libc::c_short;
            b = (*b).next;
        }
    }
    while !arg.is_null() {
        if (*arg).type_0 as u32 == argtype::D_range as i32 as u32 {
            let mut i: i64 = 0;
            let mut j: i64 = 0;
            i = (*arg).value.lval;
            arg = (*arg).next;
            j = (*arg).value.lval;
            if j > breakpoints.number as i64 {
                j = breakpoints.number as i64;
            }
            while i <= j {
                b = find_breakpoint(i);
                if !b.is_null() {
                    (*b).flags = ((*b).flags as i32 & !(1 as i32)) as libc::c_short;
                }
                i += 1;
                i;
            }
        } else {
            b = find_breakpoint((*arg).value.lval);
            if b.is_null() {
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"invalid breakpoint number\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            } else {
                (*b).flags = ((*b).flags as i32 & !(1 as i32)) as libc::c_short;
            }
        }
        arg = (*arg).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn init_debug() {
    register_exec_hook(
        Some(debug_pre_execute as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32),
        Some(debug_post_execute as unsafe extern "C" fn(*mut INSTRUCTION) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn debug_prog(mut pc: *mut INSTRUCTION) -> i32 {
    let mut run: *mut i8 = 0 as *mut i8;
    input_fd = fileno(stdin);
    out_fp = stdout;
    if os_isatty(input_fd) != 0 {
        input_from_tty = 1 as i32 != 0;
    }
    input_fd == 0 as i32 && input_from_tty as i32 != 0;
    if read_a_line.is_none() {
        read_a_line = Some(g_readline as unsafe extern "C" fn(*const i8) -> *mut i8);
    }
    push_cmd_src(input_fd, input_from_tty, read_a_line, None, 0 as i32, 2 as i32);
    setbuf(out_fp, 0 as *mut libc::c_void as *mut i8);
    cur_srcfile = (*srcfiles).prev;
    while cur_srcfile != srcfiles {
        if (*cur_srcfile).stype as u32 == srctype::SRC_FILE as i32 as u32
            || (*cur_srcfile).stype as u32 == srctype::SRC_INC as i32 as u32
        {
            break;
        }
        cur_srcfile = (*cur_srcfile).prev;
    }
    if cur_srcfile == srcfiles {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"Can only debug programs provided with the `-f' option.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit(1 as i32);
    }
    dgawk_prompt = estrdup(
        b"gawk> \0" as *const u8 as *const i8,
        strlen(b"gawk> \0" as *const u8 as *const i8),
    );
    dbg_prompt = dgawk_prompt;
    memset(
        &mut stop as *mut C2RustUnnamed_11 as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<C2RustUnnamed_11>() as u64,
    );
    stop.command = argtype::D_illegal;
    run = getenv(b"DGAWK_RESTART\0" as *const u8 as *const i8);
    if !run.is_null() {
        unserialize_list(C2RustUnnamed_13::BREAK as i32);
        unserialize_list(C2RustUnnamed_13::WATCH as i32);
        unserialize_list(C2RustUnnamed_13::DISPLAY as i32);
        unserialize_list(C2RustUnnamed_13::HISTORY as i32);
        unserialize_list(C2RustUnnamed_13::OPTION as i32);
        unsetenv(b"DGAWK_RESTART\0" as *const u8 as *const i8);
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"Restarting ...\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if strcasecmp(run, b"true\0" as *const u8 as *const i8) == 0 as i32 {
            do_run(0 as *mut CMDARG, 0 as i32);
        }
    } else if !command_file.is_null() {
        let mut fd: i32 = 0;
        fd = open_readfd(command_file);
        if fd == -(1 as i32) {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"cannot open source file `%s' for reading: %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                command_file,
                strerror(*__errno_location()),
            );
            exit(1 as i32);
        }
        push_cmd_src(
            fd,
            0 as i32 != 0,
            Some(g_readline as unsafe extern "C" fn(*const i8) -> *mut i8),
            Some(close as unsafe extern "C" fn(i32) -> i32),
            0 as i32,
            1 as i32,
        );
        (*cmd_src).str_0 = estrdup(command_file, strlen(command_file));
    } else {
        let mut fd_0: i32 = 0;
        fd_0 = open_readfd(options_file);
        if fd_0 > -(1 as i32) {
            push_cmd_src(
                fd_0,
                0 as i32 != 0,
                Some(g_readline as unsafe extern "C" fn(*const i8) -> *mut i8),
                Some(close as unsafe extern "C" fn(i32) -> i32),
                0 as i32,
                0 as i32,
            );
        }
    }
    zzparse();
    return 0 as i32;
}
unsafe extern "C" fn check_watchpoint() -> i32 {
    let mut w: *mut list_item = 0 as *mut list_item;
    if stop.command as u32 == argtype::D_return as i32 as u32 {
        return 0 as i32;
    }
    w = watch_list.prev;
    while w != &mut watch_list as *mut list_item {
        let mut wnum: i32 = watchpoint_triggered(w);
        if wnum > 0 as i32 {
            stop.watch_point = wnum;
            stop.print_frame = 1 as i32 != 0;
            return 1 as i32;
        }
        w = (*w).prev;
    }
    return 0 as i32;
}
unsafe extern "C" fn check_breakpoint(mut pi: *mut *mut INSTRUCTION) -> i32 {
    let mut pc: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    pc = *pi;
    if stop.command as u32 == argtype::D_return as i32 as u32 {
        return 0 as i32;
    }
    if (*pc).opcode as u32 == opcodeval::Op_breakpoint as i32 as u32 {
        let mut bnum: i32 = 0;
        *pi = (*pc).nexti;
        bnum = breakpoint_triggered((*pc).x.bpt);
        if bnum > 0 as i32 {
            stop.break_point = bnum;
            stop.print_frame = 1 as i32 != 0;
            return 1 as i32;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn restart(mut run: bool) -> ! {
    serialize_list(C2RustUnnamed_13::BREAK as i32);
    serialize_list(C2RustUnnamed_13::WATCH as i32);
    serialize_list(C2RustUnnamed_13::DISPLAY as i32);
    serialize_list(C2RustUnnamed_13::HISTORY as i32);
    serialize_list(C2RustUnnamed_13::OPTION as i32);
    setenv(
        b"DGAWK_RESTART\0" as *const u8 as *const i8,
        if run as i32 != 0 {
            b"true\0" as *const u8 as *const i8
        } else {
            b"false\0" as *const u8 as *const i8
        },
        1 as i32,
    );
    close_all();
    execvp(*d_argv.offset(0 as i32 as isize), d_argv as *const *mut i8);
    fprintf(
        out_fp,
        dcgettext(
            0 as *const i8,
            b"Failed to restart debugger\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    exit(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn do_run(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if prog_running {
        if !input_from_tty {
            need_restart = 1 as i32 != 0;
        } else {
            need_restart = prompt_yes_no(
                dcgettext(
                    0 as *const i8,
                    b"Program already running. Restart from beginning (y/n)? \0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                *(dcgettext(0 as *const i8, b"y\0" as *const u8 as *const i8, 5 as i32))
                    .offset(0 as i32 as isize),
                0 as i32,
                out_fp,
            ) != 0;
            if !need_restart {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Program not restarted\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return 0 as i32;
            }
        }
    }
    if need_restart {
        if !command_file.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"error: cannot restart, operation not allowed\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            exit(1 as i32);
        }
        if (*cmd_src).cmd == argtype::D_source as i32 {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"error (%s): cannot restart, ignoring rest of the commands\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*cmd_src).str_0,
            );
            pop_cmd_src();
            return 0 as i32;
        }
        restart(1 as i32 != 0);
    }
    fprintf(
        out_fp,
        dcgettext(
            0 as *const i8,
            b"Starting program:\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    prog_running = 1 as i32 != 0;
    fatal_tag_valid = 1 as i32;
    if _setjmp(fatal_tag.as_mut_ptr()) == 0 as i32 {
        interpret.expect("non-null function pointer")(code_block);
    }
    fatal_tag_valid = 0 as i32;
    prog_running = 0 as i32 != 0;
    fprintf(
        out_fp,
        if !exiting && exit_val != 0 as i32 {
            dcgettext(
                0 as *const i8,
                b"Program exited abnormally with exit value: %d\n\0" as *const u8
                    as *const i8,
                5 as i32,
            )
        } else {
            dcgettext(
                0 as *const i8,
                b"Program exited normally with exit value: %d\n\0" as *const u8
                    as *const i8,
                5 as i32,
            )
        },
        exit_val,
    );
    need_restart = 1 as i32 != 0;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_quit(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut terminate: bool = 1 as i32 != 0;
    if prog_running {
        terminate = prompt_yes_no(
            dcgettext(
                0 as *const i8,
                b"The program is running. Exit anyway (y/n)? \0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            *(dcgettext(0 as *const i8, b"y\0" as *const u8 as *const i8, 5 as i32))
                .offset(0 as i32 as isize),
            1 as i32,
            out_fp,
        ) != 0;
    }
    if terminate {
        close_all();
        do_trace = 0 as i32;
        if do_save_options != 0 && input_from_tty as i32 != 0 {
            save_options(options_file);
        }
        exit(exit_val);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_continue(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if arg.is_null() || (*arg).type_0 as u32 != argtype::D_int as i32 as u32 {
        return 1 as i32;
    }
    if stop.break_point == 0 {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"Not stopped at any breakpoint; argument ignored.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        return 1 as i32;
    }
    b = find_breakpoint(stop.break_point as i64);
    if b.is_null() {
        d_error(
            dcgettext(
                0 as *const i8,
                b"invalid breakpoint number %d\0" as *const u8 as *const i8,
                5 as i32,
            ),
            stop.break_point,
        );
        return 0 as i32;
    }
    (*b).flags = ((*b).flags as i32 | 8 as i32) as libc::c_short;
    (*b).ignore_count = (*arg).value.lval;
    fprintf(
        out_fp,
        dcgettext(
            0 as *const i8,
            b"Will ignore next %ld crossings of breakpoint %d.\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        (*b).ignore_count,
        stop.break_point,
    );
    return 1 as i32;
}
unsafe extern "C" fn next_step(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_int as i32 as u32 {
        stop.repeat_count = (*arg).value.lval as i32;
    } else {
        stop.repeat_count = 1 as i32;
    }
    stop.command = argtype::from_libc_c_uint(cmd as u32);
    return 1 as i32;
}
unsafe extern "C" fn check_step(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count != stop.fcall_count {
        stop.fcall_count = fcall_count;
        stop.sourceline = sourceline;
        stop.source = source;
        stop.print_frame = 1 as i32 != 0;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as i32) as i32;
    }
    if source != stop.source {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as i32) as i32;
    }
    if sourceline != stop.sourceline {
        stop.sourceline = sourceline;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as i32) as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_step(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut ret: i32 = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.fcall_count = fcall_count;
        stop.source = source;
        stop.sourceline = sourceline;
        stop.check_func = Some(
            check_step as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
        );
    }
    return ret;
}
unsafe extern "C" fn check_stepi(mut pi: *mut *mut INSTRUCTION) -> i32 {
    stop.repeat_count -= 1;
    return (stop.repeat_count == 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_stepi(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut ret: i32 = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.check_func = Some(
            check_stepi as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
        );
    }
    return ret;
}
unsafe extern "C" fn check_next(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count < stop.fcall_count {
        stop.fcall_count = fcall_count;
        stop.sourceline = sourceline;
        stop.source = source;
        stop.print_frame = 1 as i32 != 0;
        stop.repeat_count -= 1;
        return (stop.repeat_count == 0 as i32) as i32;
    }
    if fcall_count == stop.fcall_count {
        if source != stop.source {
            stop.source = source;
            stop.sourceline = sourceline;
            stop.repeat_count -= 1;
            return (stop.repeat_count == 0 as i32) as i32;
        }
        if sourceline != stop.sourceline {
            stop.sourceline = sourceline;
            stop.repeat_count -= 1;
            return (stop.repeat_count == 0 as i32) as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_next(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut ret: i32 = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.fcall_count = fcall_count;
        stop.check_func = Some(
            check_next as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
        );
    }
    return ret;
}
unsafe extern "C" fn check_nexti(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count < stop.fcall_count {
        stop.print_frame = 1 as i32 != 0;
        stop.fcall_count = fcall_count;
    }
    return (fcall_count == stop.fcall_count
        && {
            stop.repeat_count -= 1;
            stop.repeat_count == 0 as i32
        }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_nexti(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut ret: i32 = 0;
    ret = next_step(arg, cmd);
    if ret != 0 {
        stop.fcall_count = fcall_count;
        stop.check_func = Some(
            check_nexti as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
        );
    }
    return ret;
}
unsafe extern "C" fn check_finish(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count == stop.fcall_count {
        stop.print_frame = 1 as i32 != 0;
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_finish(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    if cur_frame == fcall_count {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"'finish' not meaningful in the outermost frame main()\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    stop.fcall_count = fcall_count - cur_frame - 1 as i32 as i64;
    fprintf(
        out_fp,
        dcgettext(
            0 as *const i8,
            b"Run until return from \0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    print_numbered_frame(cur_frame);
    stop.check_func = Some(
        check_finish as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
    );
    stop.command = argtype::from_libc_c_uint(cmd as u32);
    stop.print_ret = 1 as i32 != 0;
    return 1 as i32;
}
unsafe extern "C" fn check_return(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count == stop.fcall_count {
        stop.print_frame = 1 as i32 != 0;
        return 1 as i32;
    }
    if fcall_count > stop.fcall_count {
        let mut func: *mut NODE = 0 as *mut NODE;
        func = (*find_frame(cur_frame)).sub.nodep.x.extra;
        *pi = (*((*func).sub.nodep.r.iptr).offset(1 as i32 as isize)).d.di;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_return(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut n: *mut NODE = 0 as *mut NODE;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    func = (*find_frame(cur_frame)).sub.nodep.x.extra;
    if func.is_null() {
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"'return' not meaningful in the outermost frame main()\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    stop.fcall_count = fcall_count - cur_frame - 1 as i32 as i64;
    stop.pc = (*((*func).sub.nodep.r.iptr).offset(1 as i32 as isize)).d.di;
    stop.command = argtype::from_libc_c_uint(cmd as u32);
    stop.check_func = Some(
        check_return as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
    );
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_node as i32 as u32 {
        n = dupnode((*arg).value.nodeval);
    } else {
        n = dupnode(Nnull_string);
    }
    let ref mut fresh10 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh10 = n;
    return 1 as i32;
}
unsafe extern "C" fn check_until(mut pi: *mut *mut INSTRUCTION) -> i32 {
    if fcall_count < stop.fcall_count {
        stop.print_frame = 1 as i32 != 0;
        return 1 as i32;
    } else if fcall_count == stop.fcall_count {
        if !(stop.pc).is_null() && *pi == stop.pc {
            return 1 as i32;
        }
        if stop.sourceline > 0 as i32 && source == stop.source
            && sourceline > stop.sourceline
        {
            return 1 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_until(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut s: *mut SRCFILE = cur_srcfile;
    let mut src: *mut i8 = (*cur_srcfile).src;
    let mut lineno: i32 = 0;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ip: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut func: *mut NODE = 0 as *mut NODE;
    if !prog_running {
        d_error(
            dcgettext(
                0 as *const i8,
                b"program not running\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    stop.pc = 0 as *mut INSTRUCTION;
    stop.sourceline = 0 as i32;
    if arg.is_null() {
        stop.source = source;
        stop.sourceline = sourceline;
        stop.fcall_count = fcall_count - cur_frame;
        stop.check_func = Some(
            check_until as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
        );
        stop.command = argtype::from_libc_c_uint(cmd as u32);
        return 1 as i32;
    }
    's_166: {
        let mut current_block_39: u64;
        match (*arg).type_0 as u32 {
            44 => {
                s = source_find((*arg).value.sval);
                arg = (*arg).next;
                if s.is_null() || arg.is_null()
                    || (*arg).type_0 as u32 != argtype::D_int as i32 as u32
                        && (*arg).type_0 as u32 != argtype::D_func as i32 as u32
                {
                    return 0 as i32;
                }
                src = (*s).src;
                if (*arg).type_0 as u32 == argtype::D_func as i32 as u32 {
                    current_block_39 = 14434307700559646593;
                } else {
                    current_block_39 = 16476378062894899572;
                }
            }
            43 => {
                current_block_39 = 16476378062894899572;
            }
            50 => {
                current_block_39 = 14434307700559646593;
            }
            _ => {
                current_block_39 = 535964585519130057;
            }
        }
        match current_block_39 {
            14434307700559646593 => {
                func = (*arg).value.nodeval;
                rp = (*func).sub.nodep.r.iptr;
                ip = (*rp).nexti;
                while !ip.is_null() {
                    if (*ip).opcode as u32 != opcodeval::Op_breakpoint as i32 as u32
                        && (*ip).source_line as i32 > 0 as i32
                    {
                        stop.pc = ip;
                        stop.fcall_count = fcall_count - cur_frame;
                        stop.check_func = Some(
                            check_until
                                as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
                        );
                        stop.command = argtype::from_libc_c_uint(cmd as u32);
                        return 1 as i32;
                    }
                    ip = (*ip).nexti;
                }
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"cannot find specified location in function `%s'\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*func).sub.nodep.name,
                );
            }
            16476378062894899572 => {
                lineno = (*arg).value.lval as i32;
                if lineno <= 0 as i32 || lineno > (*s).srclines {
                    d_error(
                        dcgettext(
                            0 as *const i8,
                            b"line number %d in file `%s' out of range\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        lineno,
                        src,
                    );
                    return 0 as i32;
                }
                break 's_166;
            }
            _ => {}
        }
        return 0 as i32;
    }
    rp = find_rule(src, lineno as i64);
    if rp.is_null() {
        d_error(
            dcgettext(
                0 as *const i8,
                b"invalid source line %d in file `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            lineno,
            src,
        );
        return 0 as i32;
    }
    ip = (*rp).nexti;
    while !ip.is_null() {
        if (*ip).opcode as u32 != opcodeval::Op_breakpoint as i32 as u32
            && (*ip).source_line as i32 >= lineno
        {
            stop.pc = ip;
            stop.fcall_count = fcall_count - cur_frame;
            stop.check_func = Some(
                check_until as unsafe extern "C" fn(*mut *mut INSTRUCTION) -> i32,
            );
            stop.command = argtype::from_libc_c_uint(cmd as u32);
            return 1 as i32;
        }
        if ip == (*rp.offset(1 as i32 as isize)).d.di {
            break;
        }
        ip = (*ip).nexti;
    }
    fprintf(
        out_fp,
        dcgettext(
            0 as *const i8,
            b"cannot find specified location %d in file `%s'\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
        lineno,
        src,
    );
    return 0 as i32;
}
unsafe extern "C" fn print_watch_item(mut w: *mut list_item) {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    symbol = (*w).symbol;
    if (*w).flags & 2 as i32 != 0 as i32 {
        fprintf(out_fp, b"%s\0" as *const u8 as *const i8, (*w).sname);
        i = 0 as i32;
        while i < (*w).num_subs {
            sub = *((*w).subs).offset(i as isize);
            fprintf(
                out_fp,
                b"[\"%.*s\"]\0" as *const u8 as *const i8,
                (*sub).sub.val.slen as i32,
                (*sub).sub.val.sp,
            );
            i += 1;
            i;
        }
        fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
    } else if (*w).flags & 4 as i32 != 0 as i32 {
        fprintf(
            out_fp,
            b"$%ld\n\0" as *const u8 as *const i8,
            (*symbol).sub.val.fltnum as i64,
        );
    } else {
        fprintf(out_fp, b"%s\n\0" as *const u8 as *const i8, (*w).sname);
    }
    fprintf(out_fp, b"  Old value: \0" as *const u8 as *const i8);
    if (*w).flags & 8 as i32 != 0 as i32 {
        fprintf(
            out_fp,
            b"array, %ld elements\n\0" as *const u8 as *const i8,
            (*w).value[1 as i32 as usize].l,
        );
    } else if ((*w).value[1 as i32 as usize].n).is_null() {
        fprintf(
            out_fp,
            if (*w).flags & 2 as i32 != 0 as i32 {
                dcgettext(
                    0 as *const i8,
                    b"element not in array\n\0" as *const u8 as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"untyped variable\n\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
    } else {
        valinfo(
            (*w).value[1 as i32 as usize].n,
            Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
            out_fp,
        );
    }
    fprintf(out_fp, b"  New value: \0" as *const u8 as *const i8);
    if (*w).flags & 16 as i32 != 0 as i32 {
        fprintf(
            out_fp,
            b"array, %ld elements\n\0" as *const u8 as *const i8,
            (*w).value[0 as i32 as usize].l,
        );
    } else if ((*w).value[0 as i32 as usize].n).is_null() {
        fprintf(
            out_fp,
            if (*w).flags & 2 as i32 != 0 as i32 {
                dcgettext(
                    0 as *const i8,
                    b"element not in array\n\0" as *const u8 as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"untyped variable\n\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
    } else {
        valinfo(
            (*w).value[0 as i32 as usize].n,
            Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
            out_fp,
        );
    };
}
unsafe extern "C" fn next_command() {
    let mut current_block: u64;
    static mut last_rule: i32 = 0 as i32;
    let mut d: *mut list_item = 0 as *mut list_item;
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    if source.is_null() {
        stop.command = argtype::D_illegal;
        stop.check_func = None;
        return;
    }
    if stop.break_point != 0 {
        b = find_breakpoint(stop.break_point as i64);
        if (*b).silent {
            current_block = 7232486188964286684;
        } else {
            current_block = 7746791466490516765;
        }
    } else if stop.watch_point != 0 {
        w = find_item(&mut watch_list, stop.watch_point as i64);
        if (*w).silent != 0 {
            current_block = 7232486188964286684;
        } else {
            current_block = 7746791466490516765;
        }
    } else {
        current_block = 7746791466490516765;
    }
    match current_block {
        7746791466490516765 => {
            if cur_rule != last_rule {
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"Stopping in %s ...\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    *ruletab.as_ptr().offset(cur_rule as isize),
                );
                last_rule = cur_rule;
            }
            if !b.is_null() {
                fprintf(
                    out_fp,
                    b"Breakpoint %d, \0" as *const u8 as *const i8,
                    (*b).number,
                );
            } else if !w.is_null() {
                fprintf(
                    out_fp,
                    b"Watchpoint %d: \0" as *const u8 as *const i8,
                    (*w).number,
                );
                print_watch_item(w);
            }
            if stop.print_frame {
                print_frame((*frame_ptr).sub.nodep.x.extra, source, sourceline);
                fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
                stop.print_frame = 0 as i32 != 0;
            }
            print_lines(source, sourceline, 1 as i32);
            d = display_list.prev;
            while d != &mut display_list as *mut list_item {
                display(d);
                d = (*d).prev;
            }
        }
        _ => {}
    }
    last_printed_line = sourceline - list_size / 2 as i32;
    if last_printed_line < 0 as i32 {
        last_printed_line = 0 as i32;
    }
    s = source_find(source);
    if cur_srcfile != s {
        if (*cur_srcfile).fd != -(1 as i32) {
            close((*cur_srcfile).fd);
            (*cur_srcfile).fd = -(1 as i32);
        }
        cur_srcfile = s;
    }
    stop.command = argtype::D_illegal;
    stop.check_func = None;
    if !b.is_null() {
        let mut ret: i32 = 0;
        ret = execute_commands(&mut (*b).commands);
        if (*b).flags as i32 & 4 as i32 != 0 as i32 {
            delete_breakpoint(b);
        }
        if ret != 0 {
            return;
        }
    } else if !w.is_null() && execute_commands(&mut (*w).commands) != 0 {
        return
    }
    zzparse();
}
unsafe extern "C" fn debug_post_execute(mut pc: *mut INSTRUCTION) {
    if in_main_context() == 0 {
        return;
    }
    match (*pc).opcode as u32 {
        59 | 67 | 60 => {
            if stop.command as u32 == argtype::D_finish as i32 as u32 {
                stop.print_ret = 0 as i32 != 0;
                stop.print_frame = 0 as i32 != 0;
                stop.command = argtype::D_illegal;
                stop.check_func = None;
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"'finish' not meaningful with non-local jump '%s'\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    op2str((*pc).opcode),
                );
            } else if stop.command as u32 == argtype::D_until as i32 as u32 {
                stop.print_frame = 0 as i32 != 0;
                stop.command = argtype::D_illegal;
                stop.check_func = None;
                fprintf(
                    out_fp,
                    dcgettext(
                        0 as *const i8,
                        b"'until' not meaningful with non-local jump '%s'\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    op2str((*pc).opcode),
                );
            }
        }
        61 => {
            if stop.command as u32 == argtype::D_finish as i32 as u32
                && fcall_count == stop.fcall_count && stop.print_ret as i32 != 0
            {
                let mut r: *mut NODE = 0 as *mut NODE;
                r = (*stack_ptr).rptr;
                fprintf(out_fp, b"Returned value = \0" as *const u8 as *const i8);
                valinfo(
                    r,
                    Some(
                        fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
                    ),
                    out_fp,
                );
                stop.print_ret = 0 as i32 != 0;
            }
        }
        91 | 90 => return,
        _ => {}
    };
}
unsafe extern "C" fn debug_pre_execute(mut pi: *mut *mut INSTRUCTION) -> i32 {
    static mut cant_stop: bool = 0 as i32 != 0;
    let mut m: *mut NODE = 0 as *mut NODE;
    if in_main_context() == 0 {
        return pre_execute_code(pi);
    }
    cur_pc = *pi;
    stop.break_point = 0 as i32;
    stop.watch_point = 0 as i32;
    cur_frame = 0 as i32 as i64;
    if do_trace != 0 && (*cur_pc).opcode as u32 != opcodeval::Op_breakpoint as i32 as u32
        && stop.command as u32 != argtype::D_return as i32 as u32
    {
        print_instruction(
            cur_pc,
            Some(fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32),
            out_fp,
            0 as i32,
        );
    }
    match (*cur_pc).opcode as u32 {
        84 => {
            cant_stop = 1 as i32 != 0;
        }
        97 => {
            cant_stop = 0 as i32 != 0;
            return 1 as i32;
        }
        82 => {
            m = (*cur_pc).d.dn;
            if (*m).type_0 as u32 == nodevals::Node_var as i32 as u32
                && ((*m).sub.nodep.x.aptr).is_some()
            {
                cant_stop = 1 as i32 != 0;
            }
        }
        93 => {
            m = (*cur_pc).x.xn;
            if (*m).type_0 as u32 == nodevals::Node_var as i32 as u32
                && ((*m).sub.nodep.x.aptr).is_some()
            {
                cant_stop = 1 as i32 != 0;
            }
        }
        96 => {
            cant_stop = 0 as i32 != 0;
            return 1 as i32;
        }
        51 => {
            cur_rule = (*cur_pc).x.xl as i32;
            return 1 as i32;
        }
        101 | 95 => return 1 as i32,
        104 => {}
        _ => {
            if (*cur_pc).source_line as i32 <= 0 as i32 {
                return 1 as i32;
            }
        }
    }
    if cant_stop {
        return 1 as i32;
    }
    if check_watchpoint() != 0 {
        next_command();
        if stop.command as u32 == argtype::D_return as i32 as u32 {
            *pi = stop.pc;
        } else if (*cur_pc).opcode as u32 == opcodeval::Op_breakpoint as i32 as u32 {
            cur_pc = (*cur_pc).nexti;
        }
    } else if check_breakpoint(pi) != 0
        || (stop.check_func).is_some()
            && (stop.check_func).expect("non-null function pointer")(pi) != 0
    {
        next_command();
        if stop.command as u32 == argtype::D_return as i32 as u32 {
            *pi = stop.pc;
        }
    }
    return (cur_pc == *pi) as i32;
}
unsafe extern "C" fn print_memory(
    mut m: *mut NODE,
    mut func: *mut NODE,
    mut print_func: Func_print,
    mut fp: *mut FILE,
) {
    match (*m).type_0 as u32 {
        1 => {
            if m == Nnull_string {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"Nnull_string\0" as *const u8 as *const i8);
            } else if (*m).flags as u32 & flagvals::NUMBER as i32 as u32
                != 0 as i32 as u32
            {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"%g\0" as *const u8 as *const i8, (*m).sub.val.fltnum);
            } else if (*m).flags as u32 & flagvals::STRING as i32 as u32
                != 0 as i32 as u32
            {
                pp_string_fp(
                    print_func,
                    fp,
                    (*m).sub.val.sp,
                    (*m).sub.val.slen,
                    '"' as i32,
                    0 as i32 != 0,
                );
            } else if (*m).flags as u32 & flagvals::REGEX as i32 as u32
                != 0 as i32 as u32
            {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"@\0" as *const u8 as *const i8);
                pp_string_fp(
                    print_func,
                    fp,
                    (*m).sub.val.sp,
                    (*m).sub.val.slen,
                    '/' as i32,
                    0 as i32 != 0,
                );
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"-?-\0" as *const u8 as *const i8);
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b" [%s]\0" as *const u8 as *const i8,
                flags2str((*m).flags as i32),
            );
        }
        2 => {
            pp_string_fp(
                print_func,
                fp,
                (*(*m).sub.nodep.x.extra).sub.val.sp,
                (*(*m).sub.nodep.x.extra).sub.val.slen,
                '/' as i32,
                0 as i32 != 0,
            );
        }
        3 => {}
        8 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%s\0" as *const u8 as *const i8,
                (*((*func).sub.nodep.rn).offset((*m).sub.nodep.l.ll as isize))
                    .sub
                    .nodep
                    .name,
            );
        }
        4 | 6 | 5 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"%s\0" as *const u8 as *const i8, (*m).sub.nodep.name);
        }
        7 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"element - %p\0" as *const u8 as *const i8, m);
        }
        _ => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"?\0" as *const u8 as *const i8);
        }
    };
}
unsafe extern "C" fn print_instruction(
    mut pc: *mut INSTRUCTION,
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_dump: i32,
) {
    let mut pcount: i32 = 0 as i32;
    static mut func: *mut NODE = 0 as *const NODE as *mut NODE;
    static mut noffset: i32 = 0 as i32;
    if noffset == 0 as i32 {
        static mut buf: [i8; 50] = [0; 50];
        noffset = sprintf(
            buf.as_mut_ptr(),
            b"[      :%p] %-20.20s: \0" as *const u8 as *const i8,
            pc as *mut libc::c_void,
            opcode2str((*pc).opcode),
        );
    }
    if (*pc).opcode as u32 == opcodeval::Op_func as i32 as u32 {
        func = (*pc).x.xn;
        pcount = (*func).sub.nodep.l.ll as i32;
        if in_dump != 0 {
            let mut j: i32 = 0;
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"\n\t# Function: %s (\0" as *const u8 as *const i8,
                (*func).sub.nodep.name,
            );
            j = 0 as i32;
            while j < pcount {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"%s\0" as *const u8 as *const i8,
                    (*((*func).sub.nodep.rn).offset(j as isize)).sub.nodep.name,
                );
                if j < pcount - 1 as i32 {
                    print_func
                        .expect(
                            "non-null function pointer",
                        )(fp, b", \0" as *const u8 as *const i8);
                }
                j += 1;
                j;
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b")\n\n\0" as *const u8 as *const i8);
        }
    } else if (*pc).opcode as u32 == opcodeval::Op_rule as i32 as u32 {
        if in_dump != 0 {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"\n\t# %s\n\n\0" as *const u8 as *const i8,
                *ruletab.as_ptr().offset((*pc).x.xl as isize),
            );
        }
    }
    if (*pc).opcode as u32 == opcodeval::Op_newfile as i32 as u32 {
        print_func
            .expect("non-null function pointer")(fp, b"\n\0" as *const u8 as *const i8);
    }
    if (*pc).source_line as i32 <= 0 as i32 {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp,
            b"[      :%p] %-20.20s: \0" as *const u8 as *const i8,
            pc,
            opcode2str((*pc).opcode),
        );
    } else {
        print_func
            .expect(
                "non-null function pointer",
            )(
            fp,
            b"[%6d:%p] %-20.20s: \0" as *const u8 as *const i8,
            (*pc).source_line as i32,
            pc,
            opcode2str((*pc).opcode),
        );
    }
    if prog_running as i32 != 0 && in_dump == 0 {
        func = (*find_frame(0 as i32 as i64)).sub.nodep.x.extra;
    }
    let mut current_block_149: u64;
    match (*pc).opcode as u32 {
        117 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[branch_if = %p] [branch_else = %p] [branch_else->lasti = %p]\n\0"
                    as *const u8 as *const i8,
                (*pc).d.di,
                (*pc).x.xi,
                (*(*pc).x.xi).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        118 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[branch_end = %p]\n\0" as *const u8 as *const i8, (*pc).x.xi);
            current_block_149 = 12129449210080749085;
        }
        115 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[while_body = %p] [target_break = %p]\n\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        112 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[doloop_cond = %p] [target_break = %p]\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.di,
                (*pc).x.xi,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b" [comment = %p]\0" as *const u8 as *const i8, (*pc).comment);
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"\n\0" as *const u8 as *const i8);
            if !((*pc).comment).is_null() {
                print_instruction((*pc).comment, print_func, fp, in_dump);
            }
            current_block_149 = 12129449210080749085;
        }
        113 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[forloop_cond = %p] \0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.di,
            );
            current_block_149 = 9636104251553103026;
        }
        114 => {
            current_block_149 = 9636104251553103026;
        }
        116 => {
            let mut need_newline: bool = 0 as i32 != 0;
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[switch_start = %p] [switch_end = %p]\n\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.di,
                (*pc.offset(1 as i32 as isize)).x.xi,
            );
            if !((*pc).comment).is_null()
                || !((*(*pc.offset(1 as i32 as isize)).x.xi).comment).is_null()
            {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"%*s\0" as *const u8 as *const i8,
                    noffset,
                    b"\0" as *const u8 as *const i8,
                );
            }
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"[start_comment = %p]\0" as *const u8 as *const i8,
                    (*pc).comment,
                );
                need_newline = 1 as i32 != 0;
            }
            if !((*(*pc.offset(1 as i32 as isize)).x.xi).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"[end_comment = %p]\0" as *const u8 as *const i8,
                    (*(*pc.offset(1 as i32 as isize)).x.xi).comment,
                );
                need_newline = 1 as i32 != 0;
            }
            if need_newline {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            if !((*pc).comment).is_null() {
                print_instruction((*pc).comment, print_func, fp, in_dump);
            }
            if !((*(*pc.offset(1 as i32 as isize)).x.xi).comment).is_null() {
                print_instruction(
                    (*(*pc.offset(1 as i32 as isize)).x.xi).comment,
                    print_func,
                    fp,
                    in_dump,
                );
            }
            current_block_149 = 12129449210080749085;
        }
        53 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[stmt_start = %p] [stmt_end = %p]\0" as *const u8 as *const i8,
                (*pc).d.di,
                (*pc).x.xi,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const i8,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            current_block_149 = 12129449210080749085;
        }
        95 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[update_%s()]\n\0" as *const u8 as *const i8,
                get_spec_varname((*pc).x.aptr),
            );
            current_block_149 = 12129449210080749085;
        }
        96 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[set_%s()]\0" as *const u8 as *const i8,
                get_spec_varname((*pc).x.aptr),
            );
            if (*pc).d.dl != 0 as i32 as i64 {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b" [assign_ctxt = %s]\0" as *const u8 as *const i8,
                    opcode2str((*pc).d.dl as OPCODE),
                );
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"\n\0" as *const u8 as *const i8);
            current_block_149 = 12129449210080749085;
        }
        97 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[%s]\n\0" as *const u8 as *const i8,
                if (*pc).x.aptr == Some(reset_record as unsafe extern "C" fn() -> ()) {
                    b"reset_record()\0" as *const u8 as *const i8
                } else {
                    b"invalidate_field0()\0" as *const u8 as *const i8
                },
            );
            current_block_149 = 12129449210080749085;
        }
        84 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_assign = %p] [do_reference = %s]\n\0" as *const u8
                    as *const i8,
                (*pc).d.di,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
            current_block_149 = 12129449210080749085;
        }
        101 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[param_cnt = %d] [source_file = %s]\0" as *const u8 as *const i8,
                pcount,
                if !((*pc).d.name).is_null() {
                    (*pc).d.name
                } else {
                    b"cmd. line\0" as *const u8 as *const i8
                },
            );
            if !((*pc.offset(3 as i32 as isize)).nexti).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"[ns_list = %p]\n\0" as *const u8 as *const i8,
                    (*pc.offset(3 as i32 as isize)).nexti,
                );
                print_ns_list(
                    (*pc.offset(3 as i32 as isize)).nexti,
                    print_func,
                    fp,
                    in_dump,
                );
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            current_block_149 = 12129449210080749085;
        }
        65 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[into_var = %s] [redir_type = \"%s\"]\n\0" as *const u8 as *const i8,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
                redir2str((*pc).d.dl as i32),
            );
            current_block_149 = 12129449210080749085;
        }
        66 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[into_var = %s]\n\0" as *const u8 as *const i8,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%*s[target_beginfile = %p] [target_endfile = %p]\n\0" as *const u8
                    as *const i8,
                noffset,
                b"\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.di,
                (*pc.offset(1 as i32 as isize)).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        57 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[redir_type = \"%s\"]\n\0" as *const u8 as *const i8,
                redir2str((*pc).d.dl as i32),
            );
            current_block_149 = 12129449210080749085;
        }
        56 | 58 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[expr_count = %ld] [redir_type = \"%s\"]\n\0" as *const u8
                    as *const i8,
                (*pc).x.xl,
                redir2str((*pc).d.dl as i32),
            );
            current_block_149 = 12129449210080749085;
        }
        74 | 73 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[func_name = %s] [arg_count = %ld]\n\0" as *const u8 as *const i8,
                (*pc).d.name,
                (*pc.offset(1 as i32 as isize)).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        67 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_newfile = %p] [target_endfile = %p]\n\0" as *const u8
                    as *const i8,
                (*pc).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        91 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_jmp = %p] [target_endfile = %p]\n\0" as *const u8 as *const i8,
                (*pc).d.di,
                (*pc).x.xi,
            );
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%*s[target_get_record = %p]\n\0" as *const u8 as *const i8,
                noffset,
                b"\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        90 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_newfile = %p]\n\0" as *const u8 as *const i8,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        87 | 89 | 88 | 38 | 40 | 59 | 92 | 54 | 55 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[target_jmp = %p]\n\0" as *const u8 as *const i8, (*pc).d.di);
            current_block_149 = 12129449210080749085;
        }
        60 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_end = %p] [target_atexit = %p]\n\0" as *const u8 as *const i8,
                (*pc).d.di,
                (*pc).x.xi,
            );
            current_block_149 = 12129449210080749085;
        }
        52 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_jmp = %p] [match_exp = %s]\0" as *const u8 as *const i8,
                (*pc).d.di,
                if (*pc.offset(1 as i32 as isize)).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const i8,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            current_block_149 = 12129449210080749085;
        }
        68 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[namespace = %s]\0" as *const u8 as *const i8, (*pc).d.name);
            if !((*pc).nexti).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"[nexti = %p]\0" as *const u8 as *const i8, (*pc).nexti);
            }
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"[comment = %p]\0" as *const u8 as *const i8, (*pc).comment);
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"\n\0" as *const u8 as *const i8);
            current_block_149 = 12129449210080749085;
        }
        93 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[array_var = %s] [target_jmp = %p]\n\0" as *const u8 as *const i8,
                if (*(*pc).x.xn).type_0 as u32 == nodevals::Node_param_list as i32 as u32
                {
                    (*((*func).sub.nodep.rn)
                        .offset((*(*pc).x.xn).sub.nodep.l.ll as isize))
                        .sub
                        .nodep
                        .name
                } else {
                    (*(*pc).x.xn).sub.nodep.name
                },
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        14 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[triggered = %ld] [target_jmp = %p]\n\0" as *const u8 as *const i8,
                (*pc).x.xl,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        15 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[line_range = %p] [target_jmp = %p]\n\0" as *const u8 as *const i8,
                (*pc).x.xi,
                (*pc).d.di,
            );
            current_block_149 = 12129449210080749085;
        }
        70 => {
            let mut fname: *const i8 = b"sub\0" as *const u8 as *const i8;
            static mut values: [flagtab; 4] = [
                {
                    let mut init = flagtab {
                        val: 0x1 as i32,
                        name: b"GSUB\0" as *const u8 as *const i8,
                    };
                    init
                },
                {
                    let mut init = flagtab {
                        val: 0x2 as i32,
                        name: b"GENSUB\0" as *const u8 as *const i8,
                    };
                    init
                },
                {
                    let mut init = flagtab {
                        val: 0x4 as i32,
                        name: b"LITERAL\0" as *const u8 as *const i8,
                    };
                    init
                },
                {
                    let mut init = flagtab {
                        val: 0 as i32,
                        name: 0 as *const i8,
                    };
                    init
                },
            ];
            if (*pc).d.dl & 0x1 as i32 as i64 != 0 as i32 as i64 {
                fname = b"gsub\0" as *const u8 as *const i8;
            } else if (*pc).d.dl & 0x2 as i32 as i64 != 0 as i32 as i64 {
                fname = b"gensub\0" as *const u8 as *const i8;
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%s [arg_count = %ld] [sub_flags = %s]\n\0" as *const u8 as *const i8,
                fname,
                (*pc).x.xl,
                genflags2str((*pc).d.dl as i32, values.as_ptr()),
            );
            current_block_149 = 12129449210080749085;
        }
        69 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%s [arg_count = %ld]\n\0" as *const u8 as *const i8,
                getfname((*pc).d.fptr, 0 as i32 != 0),
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        71 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"%s [arg_count = %ld]\n\0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).d.name,
                (*pc).x.xl,
            );
            current_block_149 = 12129449210080749085;
        }
        16 | 17 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[sub_count = %ld]\n\0" as *const u8 as *const i8, (*pc).d.dl);
            current_block_149 = 12129449210080749085;
        }
        28 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b" [sub_count = %ld]\n\0" as *const u8 as *const i8, (*pc).x.xl);
            current_block_149 = 12129449210080749085;
        }
        83 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[sub_count = %ld] [do_reference = %s]\n\0" as *const u8 as *const i8,
                (*pc).d.dl,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
            current_block_149 = 12129449210080749085;
        }
        63 | 72 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[expr_count = %ld]\n\0" as *const u8 as *const i8, (*pc).x.xl);
            current_block_149 = 12129449210080749085;
        }
        13 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[expr_count = %ld] [concat_flag = %s]\n\0" as *const u8 as *const i8,
                (*pc).x.xl,
                if (*pc).d.dl & 1 as i32 as i64 != 0 as i32 as i64 {
                    b"CSUBSEP\0" as *const u8 as *const i8
                } else {
                    b"0\0" as *const u8 as *const i8
                },
            );
            current_block_149 = 12129449210080749085;
        }
        51 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[in_rule = %s] [source_file = %s]\0" as *const u8 as *const i8,
                *ruletab.as_ptr().offset((*pc).x.xl as isize),
                if !((*pc).d.name).is_null() {
                    (*pc).d.name
                } else {
                    b"cmd. line\0" as *const u8 as *const i8
                },
            );
            if !((*pc.offset(3 as i32 as isize)).nexti).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b"[ns_list = %p]\n\0" as *const u8 as *const i8,
                    (*pc.offset(3 as i32 as isize)).nexti,
                );
                print_ns_list(
                    (*pc.offset(3 as i32 as isize)).nexti,
                    print_func,
                    fp,
                    in_dump,
                );
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            current_block_149 = 12129449210080749085;
        }
        105 => {
            static mut linttypetab: [*const i8; 3] = [
                b"LINT_illegal\0" as *const u8 as *const i8,
                b"LINT_assign_in_cond\0" as *const u8 as *const i8,
                b"LINT_no_effect\0" as *const u8 as *const i8,
            ];
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[lint_type = %s]\n\0" as *const u8 as *const i8,
                linttypetab[(*pc).d.dl as usize],
            );
            current_block_149 = 12129449210080749085;
        }
        103 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"[exec_count = %llu]\n\0" as *const u8 as *const i8, (*pc).d.ldl);
            current_block_149 = 12129449210080749085;
        }
        27 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            if !((*pc).x.xn).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b" = \0" as *const u8 as *const i8);
                print_memory((*pc).x.xn, func, print_func, fp);
            }
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"\n\0" as *const u8 as *const i8);
            current_block_149 = 12129449210080749085;
        }
        82 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b" [do_reference = %s]\n\0" as *const u8 as *const i8,
                if (*pc).x.xl != 0 {
                    b"true\0" as *const u8 as *const i8
                } else {
                    b"false\0" as *const u8 as *const i8
                },
            );
            current_block_149 = 12129449210080749085;
        }
        102 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b" [comment_type = %s]\0" as *const u8 as *const i8,
                if (*(*pc).d.dn).sub.val.comtype as u32
                    == commenttype::EOL_COMMENT as i32 as u32
                {
                    b"EOL\0" as *const u8 as *const i8
                } else {
                    b"BLOCK\0" as *const u8 as *const i8
                },
            );
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const i8,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
            current_block_149 = 12129449210080749085;
        }
        78 | 75 | 76 | 77 | 81 | 80 | 79 | 49 | 48 | 50 | 8 | 10 | 2 | 12 | 4 | 6
        | 37 => {
            print_memory((*pc).d.dn, func, print_func, fp);
            current_block_149 = 13915698984488262895;
        }
        _ => {
            current_block_149 = 13915698984488262895;
        }
    }
    match current_block_149 {
        9636104251553103026 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[forloop_body = %p] \0" as *const u8 as *const i8,
                (*pc.offset(1 as i32 as isize)).x.xi,
            );
            print_func
                .expect(
                    "non-null function pointer",
                )(
                fp,
                b"[target_break = %p] [target_continue = %p]\0" as *const u8
                    as *const i8,
                (*pc).x.xi,
                (*pc).d.di,
            );
            if !((*pc).comment).is_null() {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(
                    fp,
                    b" [comment = %p]\n\0" as *const u8 as *const i8,
                    (*pc).comment,
                );
                print_instruction((*pc).comment, print_func, fp, in_dump);
            } else {
                print_func
                    .expect(
                        "non-null function pointer",
                    )(fp, b"\n\0" as *const u8 as *const i8);
            }
        }
        13915698984488262895 => {
            print_func
                .expect(
                    "non-null function pointer",
                )(fp, b"\n\0" as *const u8 as *const i8);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_trace_instruction(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_argument as i32 as u32
        && (*arg).value.lval == nametypeval::A_TRACE_ON as i32 as i64
    {
        do_trace = 1 as i32;
    } else {
        do_trace = 0 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn print_code(
    mut pc: *mut INSTRUCTION,
    mut x: *mut libc::c_void,
) -> i32 {
    let mut data: *mut pf_data = x as *mut pf_data;
    while !pc.is_null() {
        print_instruction(pc, (*data).print_func, (*data).fp, (*data).defn as i32);
        pc = (*pc).nexti;
    }
    return 0 as i32;
}
unsafe extern "C" fn print_ns_list(
    mut pc: *mut INSTRUCTION,
    mut print_func: Func_print,
    mut fp: *mut FILE,
    mut in_dump: i32,
) {
    while !pc.is_null() {
        print_instruction(pc, print_func, fp, in_dump);
        if !((*pc).comment).is_null() {
            print_instruction((*pc).comment, print_func, fp, in_dump);
        }
        pc = (*pc).nexti;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_dump_instructions(
    mut arg: *mut CMDARG,
    mut cmd: i32,
) -> i32 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut funcs: *mut *mut NODE = 0 as *mut *mut NODE;
    if !arg.is_null() && (*arg).type_0 as u32 == argtype::D_string as i32 as u32 {
        fp = fopen((*arg).value.sval, b"w\0" as *const u8 as *const i8);
        if fp.is_null() {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"could not open `%s' for writing: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*arg).value.sval,
                strerror(*__errno_location()),
            );
            return 0 as i32;
        }
        pf_data.print_func = Some(
            fprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
        );
        pf_data.fp = fp;
        pf_data.defn = 1 as i32 != 0;
        print_code(code_block, &mut pf_data as *mut pf_data as *mut libc::c_void);
        funcs = function_list(1 as i32 != 0);
        foreach_func(
            funcs,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
                Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
            >(
                Some(
                    print_code
                        as unsafe extern "C" fn(
                            *mut INSTRUCTION,
                            *mut libc::c_void,
                        ) -> i32,
                ),
            ),
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
        pma_free(funcs as *mut libc::c_void);
        fclose(fp);
        return 0 as i32;
    }
    funcs = function_list(1 as i32 != 0);
    if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
        pf_data.print_func = Some(
            gprintf as unsafe extern "C" fn(*mut FILE, *const i8, ...) -> i32,
        );
        pf_data.fp = out_fp;
        pf_data.defn = 1 as i32 != 0;
        print_code(code_block, &mut pf_data as *mut pf_data as *mut libc::c_void);
        foreach_func(
            funcs,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
                Option<unsafe extern "C" fn(*mut INSTRUCTION, *mut libc::c_void) -> i32>,
            >(
                Some(
                    print_code
                        as unsafe extern "C" fn(
                            *mut INSTRUCTION,
                            *mut libc::c_void,
                        ) -> i32,
                ),
            ),
            &mut pf_data as *mut pf_data as *mut libc::c_void,
        );
    }
    pma_free(funcs as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_save(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_option(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut value: *mut i8 = 0 as *mut i8;
    if arg.is_null() {
        opt = option_list.as_ptr();
        while !((*opt).name).is_null() {
            if !((*opt).str_val).is_null() {
                fprintf(
                    out_fp,
                    b"%s = \"%s\"\n\0" as *const u8 as *const i8,
                    (*opt).name,
                    *(*opt).str_val,
                );
            } else {
                fprintf(
                    out_fp,
                    b"%s = %d\n\0" as *const u8 as *const i8,
                    (*opt).name,
                    *(*opt).num_val,
                );
            }
            opt = opt.offset(1);
            opt;
        }
        return 0 as i32;
    }
    name = (*arg).value.sval;
    arg = (*arg).next;
    value = if !arg.is_null() { (*arg).value.sval } else { 0 as *mut i8 };
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if strcmp(name, (*opt).name) == 0 as i32 {
            break;
        }
        opt = opt.offset(1);
        opt;
    }
    if ((*opt).name).is_null() {
        return 0 as i32;
    }
    if value.is_null() {
        if !((*opt).str_val).is_null() {
            fprintf(
                out_fp,
                b"%s = \"%s\"\n\0" as *const u8 as *const i8,
                (*opt).name,
                *(*opt).str_val,
            );
        } else {
            fprintf(
                out_fp,
                b"%s = %d\n\0" as *const u8 as *const i8,
                (*opt).name,
                *(*opt).num_val,
            );
        }
    } else {
        (Some(((*opt).assign).expect("non-null function pointer")))
            .expect("non-null function pointer")(value);
    }
    return 0 as i32;
}
unsafe extern "C" fn prompt_continue(mut fp: *mut FILE) {
    let mut quit_pager: bool = 0 as i32 != 0;
    if os_isatty(fileno(fp)) != 0 && input_fd == 0 as i32 {
        quit_pager = prompt_yes_no(
            dcgettext(
                0 as *const i8,
                b"\t------[Enter] to continue or [q] + [Enter] to quit------\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            'q' as i32 as i8,
            0 as i32,
            fp,
        ) != 0;
    }
    if quit_pager {
        longjmp(pager_quit_tag.as_mut_ptr(), 1 as i32);
    }
    pager_lines_printed = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gprintf(
    mut fp: *mut FILE,
    mut format: *const i8,
    mut args: ...
) -> i32 {
    let mut args_0: ::core::ffi::VaListImpl;
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut buflen: size_t = 0 as i32 as size_t;
    static mut bl: i32 = 0 as i32;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut nchar: i32 = 0;
    if buf.is_null() {
        buflen = 512 as i32 as size_t;
        buf = emalloc_real(
            buflen.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            b"gprintf\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"debug.c\0" as *const u8 as *const i8,
            4364 as i32,
        ) as *mut i8;
    } else if buflen.wrapping_sub(bl as u64) < (512 as i32 / 2 as i32) as u64 {
        buflen = (buflen as u64).wrapping_add(512 as i32 as u64) as size_t as size_t;
        buf = erealloc_real(
            buf as *mut libc::c_void,
            buflen.wrapping_mul(::core::mem::size_of::<i8>() as u64),
            b"gprintf\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"debug.c\0" as *const u8 as *const i8,
            4367 as i32,
        ) as *mut i8;
    }
    loop {
        args_0 = args.clone();
        nchar = vsnprintf(
            buf.offset(bl as isize),
            buflen.wrapping_sub(bl as u64),
            format,
            args_0.as_va_list(),
        );
        if nchar == 0 as i32 {
            return 0 as i32;
        }
        if nchar > 0 as i32 && (nchar as u64) < buflen.wrapping_sub(bl as u64) {
            bl += nchar;
            if *buf.offset((bl - 1 as i32) as isize) as i32 != '\n' as i32 {
                return nchar;
            }
            break;
        } else {
            buflen = (buflen as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
            buf = erealloc_real(
                buf as *mut libc::c_void,
                buflen.wrapping_mul(::core::mem::size_of::<i8>() as u64),
                b"gprintf\0" as *const u8 as *const i8,
                b"buf\0" as *const u8 as *const i8,
                b"debug.c\0" as *const u8 as *const i8,
                4386 as i32,
            ) as *mut i8;
        }
    }
    bl = 0 as i32;
    p = buf;
    loop {
        q = strchr(p, '\n' as i32);
        if q.is_null() {
            break;
        }
        let mut sz: i32 = q.offset_from(p) as i64 as i32;
        while sz > 0 as i32 {
            let mut cnt: i32 = 0;
            cnt = if sz > screen_width { screen_width } else { sz };
            if cnt < sz && pager_lines_printed == screen_height - 2 as i32 {
                prompt_continue(fp);
            }
            if (if 0 != 0 && 0 != 0
                && (::core::mem::size_of::<i8>() as u64).wrapping_mul(cnt as size_t)
                    <= 8 as i32 as u64
                && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
            {
                ({
                    let mut __ptr: *const i8 = p as *const i8;
                    let mut __stream: *mut FILE = fp;
                    let mut __cnt: size_t = 0;
                    __cnt = (::core::mem::size_of::<i8>() as u64)
                        .wrapping_mul(cnt as size_t);
                    while __cnt > 0 as i32 as u64 {
                        if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                            as i32 as i64 != 0
                        {
                            let fresh11 = __ptr;
                            __ptr = __ptr.offset(1);
                            __overflow(__stream, *fresh11 as u8 as i32)
                        } else {
                            let fresh12 = __ptr;
                            __ptr = __ptr.offset(1);
                            let fresh13 = (*__stream)._IO_write_ptr;
                            (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                .offset(1);
                            *fresh13 = *fresh12;
                            *fresh13 as u8 as i32
                        }) == -(1 as i32)
                        {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (::core::mem::size_of::<i8>() as u64)
                        .wrapping_mul(cnt as size_t)
                        .wrapping_sub(__cnt)
                        .wrapping_div(::core::mem::size_of::<i8>() as u64)
                })
            } else {
                (if 0 != 0 && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                    || 0 != 0 && cnt as size_t == 0 as i32 as u64
                {
                    0 as i32 as size_t
                } else {
                    fwrite_unlocked(
                        p as *const libc::c_void,
                        ::core::mem::size_of::<i8>() as u64,
                        cnt as size_t,
                        fp,
                    )
                })
            }) != cnt as u64
            {
                return -(1 as i32);
            }
            if cnt == sz {
                break;
            }
            pager_lines_printed += 1;
            if pager_lines_printed == screen_height - 1 as i32 {
                prompt_continue(fp);
            }
            sz -= screen_width;
            p = p.offset(cnt as isize);
        }
        fprintf(fp, b"\n\0" as *const u8 as *const i8);
        pager_lines_printed += 1;
        if pager_lines_printed == screen_height - 1 as i32 {
            prompt_continue(fp);
        }
        p = p.offset(1);
        p;
        p = q.offset(1 as i32 as isize);
    }
    return nchar;
}
unsafe extern "C" fn serialize_subscript(
    mut buf: *mut i8,
    mut buflen: i32,
    mut item: *mut list_item,
) -> i32 {
    let mut bl: i32 = 0;
    let mut nchar: i32 = 0;
    let mut i: i32 = 0;
    let mut sub: *mut NODE = 0 as *mut NODE;
    nchar = snprintf(
        buf,
        buflen as u64,
        b"%d%c%d%c%s%c%d%c\0" as *const u8 as *const i8,
        (*item).number,
        '\u{1f}' as i32 as i8 as i32,
        argtype::D_subscript as i32,
        '\u{1f}' as i32 as i8 as i32,
        (*item).sname,
        '\u{1f}' as i32 as i8 as i32,
        (*item).num_subs,
        '\u{1f}' as i32 as i8 as i32,
    );
    if nchar <= 0 as i32 { return 0 as i32 } else if nchar >= buflen { return nchar }
    bl = nchar;
    i = 0 as i32;
    while i < (*item).num_subs {
        sub = *((*item).subs).offset(i as isize);
        nchar = snprintf(
            buf.offset(bl as isize),
            (buflen - bl) as u64,
            b"%lu%c%.*s%c\0" as *const u8 as *const i8,
            (*sub).sub.val.slen,
            '\u{1f}' as i32 as i8 as i32,
            (*sub).sub.val.slen as i32,
            (*sub).sub.val.sp,
            '\u{1f}' as i32 as i8 as i32,
        );
        if nchar <= 0 as i32 {
            return 0 as i32;
        }
        bl += nchar;
        if bl >= buflen {
            return bl;
        }
        i += 1;
        i;
    }
    return bl;
}
unsafe extern "C" fn serialize_list(mut type_0: i32) {
    static mut buf: *mut i8 = 0 as *const i8 as *mut i8;
    static mut buflen: i32 = 0 as i32;
    let mut bl: i32 = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut wd: *mut list_item = 0 as *mut list_item;
    let mut hist_list: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut hist_index: i32 = 0 as i32;
    let mut opt: *mut dbg_option = 0 as *mut dbg_option;
    let mut commands: *mut commands_item = 0 as *mut commands_item;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut cnum: i32 = 0 as i32;
    let mut cndn: *mut condition = 0 as *mut condition;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut end_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    match type_0 {
        1 => {
            end_ptr = &mut breakpoints as *mut BREAKPOINT as *mut libc::c_void;
            ptr = breakpoints.prev as *mut libc::c_void;
        }
        2 => {
            end_ptr = &mut watch_list as *mut list_item as *mut libc::c_void;
            ptr = watch_list.prev as *mut libc::c_void;
        }
        3 => {
            end_ptr = &mut display_list as *mut list_item as *mut libc::c_void;
            ptr = display_list.prev as *mut libc::c_void;
        }
        4 => {
            hist_list = 0 as *mut *mut libc::c_void;
            if hist_list.is_null() {
                return;
            }
            end_ptr = 0 as *mut libc::c_void;
            ptr = *hist_list.offset(0 as i32 as isize);
        }
        5 => {
            let mut n: i32 = 0;
            n = (::core::mem::size_of::<[dbg_option; 8]>() as u64)
                .wrapping_div(::core::mem::size_of::<dbg_option>() as u64) as i32;
            end_ptr = &*option_list.as_ptr().offset((n - 1 as i32) as isize)
                as *const dbg_option as *mut libc::c_void;
            ptr = option_list.as_ptr() as *mut libc::c_void;
        }
        _ => return,
    }
    if type_0 != C2RustUnnamed_13::HISTORY as i32 && ptr == end_ptr {
        return;
    }
    if buf.is_null() {
        buflen = 512 as i32;
        buf = emalloc_real(
            (buflen + 1 as i32) as size_t,
            b"serialize\0" as *const u8 as *const i8,
            b"buf\0" as *const u8 as *const i8,
            b"debug.c\0" as *const u8 as *const i8,
            4516 as i32,
        ) as *mut i8;
    }
    bl = 0 as i32;
    while ptr != end_ptr {
        let mut current_block_112: u64;
        let mut nchar: i32 = 0 as i32;
        if buflen - bl < 512 as i32 / 2 as i32 {
            current_block_112 = 13512958342466939935;
        } else {
            current_block_112 = 4488286894823169796;
        }
        loop {
            match current_block_112 {
                13512958342466939935 => {
                    buflen *= 2 as i32;
                    buf = erealloc_real(
                        buf as *mut libc::c_void,
                        (buflen + 1 as i32) as size_t,
                        b"serialize\0" as *const u8 as *const i8,
                        b"buf\0" as *const u8 as *const i8,
                        b"debug.c\0" as *const u8 as *const i8,
                        4525 as i32,
                    ) as *mut i8;
                    current_block_112 = 4488286894823169796;
                }
                _ => {
                    match type_0 {
                        1 => {
                            b = ptr as *mut BREAKPOINT;
                            nchar = snprintf(
                                buf.offset(bl as isize),
                                (buflen - bl) as u64,
                                b"%s%c%d%c%d%c%d%c%d%c%d%c\0" as *const u8 as *const i8,
                                (*b).src,
                                '\u{1f}' as i32 as i8 as i32,
                                (*(*b).bpi).source_line as i32,
                                '\u{1f}' as i32 as i8 as i32,
                                (*b).flags as i32,
                                '\u{1f}' as i32 as i8 as i32,
                                (*b).ignore_count as i32,
                                '\u{1f}' as i32 as i8 as i32,
                                (*b).hit_count as i32,
                                '\u{1f}' as i32 as i8 as i32,
                                (*b).number,
                                '\u{1f}' as i32 as i8 as i32,
                            );
                            cnum = (*b).number;
                            commands = &mut (*b).commands;
                            cndn = &mut (*b).cndn;
                        }
                        3 | 2 => {
                            wd = ptr as *mut list_item;
                            if (*wd).flags & 1 as i32 != 0 as i32 {
                                nchar = 0 as i32;
                            } else if (*wd).flags & 2 as i32 != 0 as i32 {
                                nchar = serialize_subscript(
                                    buf.offset(bl as isize),
                                    buflen - bl,
                                    wd,
                                );
                            } else if (*wd).flags & 4 as i32 != 0 as i32 {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as u64,
                                    b"%d%c%d%c%d%c\0" as *const u8 as *const i8,
                                    (*wd).number,
                                    '\u{1f}' as i32 as i8 as i32,
                                    argtype::D_field as i32,
                                    '\u{1f}' as i32 as i8 as i32,
                                    (*(*wd).symbol).sub.val.fltnum as i64 as i32,
                                    '\u{1f}' as i32 as i8 as i32,
                                );
                            } else {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as u64,
                                    b"%d%c%d%c%s%c\0" as *const u8 as *const i8,
                                    (*wd).number,
                                    '\u{1f}' as i32 as i8 as i32,
                                    argtype::D_variable as i32,
                                    '\u{1f}' as i32 as i8 as i32,
                                    (*wd).sname,
                                    '\u{1f}' as i32 as i8 as i32,
                                );
                            }
                            cnum = (*wd).number;
                            commands = &mut (*wd).commands;
                            cndn = &mut (*wd).cndn;
                        }
                        5 => {
                            opt = ptr as *mut dbg_option;
                            if !((*opt).num_val).is_null() {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as u64,
                                    b"%s%c%d%c\0" as *const u8 as *const i8,
                                    (*opt).name,
                                    '\u{1f}' as i32 as i8 as i32,
                                    *(*opt).num_val,
                                    '\u{1f}' as i32 as i8 as i32,
                                );
                            } else {
                                nchar = snprintf(
                                    buf.offset(bl as isize),
                                    (buflen - bl) as u64,
                                    b"%s%c%s%c\0" as *const u8 as *const i8,
                                    (*opt).name,
                                    '\u{1f}' as i32 as i8 as i32,
                                    *(*opt).str_val,
                                    '\u{1f}' as i32 as i8 as i32,
                                );
                            }
                        }
                        4 | _ => {}
                    }
                    if nchar == 0 as i32 {
                        break;
                    }
                    if !(nchar > 0 as i32 && nchar < buflen - bl) {
                        current_block_112 = 13512958342466939935;
                        continue;
                    }
                    bl += nchar;
                    *buf.offset(bl as isize) = '\u{1e}' as i32 as i8;
                    bl += 1;
                    *buf.offset(bl as isize) = '\0' as i32 as i8;
                    break;
                }
            }
        }
        match type_0 {
            1 | 2 => {
                bl -= 1;
                bl;
                nchar = 0 as i32;
                c = (*commands).next;
                while c != commands {
                    nchar = (nchar as u64)
                        .wrapping_add(
                            (strlen((*c).cmd_string)).wrapping_add(1 as i32 as u64),
                        ) as i32 as i32;
                    if (*c).cmd == argtype::D_eval as i32 {
                        let mut a: *mut CMDARG = (*c).arg;
                        nchar = (nchar as u64)
                            .wrapping_add(
                                (strlen((*a).value.sval)).wrapping_add(1 as i32 as u64),
                            ) as i32 as i32;
                        nchar = (nchar as u64)
                            .wrapping_add(
                                (strlen(b"end\0" as *const u8 as *const i8))
                                    .wrapping_add(1 as i32 as u64),
                            ) as i32 as i32;
                    }
                    c = (*c).next;
                }
                if nchar > 0 as i32 {
                    nchar = (nchar as u64)
                        .wrapping_add(
                            (strlen(b"commands \0" as *const u8 as *const i8))
                                .wrapping_add(20 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                                .wrapping_add(strlen(b"end\0" as *const u8 as *const i8))
                                .wrapping_add(1 as i32 as u64),
                        ) as i32 as i32;
                    if nchar >= buflen - bl {
                        buflen = bl + nchar + 1 as i32;
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            (buflen + 1 as i32) as size_t,
                            b"serialize_list\0" as *const u8 as *const i8,
                            b"buf\0" as *const u8 as *const i8,
                            b"debug.c\0" as *const u8 as *const i8,
                            4629 as i32,
                        ) as *mut i8;
                    }
                    nchar = sprintf(
                        buf.offset(bl as isize),
                        b"commands %d\0" as *const u8 as *const i8,
                        cnum,
                    );
                    bl += nchar;
                    let fresh14 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh14 as isize) = '\u{1d}' as i32 as i8;
                    c = (*commands).next;
                    while c != commands {
                        nchar = strlen((*c).cmd_string) as i32;
                        memcpy(
                            buf.offset(bl as isize) as *mut libc::c_void,
                            (*c).cmd_string as *const libc::c_void,
                            nchar as u64,
                        );
                        bl += nchar;
                        let fresh15 = bl;
                        bl = bl + 1;
                        *buf.offset(fresh15 as isize) = '\u{1d}' as i32 as i8;
                        if (*c).cmd == argtype::D_eval as i32 {
                            let mut a_0: *mut CMDARG = (*c).arg;
                            nchar = strlen((*a_0).value.sval) as i32;
                            memcpy(
                                buf.offset(bl as isize) as *mut libc::c_void,
                                (*a_0).value.sval as *const libc::c_void,
                                nchar as u64,
                            );
                            bl += nchar;
                            let fresh16 = bl;
                            bl = bl + 1;
                            *buf.offset(fresh16 as isize) = '\u{1d}' as i32 as i8;
                            nchar = strlen(b"end\0" as *const u8 as *const i8) as i32;
                            memcpy(
                                buf.offset(bl as isize) as *mut libc::c_void,
                                b"end\0" as *const u8 as *const i8 as *const libc::c_void,
                                nchar as u64,
                            );
                            bl += nchar;
                            let fresh17 = bl;
                            bl = bl + 1;
                            *buf.offset(fresh17 as isize) = '\u{1d}' as i32 as i8;
                        }
                        c = (*c).next;
                    }
                    nchar = strlen(b"end\0" as *const u8 as *const i8) as i32;
                    memcpy(
                        buf.offset(bl as isize) as *mut libc::c_void,
                        b"end\0" as *const u8 as *const i8 as *const libc::c_void,
                        nchar as u64,
                    );
                    bl += nchar;
                    let fresh18 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh18 as isize) = '\u{1f}' as i32 as i8;
                }
                let fresh19 = bl;
                bl = bl + 1;
                *buf.offset(fresh19 as isize) = '\u{1e}' as i32 as i8;
                *buf.offset(bl as isize) = '\0' as i32 as i8;
                if !((*cndn).expr).is_null() {
                    bl -= 1;
                    bl;
                    nchar = strlen((*cndn).expr) as i32;
                    if nchar + 1 as i32 >= buflen - bl {
                        buflen = bl + nchar + 1 as i32 + 1 as i32;
                        buf = erealloc_real(
                            buf as *mut libc::c_void,
                            (buflen + 1 as i32) as size_t,
                            b"serialize_list\0" as *const u8 as *const i8,
                            b"buf\0" as *const u8 as *const i8,
                            b"debug.c\0" as *const u8 as *const i8,
                            4666 as i32,
                        ) as *mut i8;
                    }
                    memcpy(
                        buf.offset(bl as isize) as *mut libc::c_void,
                        (*cndn).expr as *const libc::c_void,
                        nchar as u64,
                    );
                    bl += nchar;
                    let fresh20 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh20 as isize) = '\u{1f}' as i32 as i8;
                    let fresh21 = bl;
                    bl = bl + 1;
                    *buf.offset(fresh21 as isize) = '\u{1e}' as i32 as i8;
                    *buf.offset(bl as isize) = '\0' as i32 as i8;
                }
                ptr = if type_0 == C2RustUnnamed_13::BREAK as i32 {
                    (*b).prev as *mut libc::c_void
                } else {
                    (*wd).prev as *mut libc::c_void
                };
            }
            3 => {
                ptr = (*wd).prev as *mut libc::c_void;
            }
            4 => {
                hist_index += 1;
                ptr = *hist_list.offset(hist_index as isize);
            }
            5 => {
                opt = opt.offset(1);
                ptr = opt as *mut libc::c_void;
            }
            _ => {}
        }
    }
    if bl > 0 as i32 {
        setenv(env_variable[type_0 as usize], buf, 1 as i32);
    }
}
unsafe extern "C" fn unserialize_commands(mut str: *mut i8, mut str_len: i32) {
    if str_len <= 0 as i32 || str.is_null() {
        return;
    }
    commands_string = str;
    commands_string_len = str_len;
    push_cmd_src(
        -(1 as i32),
        0 as i32 != 0,
        Some(read_commands_string as unsafe extern "C" fn(*const i8) -> *mut i8),
        None,
        0 as i32,
        2 as i32,
    );
    line_sep = '\u{1d}' as i32 as i8;
    zzparse();
    pop_cmd_src();
}
unsafe extern "C" fn unserialize_list_item(
    mut list: *mut list_item,
    mut pstr: *mut *mut i8,
    mut pstr_len: *mut i32,
    mut field_cnt: i32,
) -> *mut list_item {
    let mut num: i32 = 0;
    let mut type_0: i32 = 0;
    let mut i: i32 = 0;
    let mut l: *mut list_item = 0 as *mut list_item;
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut sub_cnt: i32 = 0 as i32;
    let mut cnt: i32 = 0;
    let mut subs: *mut *mut NODE = 0 as *mut *mut NODE;
    num = strtol(*pstr.offset(0 as i32 as isize), 0 as *mut *mut i8, 0 as i32) as i32;
    type_0 = strtol(*pstr.offset(1 as i32 as isize), 0 as *mut *mut i8, 0 as i32) as i32;
    if type_0 == argtype::D_field as i32 {
        let mut field_num: i32 = 0;
        field_num = strtol(*pstr.offset(2 as i32 as isize), 0 as *mut *mut i8, 0 as i32)
            as i32;
        symbol = make_number
            .expect("non-null function pointer")(field_num as libc::c_double);
        cnt = 3 as i32;
    } else {
        let mut name: *mut i8 = 0 as *mut i8;
        name = estrdup(
            *pstr.offset(2 as i32 as isize),
            *pstr_len.offset(2 as i32 as isize) as size_t,
        );
        symbol = find_symbol(name, 0 as *mut *mut i8);
        pma_free(name as *mut libc::c_void);
        if symbol.is_null() {
            return 0 as *mut list_item;
        }
        cnt = 3 as i32;
        if type_0 == argtype::D_subscript as i32 {
            let mut sub_len: i32 = 0;
            sub_cnt = strtol(
                *pstr.offset(3 as i32 as isize),
                0 as *mut *mut i8,
                0 as i32,
            ) as i32;
            subs = emalloc_real(
                (sub_cnt as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                b"unserialize_list_item\0" as *const u8 as *const i8,
                b"subs\0" as *const u8 as *const i8,
                b"debug.c\0" as *const u8 as *const i8,
                4745 as i32,
            ) as *mut *mut NODE;
            cnt += 1;
            cnt;
            i = 0 as i32;
            while i < sub_cnt {
                sub_len = strtol(*pstr.offset(cnt as isize), 0 as *mut *mut i8, 0 as i32)
                    as i32;
                let ref mut fresh22 = *subs.offset(i as isize);
                *fresh22 = make_str_node(
                    *pstr.offset((cnt + 1 as i32) as isize),
                    sub_len as size_t,
                    0 as i32,
                );
                cnt += 2 as i32;
                i += 1;
                i;
            }
        }
    }
    l = add_item(list, type_0, symbol, 0 as *mut i8);
    if type_0 == argtype::D_subscript as i32 {
        (*l).num_subs = sub_cnt;
        (*l).subs = subs;
    }
    (*l).number = num;
    if list == &mut watch_list as *mut list_item {
        initialize_watch_item(l);
        unserialize_commands(*pstr.offset(cnt as isize), *pstr_len.offset(cnt as isize));
        cnt += 1;
        cnt;
        if field_cnt > cnt {
            let mut expr: *mut i8 = 0 as *mut i8;
            expr = estrdup(
                *pstr.offset(cnt as isize),
                *pstr_len.offset(cnt as isize) as size_t,
            );
            if parse_condition(argtype::D_watch as i32, (*l).number, expr) != 0 as i32 {
                pma_free(expr as *mut libc::c_void);
            }
        }
        if num > (*list).number {
            (*list).number = num;
        }
    } else {
        (*list).number = num;
    }
    return l;
}
unsafe extern "C" fn unserialize_breakpoint(
    mut pstr: *mut *mut i8,
    mut pstr_len: *mut i32,
    mut field_cnt: i32,
) -> *mut BREAKPOINT {
    let mut src: *mut i8 = 0 as *mut i8;
    let mut lineno: i32 = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut s: *mut SRCFILE = 0 as *mut SRCFILE;
    src = estrdup(
        *pstr.offset(0 as i32 as isize),
        *pstr_len.offset(0 as i32 as isize) as size_t,
    );
    s = source_find(src);
    pma_free(src as *mut libc::c_void);
    if s.is_null() {
        return 0 as *mut BREAKPOINT;
    }
    src = (*s).src;
    lineno = strtol(*pstr.offset(1 as i32 as isize), 0 as *mut *mut i8, 0 as i32) as i32;
    if lineno <= 0 as i32 || lineno > (*s).srclines {
        return 0 as *mut BREAKPOINT;
    }
    rp = find_rule(src, lineno as i64);
    if rp.is_null()
        || {
            b = set_breakpoint_at(rp, lineno, 1 as i32 != 0);
            b.is_null()
        }
    {
        return 0 as *mut BREAKPOINT;
    }
    (*b).flags = strtol(*pstr.offset(2 as i32 as isize), 0 as *mut *mut i8, 0 as i32)
        as libc::c_short;
    (*b).ignore_count = strtol(
        *pstr.offset(3 as i32 as isize),
        0 as *mut *mut i8,
        0 as i32,
    );
    (*b).hit_count = strtol(
        *pstr.offset(4 as i32 as isize),
        0 as *mut *mut i8,
        0 as i32,
    );
    (*b).number = strtol(*pstr.offset(5 as i32 as isize), 0 as *mut *mut i8, 0 as i32)
        as i32;
    if field_cnt > 6 as i32 {
        unserialize_commands(
            *pstr.offset(6 as i32 as isize),
            *pstr_len.offset(6 as i32 as isize),
        );
    }
    if field_cnt > 7 as i32 {
        let mut expr: *mut i8 = 0 as *mut i8;
        expr = estrdup(
            *pstr.offset(7 as i32 as isize),
            *pstr_len.offset(7 as i32 as isize) as size_t,
        );
        if parse_condition(argtype::D_break as i32, (*b).number, expr) != 0 as i32 {
            pma_free(expr as *mut libc::c_void);
        }
    }
    if (*b).number > watch_list.number {
        watch_list.number = (*b).number;
    }
    return b;
}
unsafe extern "C" fn unserialize_option(
    mut pstr: *mut *mut i8,
    mut pstr_len: *mut i32,
    mut field_cnt: i32,
) -> *mut dbg_option {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if strncmp(
            *pstr.offset(0 as i32 as isize),
            (*opt).name,
            *pstr_len.offset(0 as i32 as isize) as u64,
        ) == 0 as i32
        {
            let mut value: *mut i8 = 0 as *mut i8;
            value = estrdup(
                *pstr.offset(1 as i32 as isize),
                *pstr_len.offset(1 as i32 as isize) as size_t,
            );
            (Some(((*opt).assign).expect("non-null function pointer")))
                .expect("non-null function pointer")(value);
            pma_free(value as *mut libc::c_void);
            return opt as *mut dbg_option;
        }
        opt = opt.offset(1);
        opt;
    }
    return 0 as *mut dbg_option;
}
unsafe extern "C" fn unserialize_list(mut type_0: i32) {
    let mut val: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut r: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    static mut pstr: [*mut i8; 30] = [0 as *const i8 as *mut i8; 30];
    static mut pstr_len: [i32; 30] = [0; 30];
    val = getenv(env_variable[type_0 as usize]);
    if val.is_null() {
        return;
    }
    p = val;
    loop {
        q = strchr(p, '\u{1e}' as i32 as i8 as i32);
        if q.is_null() {
            break;
        }
        let mut field_cnt: i32 = 0 as i32;
        if type_0 == C2RustUnnamed_13::HISTORY as i32 {
            *q = '\0' as i32 as i8;
            *q = '\u{1e}' as i32 as i8;
        } else {
            r = p;
            loop {
                s = strchr(r, '\u{1f}' as i32 as i8 as i32);
                if !(!s.is_null() && s < q) {
                    break;
                }
                pstr[field_cnt as usize] = r;
                pstr_len[field_cnt as usize] = s.offset_from(r) as i64 as i32;
                r = s.offset(1 as i32 as isize);
                field_cnt += 1;
                field_cnt;
                if field_cnt == 30 as i32 {
                    return;
                }
            }
            match type_0 {
                1 => {
                    unserialize_breakpoint(
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                3 => {
                    unserialize_list_item(
                        &mut display_list,
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                2 => {
                    unserialize_list_item(
                        &mut watch_list,
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                5 => {
                    unserialize_option(
                        pstr.as_mut_ptr(),
                        pstr_len.as_mut_ptr(),
                        field_cnt,
                    );
                }
                4 | _ => {}
            }
        }
        p = q.offset(1 as i32 as isize);
    }
    unsetenv(env_variable[type_0 as usize]);
}
unsafe extern "C" fn prompt_yes_no(
    mut mesg: *const i8,
    mut res_true: i8,
    mut res_default: i32,
    mut fp: *mut FILE,
) -> i32 {
    let mut in_str: *mut i8 = 0 as *mut i8;
    let mut ret: i32 = res_default;
    if input_from_tty {
        fprintf(
            fp,
            b"%s\0" as *const u8 as *const i8,
            dcgettext(0 as *const i8, mesg, 5 as i32),
        );
        in_str = read_a_line.expect("non-null function pointer")(0 as *const i8);
        if in_str.is_null() {
            exit(1 as i32);
        }
        ret = (*in_str as i32 == res_true as i32) as i32;
        pma_free(in_str as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn has_break_or_watch_point(
    mut pnum: *mut i32,
    mut any: bool,
) -> i32 {
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut w: *mut list_item = 0 as *mut list_item;
    if any {
        if breakpoints.next != &mut breakpoints as *mut BREAKPOINT {
            b = breakpoints.next;
        }
        if watch_list.next != &mut watch_list as *mut list_item {
            w = watch_list.next;
        }
        if b.is_null() && w.is_null() {
            return 0 as i32;
        }
        if !b.is_null() && w.is_null() {
            *pnum = (*b).number;
            return argtype::D_break as i32;
        }
        if !w.is_null() && b.is_null() {
            *pnum = (*w).number;
            return argtype::D_watch as i32;
        }
        if (*w).number > (*b).number {
            *pnum = (*w).number;
            return argtype::D_watch as i32;
        }
        *pnum = (*b).number;
        return argtype::D_break as i32;
    }
    b = breakpoints.next;
    while b != &mut breakpoints as *mut BREAKPOINT {
        if (*b).number == *pnum {
            return argtype::D_break as i32;
        }
        b = (*b).next;
    }
    w = watch_list.next;
    while w != &mut watch_list as *mut list_item {
        if (*w).number == *pnum {
            return argtype::D_watch as i32;
        }
        w = (*w).next;
    }
    return 0 as i32;
}
unsafe extern "C" fn delete_commands_item(mut c: *mut commands_item) {
    pma_free((*c).cmd_string as *mut libc::c_void);
    free_cmdarg((*c).arg);
    (*(*c).next).prev = (*c).prev;
    (*(*c).prev).next = (*c).next;
    pma_free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn do_commands(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    static mut b: *mut BREAKPOINT = 0 as *const BREAKPOINT as *mut BREAKPOINT;
    static mut w: *mut list_item = 0 as *const list_item as *mut list_item;
    static mut commands: *mut commands_item = 0 as *const commands_item
        as *mut commands_item;
    let mut c: *mut commands_item = 0 as *mut commands_item;
    if cmd == argtype::D_commands as i32 {
        let mut num: i32 = -(1 as i32);
        let mut type_0: i32 = 0;
        if arg.is_null() {
            type_0 = has_break_or_watch_point(&mut num, 1 as i32 != 0);
        } else {
            num = (*arg).value.lval as i32;
            type_0 = has_break_or_watch_point(&mut num, 0 as i32 != 0);
        }
        b = 0 as *mut BREAKPOINT;
        w = 0 as *mut list_item;
        if type_0 == argtype::D_break as i32 {
            b = find_breakpoint(num as i64);
        } else if type_0 == argtype::D_watch as i32 {
            w = find_item(&mut watch_list, num as i64);
        }
        commands = if !b.is_null() { &mut (*b).commands } else { &mut (*w).commands };
        c = (*commands).next;
        while c != commands {
            c = (*c).prev;
            delete_commands_item((*c).next);
            c = (*c).next;
        }
        return 0 as i32;
    } else if cmd == argtype::D_end as i32 {
        commands = 0 as *mut commands_item;
        if read_a_line
            == Some(read_commands_string as unsafe extern "C" fn(*const i8) -> *mut i8)
        {
            return 1 as i32;
        }
        return 0 as i32;
    } else if cmd == argtype::D_silent as i32 {
        if !b.is_null() {
            (*b).silent = 1 as i32 != 0;
        } else if !w.is_null() {
            (*w).silent = 1 as i32;
        }
    }
    c = emalloc_real(
        ::core::mem::size_of::<commands_item>() as u64,
        b"do_commands\0" as *const u8 as *const i8,
        b"c\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        5054 as i32,
    ) as *mut commands_item;
    (*c).next = 0 as *mut commands_item;
    (*c).cmd = cmd;
    (*c).cmd_string = (*arg).value.sval;
    (*c).arg = (*arg).next;
    pma_free(arg as *mut libc::c_void);
    (*c).prev = (*commands).prev;
    (*c).next = commands;
    (*commands).prev = c;
    (*(*c).prev).next = c;
    return 0 as i32;
}
unsafe extern "C" fn execute_commands(mut commands: *mut commands_item) -> i32 {
    let mut c: *mut commands_item = 0 as *mut commands_item;
    let mut cmd_ptr: Func_cmd = None;
    let mut ret: bool = 0 as i32 != 0;
    c = (*commands).next;
    while c != commands {
        if !((*c).cmd == argtype::D_silent as i32) {
            cmd_ptr = get_command((*c).cmd);
            ret = (Some(cmd_ptr.expect("non-null function pointer")))
                .expect("non-null function pointer")((*c).arg, (*c).cmd) != 0;
            if ret {
                break;
            }
        }
        c = (*c).next;
    }
    return ret as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_print_f(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut current_block: u64;
    let mut count: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut fatal_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    a = arg;
    while !a.is_null() {
        count += 1;
        count;
        a = (*a).next;
    }
    tmp = emalloc_real(
        (count as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"do_print_f\0" as *const u8 as *const i8,
        b"tmp\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        5110 as i32,
    ) as *mut *mut NODE;
    i = 0 as i32;
    a = arg;
    's_33: loop {
        if a.is_null() {
            current_block = 1847472278776910194;
            break;
        }
        match (*a).type_0 as u32 {
            45 => {
                name = (*a).value.sval;
                r = find_symbol(name, 0 as *mut *mut i8);
                if r.is_null() {
                    current_block = 14730812944210663008;
                    break;
                }
                if (*r).type_0 as u32 == nodevals::Node_var_new as i32 as u32
                    || (*r).type_0 as u32 == nodevals::Node_elem_new as i32 as u32
                {
                    let ref mut fresh23 = *tmp.offset(i as isize);
                    *fresh23 = Nnull_string;
                } else if (*r).type_0 as u32 != nodevals::Node_var as i32 as u32 {
                    d_error(
                        dcgettext(
                            0 as *const i8,
                            b"`%s' is not a scalar variable\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        name,
                    );
                    current_block = 14730812944210663008;
                    break;
                } else {
                    let ref mut fresh24 = *tmp.offset(i as isize);
                    *fresh24 = (*r).sub.nodep.l.lptr;
                }
            }
            47 => {
                let mut field_num: i64 = 0;
                r = (*a).value.nodeval;
                field_num = (*r).sub.val.fltnum as i64;
                let ref mut fresh25 = *tmp.offset(i as isize);
                *fresh25 = *get_field(field_num, 0 as *mut Func_ptr);
            }
            49 => {
                let mut cnt: i32 = (*a).a_count;
                name = (*a).value.sval;
                r = find_array(name);
                if r.is_null() {
                    current_block = 14730812944210663008;
                    break;
                }
                while cnt > 0 as i32 {
                    let mut value: *mut NODE = 0 as *mut NODE;
                    let mut subs: *mut NODE = 0 as *mut NODE;
                    a = (*a).next;
                    subs = (*a).value.nodeval;
                    value = in_array(r, subs);
                    if cnt == 1 as i32 {
                        if value.is_null() {
                            let ref mut fresh26 = *tmp.offset(i as isize);
                            *fresh26 = Nnull_string;
                        } else if (*value).type_0 as u32
                            == nodevals::Node_var_array as i32 as u32
                        {
                            d_error(
                                dcgettext(
                                    0 as *const i8,
                                    b"attempt to use array `%s[\"%.*s\"]' in a scalar context\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                name,
                                (*subs).sub.val.slen as i32,
                                (*subs).sub.val.sp,
                            );
                            current_block = 14730812944210663008;
                            break 's_33;
                        } else {
                            let ref mut fresh27 = *tmp.offset(i as isize);
                            *fresh27 = value;
                        }
                    } else if value.is_null() {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"[\"%.*s\"] not in array `%s'\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            (*subs).sub.val.slen as i32,
                            (*subs).sub.val.sp,
                            name,
                        );
                        current_block = 14730812944210663008;
                        break 's_33;
                    } else if (*value).type_0 as u32
                        != nodevals::Node_var_array as i32 as u32
                    {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"attempt to use scalar `%s[\"%.*s\"]' as array\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                            name,
                            (*subs).sub.val.slen as i32,
                            (*subs).sub.val.sp,
                        );
                        current_block = 14730812944210663008;
                        break 's_33;
                    } else {
                        r = value;
                        name = (*r).sub.nodep.name;
                    }
                    cnt -= 1;
                    cnt;
                }
            }
            46 => {
                let ref mut fresh28 = *tmp.offset(i as isize);
                *fresh28 = (*a).value.nodeval;
            }
            _ => {}
        }
        i += 1;
        i;
        a = (*a).next;
    }
    match current_block {
        1847472278776910194 => {
            let ref mut fresh29 = *tmp.offset(0 as i32 as isize);
            *fresh29 = force_string_fmt(
                *tmp.offset(0 as i32 as isize),
                CONVFMT,
                CONVFMTidx,
            );
            let fresh30 = fatal_tag_valid;
            fatal_tag_valid = fatal_tag_valid + 1;
            if fresh30 != 0 {
                memcpy(
                    fatal_tag_stack.as_mut_ptr() as *mut i8 as *mut libc::c_void,
                    fatal_tag.as_mut_ptr() as *const i8 as *const libc::c_void,
                    ::core::mem::size_of::<jmp_buf>() as u64,
                );
            }
            if _setjmp(fatal_tag.as_mut_ptr()) == 0 as i32 {
                r = format_tree(
                    (**tmp.offset(0 as i32 as isize)).sub.val.sp,
                    (**tmp.offset(0 as i32 as isize)).sub.val.slen,
                    tmp,
                    i as i64,
                );
            } else {
                exit_val = 0 as i32;
                r = 0 as *mut NODE;
            }
            fatal_tag_valid -= 1;
            if fatal_tag_valid != 0 {
                memcpy(
                    fatal_tag.as_mut_ptr() as *mut i8 as *mut libc::c_void,
                    fatal_tag_stack.as_mut_ptr() as *const i8 as *const libc::c_void,
                    ::core::mem::size_of::<jmp_buf>() as u64,
                );
            }
            if !r.is_null() {
                if 0 != 0 && 0 != 0
                    && (::core::mem::size_of::<i8>() as u64)
                        .wrapping_mul((*r).sub.val.slen) <= 8 as i32 as u64
                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                {
                    ({
                        let mut __ptr: *const i8 = (*r).sub.val.sp as *const i8;
                        let mut __stream: *mut FILE = out_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::core::mem::size_of::<i8>() as u64)
                            .wrapping_mul((*r).sub.val.slen);
                        while __cnt > 0 as i32 as u64 {
                            if (if ((*__stream)._IO_write_ptr
                                >= (*__stream)._IO_write_end) as i32 as i64 != 0
                            {
                                let fresh31 = __ptr;
                                __ptr = __ptr.offset(1);
                                __overflow(__stream, *fresh31 as u8 as i32)
                            } else {
                                let fresh32 = __ptr;
                                __ptr = __ptr.offset(1);
                                let fresh33 = (*__stream)._IO_write_ptr;
                                (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                    .offset(1);
                                *fresh33 = *fresh32;
                                *fresh33 as u8 as i32
                            }) == -(1 as i32)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        compile_error!("Binary expression is not supposed to be used")
                    });
                } else {
                    if 0 != 0 && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                        || 0 != 0 && (*r).sub.val.slen == 0 as i32 as u64
                    {} else {
                        fwrite_unlocked(
                            (*r).sub.val.sp as *const libc::c_void,
                            ::core::mem::size_of::<i8>() as u64,
                            (*r).sub.val.slen,
                            out_fp,
                        );
                    };
                };
                compile_error!("Conditional expression is not supposed to be used");
                unref(r);
            }
        }
        _ => {}
    }
    pma_free(tmp as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_source(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut fd: i32 = 0;
    let mut file: *mut i8 = (*arg).value.sval;
    fd = open_readfd(file);
    if fd <= -(1 as i32) {
        d_error(
            dcgettext(
                0 as *const i8,
                b"cannot open source file `%s' for reading: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            file,
            strerror(*__errno_location()),
        );
        return 0 as i32;
    }
    push_cmd_src(
        fd,
        0 as i32 != 0,
        Some(g_readline as unsafe extern "C" fn(*const i8) -> *mut i8),
        Some(close as unsafe extern "C" fn(i32) -> i32),
        argtype::D_source as i32,
        0 as i32,
    );
    (*cmd_src).str_0 = estrdup(file, strlen(file));
    return 0 as i32;
}
unsafe extern "C" fn open_readfd(mut file: *const i8) -> i32 {
    let mut fd: i32 = 0;
    fd = open(file, 0 as i32);
    if fd <= -(1 as i32) {
        return -(1 as i32)
    } else if os_isdir(fd) != 0 {
        close(fd);
        *__errno_location() = 21 as i32;
        return -(1 as i32);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn find_option(mut name: *mut i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut idx: i32 = 0;
    idx = 0 as i32;
    loop {
        p = option_list[idx as usize].name;
        if p.is_null() {
            break;
        }
        if strcmp(p, name) == 0 as i32 {
            return idx;
        }
        idx += 1;
        idx;
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn option_help() {
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        fprintf(
            out_fp,
            b"\t%-15.15s - %s\n\0" as *const u8 as *const i8,
            (*opt).name,
            dcgettext(0 as *const i8, (*opt).help_txt, 5 as i32),
        );
        opt = opt.offset(1);
        opt;
    }
}
unsafe extern "C" fn set_gawk_output(mut file: *const i8) {
    let mut fd: i32 = -(1 as i32);
    let mut fp: *mut FILE = 0 as *mut FILE;
    if output_fp != stdout {
        if output_fp != stderr {
            fclose(output_fp);
            pma_free(output_file as *mut libc::c_void);
        }
        output_fp = stdout;
        output_is_tty = os_isatty(fileno(stdout)) != 0;
        output_file = b"/dev/stdout\0" as *const u8 as *const i8;
    }
    if file.is_null() || *file.offset(0 as i32 as isize) as i32 == '\0' as i32 {
        return;
    }
    *__errno_location() = 0 as i32;
    fd = os_devopen(file, 0o1 as i32);
    if fd != -(1 as i32) {
        fp = fdopen(fd, b"w\0" as *const u8 as *const i8);
        if fp.is_null() {
            close(fd);
        }
    } else if strncmp(file, b"/dev/\0" as *const u8 as *const i8, 5 as i32 as u64)
        == 0 as i32
    {
        let mut cp: *mut i8 = (file as *mut i8).offset(5 as i32 as isize);
        if strcmp(cp, b"stdout\0" as *const u8 as *const i8) == 0 as i32 {
            return;
        }
        if strcmp(cp, b"stderr\0" as *const u8 as *const i8) == 0 as i32 {
            output_fp = stderr;
            output_file = b"/dev/stderr\0" as *const u8 as *const i8;
            output_is_tty = os_isatty(fileno(stderr)) != 0;
            return;
        }
        if strncmp(cp, b"fd/\0" as *const u8 as *const i8, 3 as i32 as u64) == 0 as i32 {
            cp = cp.offset(3 as i32 as isize);
            fd = strtoul(cp, 0 as *mut *mut i8, 10 as i32) as i32;
            if *__errno_location() == 0 as i32 && fd > -(1 as i32) {
                fp = fdopen(fd, b"w\0" as *const u8 as *const i8);
                if fp.is_null() {
                    fd = -(1 as i32);
                }
            } else {
                fd = -(1 as i32);
            }
        } else {
            fd = open(file, 0o1 as i32);
        }
        if fd > -(1 as i32) && fp.is_null() {
            fp = fdopen(fd, b"w\0" as *const u8 as *const i8);
            if fp.is_null() {
                close(fd);
            }
        }
    } else {
        fp = fopen(file, b"w\0" as *const u8 as *const i8);
    }
    if !fp.is_null() {
        output_fp = fp;
        output_file = estrdup(file, strlen(file));
        setbuf(fp, 0 as *mut libc::c_void as *mut i8);
        output_is_tty = os_isatty(fileno(fp)) != 0;
    } else {
        d_error(
            dcgettext(
                0 as *const i8,
                b"could not open `%s' for writing: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file,
            if *__errno_location() != 0 as i32 {
                strerror(*__errno_location())
            } else {
                dcgettext(
                    0 as *const i8,
                    b"reason unknown\0" as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
        fprintf(
            out_fp,
            dcgettext(
                0 as *const i8,
                b"sending output to stdout\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    };
}
unsafe extern "C" fn set_prompt(mut value: *const i8) {
    pma_free(dgawk_prompt as *mut libc::c_void);
    dgawk_prompt = estrdup(value, strlen(value));
    dbg_prompt = dgawk_prompt;
}
unsafe extern "C" fn set_option_flag(mut value: *const i8) -> i32 {
    let mut n: i64 = 0;
    if strcmp(value, b"on\0" as *const u8 as *const i8) == 0 as i32 {
        return 1 as i32;
    }
    if strcmp(value, b"off\0" as *const u8 as *const i8) == 0 as i32 {
        return 0 as i32;
    }
    *__errno_location() = 0 as i32;
    n = strtol(value, 0 as *mut *mut i8, 0 as i32);
    return (*__errno_location() == 0 as i32 && n != 0 as i32 as i64) as i32;
}
unsafe extern "C" fn set_option_num(mut pnum: *mut i32, mut value: *const i8) {
    let mut n: i64 = 0;
    *__errno_location() = 0 as i32;
    n = strtol(value, 0 as *mut *mut i8, 0 as i32);
    if *__errno_location() == 0 as i32 && n > 0 as i32 as i64 {
        *pnum = n as i32;
    } else {
        d_error(
            dcgettext(
                0 as *const i8,
                b"invalid number\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    };
}
unsafe extern "C" fn set_listsize(mut value: *const i8) {
    set_option_num(&mut list_size, value);
}
unsafe extern "C" fn set_trace(mut value: *const i8) {
    do_trace = set_option_flag(value);
}
unsafe extern "C" fn set_save_history(mut value: *const i8) {
    do_save_history = set_option_flag(value);
}
unsafe extern "C" fn set_save_options(mut value: *const i8) {
    do_save_options = set_option_flag(value);
}
unsafe extern "C" fn set_history_size(mut value: *const i8) {
    set_option_num(&mut history_size, value);
}
#[no_mangle]
pub unsafe extern "C" fn read_commands_string(mut prompt: *const i8) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut line: *mut i8 = 0 as *mut i8;
    if commands_string.is_null() {
        return 0 as *mut i8;
    }
    p = commands_string as *mut i8;
    end = (commands_string as *mut i8).offset(commands_string_len as isize);
    while p < end {
        if *p as i32 == line_sep as i32 {
            line = estrdup(
                commands_string,
                p.offset_from(commands_string) as i64 as size_t,
            );
            commands_string = p.offset(1 as i32 as isize);
            commands_string_len = end.offset_from(commands_string) as i64 as i32;
            return line;
        }
        p = p.offset(1);
        p;
    }
    line = estrdup(commands_string, commands_string_len as size_t);
    commands_string = 0 as *const i8;
    commands_string_len = 0 as i32;
    return line;
}
unsafe extern "C" fn save_options(mut file: *const i8) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut opt: *const dbg_option = 0 as *const dbg_option;
    fp = fopen(file, b"w\0" as *const u8 as *const i8);
    if fp.is_null() {
        return;
    }
    opt = option_list.as_ptr();
    while !((*opt).name).is_null() {
        if !((*opt).str_val).is_null() {
            fprintf(
                fp,
                b"option %s = \"%s\"\n\0" as *const u8 as *const i8,
                (*opt).name,
                *(*opt).str_val,
            );
        } else {
            fprintf(
                fp,
                b"option %s = %d\n\0" as *const u8 as *const i8,
                (*opt).name,
                *(*opt).num_val,
            );
        }
        opt = opt.offset(1);
        opt;
    }
    fclose(fp);
    chmod(file, 0o600 as i32 as __mode_t);
}
unsafe extern "C" fn close_all() {
    let mut stdio_problem: bool = false;
    let mut got_EPIPE: bool = false;
    let mut cs: *mut command_source = 0 as *mut command_source;
    nextfile(&mut curfile, 1 as i32 != 0);
    close_io(&mut stdio_problem, &mut got_EPIPE);
    if (*cur_srcfile).fd != -(1 as i32) {
        close((*cur_srcfile).fd);
        (*cur_srcfile).fd = -(1 as i32);
    }
    cs = cmd_src;
    while !cs.is_null() {
        if ((*cs).close_func).is_some() && (*cs).fd != -(1 as i32) {
            ((*cs).close_func).expect("non-null function pointer")((*cs).fd);
            (*cs).fd = -(1 as i32);
        }
        cs = (*cs).next;
    }
    close_extensions();
    set_gawk_output(0 as *const i8);
}
unsafe extern "C" fn pre_execute_code(mut pi: *mut *mut INSTRUCTION) -> i32 {
    let mut ei: *mut INSTRUCTION = *pi;
    match (*ei).opcode as u32 {
        60 | 59 | 67 | 66 => {
            d_error(
                dcgettext(
                    0 as *const i8,
                    b"`%s' not allowed in current context; statement ignored\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                op2str((*ei).opcode),
            );
            *pi = (*ei).nexti;
        }
        62 => {
            if !((*ei).nexti).is_null() {
                let mut r: *mut NODE = 0 as *mut NODE;
                d_error(
                    dcgettext(
                        0 as *const i8,
                        b"`return' not allowed in current context; statement ignored\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                r = POP_SCALAR();
                DEREF(r);
                *pi = (*ei).nexti;
            }
        }
        _ => {}
    }
    return (ei == *pi) as i32;
}
unsafe extern "C" fn execute_code(mut code: *mut INSTRUCTION) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut fatal_tag_stack: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut save_flags: i32 = do_flags as i32;
    do_flags = do_flag_values::DO_FLAG_NONE;
    let fresh34 = fatal_tag_valid;
    fatal_tag_valid = fatal_tag_valid + 1;
    if fresh34 != 0 {
        memcpy(
            fatal_tag_stack.as_mut_ptr() as *mut i8 as *mut libc::c_void,
            fatal_tag.as_mut_ptr() as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<jmp_buf>() as u64,
        );
    }
    if _setjmp(fatal_tag.as_mut_ptr()) == 0 as i32 {
        interpret.expect("non-null function pointer")(code as *mut INSTRUCTION);
        r = POP_SCALAR() as *mut NODE;
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"fatal error during eval, need to restart.\n\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        restart(0 as i32 != 0);
    }
    fatal_tag_valid -= 1;
    if fatal_tag_valid != 0 {
        memcpy(
            fatal_tag.as_mut_ptr() as *mut i8 as *mut libc::c_void,
            fatal_tag_stack.as_mut_ptr() as *const i8 as *const libc::c_void,
            ::core::mem::size_of::<jmp_buf>() as u64,
        );
    }
    do_flags = do_flag_values::from_libc_c_uint(save_flags as u32);
    if exit_val != 0 as i32 {
        exit_val = 0 as i32;
        return 0 as *mut NODE;
    }
    return r as *mut NODE;
}
#[no_mangle]
pub unsafe extern "C" fn do_eval(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret_val: *mut NODE = 0 as *mut NODE;
    let mut f: *mut NODE = 0 as *mut NODE;
    let mut this_frame: *mut NODE = 0 as *mut NODE;
    let mut this_func: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut eval: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    let mut ecount: i32 = 0 as i32;
    let mut pcount: i32 = 0 as i32;
    let mut ret: i32 = 0;
    let mut save_flags: i32 = do_flags as i32;
    let mut the_source: *mut SRCFILE = 0 as *mut SRCFILE;
    if prog_running {
        this_frame = find_frame(0 as i32 as i64);
        this_func = (*this_frame).sub.nodep.x.extra;
    }
    install_params(this_func);
    ctxt = new_context();
    (*ctxt).install_func = Some(append_symbol as unsafe extern "C" fn(*mut NODE) -> ());
    push_context(ctxt);
    the_source = add_srcfile(
        srctype::SRC_CMDLINE,
        (*arg).value.sval,
        srcfiles,
        0 as *mut bool,
        0 as *mut i32,
    );
    do_flags = ::core::mem::transmute::<
        u32,
        do_flag_values,
    >(do_flags as u32 & do_flag_values::DO_MPFR as i32 as u32);
    ret = parse_program(&mut code, 1 as i32 != 0);
    do_flags = do_flag_values::from_libc_c_uint(save_flags as u32);
    remove_params(this_func);
    if ret != 0 as i32 {
        pop_context();
        free_context(ctxt, 0 as i32 != 0);
        let mut s: *mut NODE = make_str_node(
            b"@eval\0" as *const u8 as *const i8,
            5 as i32 as size_t,
            0 as i32,
        );
        ((*(*func_table).sub.nodep.l.lp).remove)
            .expect("non-null function pointer")(func_table, s);
        unref(s);
        return 0 as i32;
    }
    f = lookup(b"@eval\0" as *const u8 as *const i8);
    if this_func.is_null() {
        eval = bcalloc(opcodeval::Op_func_call, 2 as i32, 0 as i32);
        (*eval).d.name = (*cur_srcfile).src;
        (*eval).x.xn = f;
        (*eval).d.name = 0 as *mut i8;
        (*eval.offset(1 as i32 as isize)).x.xl = 0 as i32 as i64;
        (*eval).nexti = bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32);
    } else {
        let mut i: i32 = 0;
        let mut t: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        eval = (*f).sub.nodep.r.iptr;
        (*eval).d.name = (*cur_srcfile).src;
        t = (*eval.offset(1 as i32 as isize)).d.di;
        (*t).opcode = opcodeval::Op_stop;
        ecount = (*f).sub.nodep.l.ll as i32;
        pcount = (*this_func).sub.nodep.l.ll as i32;
        if ecount > 0 as i32 {
            if pcount == 0 as i32 {
                (*this_frame).sub.nodep.r.av = emalloc_real(
                    (ecount as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                    b"do_eval\0" as *const u8 as *const i8,
                    b"this_frame->stack\0" as *const u8 as *const i8,
                    b"debug.c\0" as *const u8 as *const i8,
                    5679 as i32,
                ) as *mut *mut NODE;
            } else {
                (*this_frame).sub.nodep.r.av = erealloc_real(
                    (*this_frame).sub.nodep.r.av as *mut libc::c_void,
                    ((pcount + ecount) as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                    b"do_eval\0" as *const u8 as *const i8,
                    b"this_frame->stack\0" as *const u8 as *const i8,
                    b"debug.c\0" as *const u8 as *const i8,
                    5681 as i32,
                ) as *mut *mut NODE;
            }
            sp = ((*this_frame).sub.nodep.r.av).offset(pcount as isize);
            i = 0 as i32;
            while i < ecount {
                let mut np: *mut NODE = 0 as *mut NODE;
                np = ((*f).sub.nodep.rn).offset(i as isize);
                (*np).sub.nodep.l.ll += pcount as i64;
                r = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
                if !r.is_null() {
                    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(r
                        as *mut block_item))
                        .freep;
                } else {
                    r = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
                };
                memset(
                    r as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<NODE>() as u64,
                );
                let fresh35 = sp;
                sp = sp.offset(1);
                *fresh35 = r;
                (*r).type_0 = nodevals::Node_var_new;
                (*r).sub.nodep.name = (*np).sub.nodep.name;
                i += 1;
                i;
            }
            (*this_func).sub.nodep.l.ll += ecount as i64;
        }
    }
    ret_val = execute_code(eval as *mut INSTRUCTION);
    if !ret_val.is_null() {
        DEREF(ret_val);
    }
    if !this_func.is_null() && ecount > 0 as i32 {
        let mut i_0: i32 = 0;
        sp = ((*this_frame).sub.nodep.r.av).offset(pcount as isize);
        i_0 = ecount;
        while i_0 > 0 as i32 {
            r = *sp;
            if (*r).type_0 as u32 == nodevals::Node_var as i32 as u32 {
                DEREF((*r).sub.nodep.l.lptr);
            } else if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
                ((*(*r).sub.nodep.l.lp).clear)
                    .expect("non-null function pointer")(r, 0 as *mut exp_node);
            }
            let ref mut fresh36 = (*(r as *mut block_item)).freep;
            *fresh36 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = r as *mut block_item;
            let fresh37 = sp;
            sp = sp.offset(1);
            *fresh37 = 0 as *mut NODE;
            i_0 -= 1;
            i_0;
        }
        if pcount == 0 as i32 {
            pma_free((*this_frame).sub.nodep.r.av as *mut libc::c_void);
            (*this_frame).sub.nodep.r.av = 0 as *mut *mut exp_node;
        }
        (*this_func).sub.nodep.l.ll -= ecount as i64;
    }
    pop_context();
    free_context(ctxt, !ret_val.is_null());
    if !ret_val.is_null() {
        let mut s_0: *mut NODE = make_str_node(
            b"@eval\0" as *const u8 as *const i8,
            5 as i32 as size_t,
            0 as i32,
        );
        ((*(*func_table).sub.nodep.l.lp).remove)
            .expect("non-null function pointer")(func_table, s_0);
        unref(s_0);
    }
    pma_free((*f).sub.nodep.name as *mut libc::c_void);
    let ref mut fresh38 = (*(f as *mut block_item)).freep;
    *fresh38 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
    nextfree[block_id::BLOCK_NODE as i32 as usize].freep = f as *mut block_item;
    free_srcfile(the_source);
    return 0 as i32;
}
static mut invalid_symbol: i32 = 0 as i32;
unsafe extern "C" fn check_symbol(mut r: *mut NODE) {
    invalid_symbol += 1;
    invalid_symbol;
    d_error(
        dcgettext(
            0 as *const i8,
            b"no symbol `%s' in current context\0" as *const u8 as *const i8,
            5 as i32,
        ),
        (*r).sub.nodep.name,
    );
    append_symbol(r);
}
unsafe extern "C" fn parse_condition(
    mut type_0: i32,
    mut num: i32,
    mut expr: *mut i8,
) -> i32 {
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut ctxt: *mut AWK_CONTEXT = 0 as *mut AWK_CONTEXT;
    let mut ret: i32 = 0;
    let mut b: *mut BREAKPOINT = 0 as *mut BREAKPOINT;
    let mut w: *mut list_item = 0 as *mut list_item;
    let mut this_func: *mut NODE = 0 as *mut NODE;
    let mut it: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut stop_0: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut rule: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    let mut cndn: *mut condition = 0 as *mut condition;
    let mut save_flags: i32 = do_flags as i32;
    if type_0 == argtype::D_break as i32
        && {
            b = find_breakpoint(num as i64);
            !b.is_null()
        }
    {
        let mut rp: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
        cndn = &mut (*b).cndn;
        rp = find_rule((*b).src, (*(*b).bpi).source_line as i64);
        if !rp.is_null() && (*rp).opcode as u32 == opcodeval::Op_func as i32 as u32 {
            this_func = (*rp).x.xn;
        }
    } else if type_0 == argtype::D_watch as i32
        && {
            w = find_item(&mut watch_list, num as i64);
            !w.is_null()
        }
    {
        cndn = &mut (*w).cndn;
        this_func = (*find_frame(cur_frame)).sub.nodep.x.extra;
    }
    if cndn.is_null() {
        return -(1 as i32);
    }
    if !expr.is_null() {
        install_params(this_func);
        ctxt = new_context();
        invalid_symbol = 0 as i32;
        (*ctxt).install_func = Some(
            check_symbol as unsafe extern "C" fn(*mut NODE) -> (),
        );
        push_context(ctxt);
        add_srcfile(srctype::SRC_CMDLINE, expr, srcfiles, 0 as *mut bool, 0 as *mut i32);
        do_flags = do_flag_values::DO_FLAG_NONE;
        ret = parse_program(&mut code, 1 as i32 != 0);
        do_flags = do_flag_values::from_libc_c_uint(save_flags as u32);
        remove_params(this_func);
        pop_context();
        if ret != 0 as i32 || invalid_symbol != 0 {
            free_context(ctxt, 0 as i32 != 0);
            return -(1 as i32);
        }
        rule = (*ctxt).rule_list.nexti;
        stop_0 = bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32);
        it = (*rule).x.xi;
        (*it).opcode = opcodeval::Op_push_i;
        (*it).d.dn = make_number.expect("non-null function pointer")(1.0f64);
        (*it).nexti = bcalloc(opcodeval::Op_jmp, 1 as i32, 0 as i32);
        (*(*it).nexti).d.di = stop_0;
        (*(*it).nexti).nexti = (*rule).d.di;
        it = (*rule).d.di;
        (*it).opcode = opcodeval::Op_push_i;
        (*it).d.dn = make_number.expect("non-null function pointer")(0.0f64);
        (*it).nexti = stop_0;
    }
    if !((*cndn).expr).is_null() {
        pma_free((*cndn).expr as *mut libc::c_void);
    }
    free_context((*cndn).ctxt, 0 as i32 != 0);
    (*cndn).code = code;
    (*cndn).expr = expr;
    (*cndn).ctxt = ctxt;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn do_condition(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut type_0: i32 = 0;
    let mut num: i32 = 0;
    let mut expr: *mut i8 = 0 as *mut i8;
    num = (*arg).value.lval as i32;
    type_0 = has_break_or_watch_point(&mut num, 0 as i32 != 0);
    if type_0 == 0 {
        return 0 as i32;
    }
    arg = (*arg).next;
    if !arg.is_null() {
        expr = (*arg).value.sval;
    }
    if parse_condition(type_0, num, expr) == 0 as i32 && !arg.is_null() {
        (*arg).value.sval = 0 as *mut i8;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn in_cmd_src(mut filename: *const i8) -> i32 {
    let mut cs: *mut command_source = 0 as *mut command_source;
    cs = cmd_src;
    while !cs.is_null() {
        if !((*cs).str_0).is_null() && strcmp((*cs).str_0, filename) == 0 as i32 {
            return 1 as i32;
        }
        cs = (*cs).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_eof_status() -> i32 {
    if cmd_src.is_null() {
        return 2 as i32;
    }
    return (*cmd_src).eof_status;
}
#[no_mangle]
pub unsafe extern "C" fn push_cmd_src(
    mut fd: i32,
    mut istty: bool,
    mut readfunc: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    mut closefunc: Option<unsafe extern "C" fn(i32) -> i32>,
    mut ctype: i32,
    mut eofstatus: i32,
) {
    let mut cs: *mut command_source = 0 as *mut command_source;
    cs = emalloc_real(
        ::core::mem::size_of::<command_source>() as u64,
        b"push_cmd_src\0" as *const u8 as *const i8,
        b"cs\0" as *const u8 as *const i8,
        b"debug.c\0" as *const u8 as *const i8,
        5923 as i32,
    ) as *mut command_source;
    (*cs).fd = fd;
    (*cs).is_tty = istty as i32;
    (*cs).read_func = readfunc;
    (*cs).close_func = closefunc;
    (*cs).cmd = ctype;
    (*cs).eof_status = eofstatus;
    (*cs).str_0 = 0 as *mut i8;
    (*cs).next = cmd_src;
    cmd_src = cs;
    input_fd = fd;
    input_from_tty = istty;
    read_a_line = readfunc;
}
#[no_mangle]
pub unsafe extern "C" fn pop_cmd_src() -> i32 {
    let mut cs: *mut command_source = 0 as *mut command_source;
    if ((*cmd_src).next).is_null() {
        return -(1 as i32);
    }
    cs = cmd_src;
    cmd_src = (*cs).next;
    if ((*cs).close_func).is_some() && (*cs).fd != -(1 as i32) {
        ((*cs).close_func).expect("non-null function pointer")((*cs).fd);
    }
    if !((*cs).str_0).is_null() {
        pma_free((*cs).str_0 as *mut libc::c_void);
    }
    pma_free(cs as *mut libc::c_void);
    input_fd = (*cmd_src).fd;
    input_from_tty = (*cmd_src).is_tty != 0;
    read_a_line = (*cmd_src).read_func;
    return 0 as i32;
}