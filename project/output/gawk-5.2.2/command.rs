use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type break_point;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut SUBSEP_node: *mut NODE;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut nextfree: [block_header; 2];
    static mut exit_val: i32;
    fn r_unref(tmp: *mut NODE);
    fn negate_num(n: *mut NODE);
    fn is_alpha(c: i32) -> bool;
    fn is_letter(c: i32) -> bool;
    fn is_identchar(c: i32) -> bool;
    fn estrdup(str: *const i8, len: size_t) -> *mut i8;
    fn set_loc(file: *const i8, line: i32);
    fn r_fatal(mesg: *const i8, _: ...);
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn lookup(name: *const i8) -> *mut NODE;
    fn gprintf(fp: *mut FILE, format: *const i8, _: ...) -> i32;
    static mut pager_quit_tag: jmp_buf;
    static mut input_from_tty: bool;
    static mut out_fp: *mut FILE;
    static mut dbg_prompt: *const i8;
    static mut commands_prompt: *const i8;
    static mut eval_prompt: *const i8;
    static mut dgawk_prompt: *const i8;
    fn d_error(mesg: *const i8, _: ...);
    fn find_option(name: *mut i8) -> i32;
    fn option_help();
    static mut read_a_line: Option<unsafe extern "C" fn(*const i8) -> *mut i8>;
    fn read_commands_string(prompt: *const i8) -> *mut i8;
    fn in_cmd_src(_: *const i8) -> i32;
    fn get_eof_status() -> i32;
    fn pop_cmd_src() -> i32;
    fn has_break_or_watch_point(pnum: *mut i32, any: bool) -> i32;
    fn do_list(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_info(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_print_var(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_backtrace(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_tmp_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_delete_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_enable_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_disable_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_ignore_breakpoint(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_run(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_quit(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_continue(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_step(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_stepi(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_next(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_nexti(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_clear(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_finish(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_up(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_down(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_frame(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_until(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_set_var(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_return(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_display(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_undisplay(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_watch(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_unwatch(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_dump_instructions(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_trace_instruction(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_option(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_commands(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_print_f(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_source(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_eval(arg: *mut CMDARG, cmd: i32) -> i32;
    fn do_condition(arg: *mut CMDARG, cmd: i32) -> i32;
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
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
    pub sub: C2RustUnnamed_1,
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
pub union C2RustUnnamed_1 {
    pub nodep: C2RustUnnamed_3,
    pub val: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub l: C2RustUnnamed_10,
    pub r: C2RustUnnamed_5,
    pub x: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: i64,
    pub cmnt: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub x: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_argument {
    pub next: *mut cmd_argument,
    pub type_0: argtype,
    pub value: C2RustUnnamed_11,
    pub a_count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub lval: i64,
    pub sval: *mut i8,
    pub nodeval: *mut NODE,
}
pub type CMDARG = cmd_argument;
pub type Func_cmd = Option<unsafe extern "C" fn(*mut CMDARG, i32) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdtoken {
    pub name: *const i8,
    pub abbrvn: *const i8,
    pub type_0: argtype,
    pub lex_class: i32,
    pub cf_ptr: Func_cmd,
    pub help_txt: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argtoken {
    pub name: *const i8,
    pub cmd: argtype,
    pub value: nametypeval,
}
pub type yy_state_t = yytype_uint8;
pub type yytype_uint8 = u8;
pub type yysymbol_kind_t = i32;
pub const YYSYMBOL_nls: yysymbol_kind_t = 113;
pub const YYSYMBOL_integer: yysymbol_kind_t = 112;
pub const YYSYMBOL_plus_integer: yysymbol_kind_t = 111;
pub const YYSYMBOL_opt_integer: yysymbol_kind_t = 110;
pub const YYSYMBOL_opt_plus_integer: yysymbol_kind_t = 109;
pub const YYSYMBOL_node: yysymbol_kind_t = 108;
pub const YYSYMBOL_variable: yysymbol_kind_t = 107;
pub const YYSYMBOL_subscript_list: yysymbol_kind_t = 106;
pub const YYSYMBOL_subscript: yysymbol_kind_t = 105;
pub const YYSYMBOL_exp_list: yysymbol_kind_t = 104;
pub const YYSYMBOL_integer_list: yysymbol_kind_t = 103;
pub const YYSYMBOL_opt_integer_list: yysymbol_kind_t = 102;
pub const YYSYMBOL_integer_range: yysymbol_kind_t = 101;
pub const YYSYMBOL_list_args: yysymbol_kind_t = 100;
pub const YYSYMBOL_printf_args: yysymbol_kind_t = 99;
pub const YYSYMBOL_printf_exp: yysymbol_kind_t = 98;
pub const YYSYMBOL_print_args: yysymbol_kind_t = 97;
pub const YYSYMBOL_print_exp: yysymbol_kind_t = 96;
pub const YYSYMBOL_enable_args: yysymbol_kind_t = 95;
pub const YYSYMBOL_help_args: yysymbol_kind_t = 94;
pub const YYSYMBOL_opt_node: yysymbol_kind_t = 93;
pub const YYSYMBOL_opt_string: yysymbol_kind_t = 92;
pub const YYSYMBOL_opt_variable: yysymbol_kind_t = 91;
pub const YYSYMBOL_90_10: yysymbol_kind_t = 90;
pub const YYSYMBOL_89_9: yysymbol_kind_t = 89;
pub const YYSYMBOL_break_args: yysymbol_kind_t = 88;
pub const YYSYMBOL_location: yysymbol_kind_t = 87;
pub const YYSYMBOL_func_name: yysymbol_kind_t = 86;
pub const YYSYMBOL_option_args: yysymbol_kind_t = 85;
pub const YYSYMBOL_string_node: yysymbol_kind_t = 84;
pub const YYSYMBOL_opt_string_node: yysymbol_kind_t = 83;
pub const YYSYMBOL_param_list: yysymbol_kind_t = 82;
pub const YYSYMBOL_opt_param_list: yysymbol_kind_t = 81;
pub const YYSYMBOL_commands_arg: yysymbol_kind_t = 80;
pub const YYSYMBOL_condition_exp: yysymbol_kind_t = 79;
pub const YYSYMBOL_78_8: yysymbol_kind_t = 78;
pub const YYSYMBOL_77_7: yysymbol_kind_t = 77;
pub const YYSYMBOL_76_6: yysymbol_kind_t = 76;
pub const YYSYMBOL_75_5: yysymbol_kind_t = 75;
pub const YYSYMBOL_74_4: yysymbol_kind_t = 74;
pub const YYSYMBOL_73_3: yysymbol_kind_t = 73;
pub const YYSYMBOL_72_2: yysymbol_kind_t = 72;
pub const YYSYMBOL_command: yysymbol_kind_t = 71;
pub const YYSYMBOL_eval_cmd: yysymbol_kind_t = 70;
pub const YYSYMBOL_69_1: yysymbol_kind_t = 69;
pub const YYSYMBOL_statement_list: yysymbol_kind_t = 68;
pub const YYSYMBOL_eval_prologue: yysymbol_kind_t = 67;
pub const YYSYMBOL_set_want_nodeval: yysymbol_kind_t = 66;
pub const YYSYMBOL_break_cmd: yysymbol_kind_t = 65;
pub const YYSYMBOL_frame_cmd: yysymbol_kind_t = 64;
pub const YYSYMBOL_d_cmd: yysymbol_kind_t = 63;
pub const YYSYMBOL_control_cmd: yysymbol_kind_t = 62;
pub const YYSYMBOL_line: yysymbol_kind_t = 61;
pub const YYSYMBOL_input: yysymbol_kind_t = 60;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 59;
pub const YYSYMBOL_58_n_: yysymbol_kind_t = 58;
pub const YYSYMBOL_57_: yysymbol_kind_t = 57;
pub const YYSYMBOL_56_: yysymbol_kind_t = 56;
pub const YYSYMBOL_55_: yysymbol_kind_t = 55;
pub const YYSYMBOL_54_: yysymbol_kind_t = 54;
pub const YYSYMBOL_53_: yysymbol_kind_t = 53;
pub const YYSYMBOL_52_: yysymbol_kind_t = 52;
pub const YYSYMBOL_51_: yysymbol_kind_t = 51;
pub const YYSYMBOL_50_: yysymbol_kind_t = 50;
pub const YYSYMBOL_49_: yysymbol_kind_t = 49;
pub const YYSYMBOL_D_STATEMENT: yysymbol_kind_t = 48;
pub const YYSYMBOL_D_CONDITION: yysymbol_kind_t = 47;
pub const YYSYMBOL_D_EVAL: yysymbol_kind_t = 46;
pub const YYSYMBOL_D_SAVE: yysymbol_kind_t = 45;
pub const YYSYMBOL_D_SOURCE: yysymbol_kind_t = 44;
pub const YYSYMBOL_D_SILENT: yysymbol_kind_t = 43;
pub const YYSYMBOL_D_END: yysymbol_kind_t = 42;
pub const YYSYMBOL_D_COMMANDS: yysymbol_kind_t = 41;
pub const YYSYMBOL_D_OPTION: yysymbol_kind_t = 40;
pub const YYSYMBOL_D_VARIABLE: yysymbol_kind_t = 39;
pub const YYSYMBOL_D_NODE: yysymbol_kind_t = 38;
pub const YYSYMBOL_D_STRING: yysymbol_kind_t = 37;
pub const YYSYMBOL_D_INT: yysymbol_kind_t = 36;
pub const YYSYMBOL_D_TRACE: yysymbol_kind_t = 35;
pub const YYSYMBOL_D_DUMP: yysymbol_kind_t = 34;
pub const YYSYMBOL_D_UNWATCH: yysymbol_kind_t = 33;
pub const YYSYMBOL_D_WATCH: yysymbol_kind_t = 32;
pub const YYSYMBOL_D_UNDISPLAY: yysymbol_kind_t = 31;
pub const YYSYMBOL_D_DISPLAY: yysymbol_kind_t = 30;
pub const YYSYMBOL_D_UNTIL: yysymbol_kind_t = 29;
pub const YYSYMBOL_D_UP: yysymbol_kind_t = 28;
pub const YYSYMBOL_D_TBREAK: yysymbol_kind_t = 27;
pub const YYSYMBOL_D_STEPI: yysymbol_kind_t = 26;
pub const YYSYMBOL_D_STEP: yysymbol_kind_t = 25;
pub const YYSYMBOL_D_SET: yysymbol_kind_t = 24;
pub const YYSYMBOL_D_RUN: yysymbol_kind_t = 23;
pub const YYSYMBOL_D_RETURN: yysymbol_kind_t = 22;
pub const YYSYMBOL_D_QUIT: yysymbol_kind_t = 21;
pub const YYSYMBOL_D_PRINTF: yysymbol_kind_t = 20;
pub const YYSYMBOL_D_PRINT: yysymbol_kind_t = 19;
pub const YYSYMBOL_D_NEXTI: yysymbol_kind_t = 18;
pub const YYSYMBOL_D_NEXT: yysymbol_kind_t = 17;
pub const YYSYMBOL_D_LIST: yysymbol_kind_t = 16;
pub const YYSYMBOL_D_INFO: yysymbol_kind_t = 15;
pub const YYSYMBOL_D_IGNORE: yysymbol_kind_t = 14;
pub const YYSYMBOL_D_HELP: yysymbol_kind_t = 13;
pub const YYSYMBOL_D_FRAME: yysymbol_kind_t = 12;
pub const YYSYMBOL_D_FINISH: yysymbol_kind_t = 11;
pub const YYSYMBOL_D_ENABLE: yysymbol_kind_t = 10;
pub const YYSYMBOL_D_DOWN: yysymbol_kind_t = 9;
pub const YYSYMBOL_D_DISABLE: yysymbol_kind_t = 8;
pub const YYSYMBOL_D_DELETE: yysymbol_kind_t = 7;
pub const YYSYMBOL_D_CONTINUE: yysymbol_kind_t = 6;
pub const YYSYMBOL_D_CLEAR: yysymbol_kind_t = 5;
pub const YYSYMBOL_D_BREAK: yysymbol_kind_t = 4;
pub const YYSYMBOL_D_BACKTRACE: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub type yytype_int8 = libc::c_schar;
pub type yy_state_fast_t = i32;
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: *mut CMDARG,
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
static mut want_nodeval: bool = 0 as i32 != 0;
static mut cmd_idx: i32 = -(1 as i32);
static mut repeat_idx: i32 = -(1 as i32);
static mut arg_list: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
static mut dbg_errcount: i64 = 0 as i32 as i64;
static mut lexptr_begin: *mut i8 = 0 as *const i8 as *mut i8;
static mut in_commands: bool = 0 as i32 != 0;
static mut num_dim: i32 = 0;
static mut in_eval: bool = 0 as i32 != 0;
static mut start_EVAL: [i8; 18] = unsafe {
    *::core::mem::transmute::<&[u8; 18], &[i8; 18]>(b"function @eval(){\0")
};
static mut end_EVAL: [i8; 2] = unsafe {
    *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b"}\0")
};
static mut rl_inhibit_completion: i32 = 0;
unsafe extern "C" fn append_statement(
    mut stmt_list: *mut CMDARG,
    mut stmt: *mut i8,
) -> *mut CMDARG {
    let mut a: *mut CMDARG = 0 as *mut CMDARG;
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut slen: i32 = 0;
    let mut ssize: i32 = 0;
    if stmt == start_EVAL.as_ptr() as *mut i8 {
        len = ::core::mem::size_of::<[i8; 18]>() as u64 as i32;
        a = stmt_list;
        while !a.is_null() {
            len = (len as u64)
                .wrapping_add((strlen((*a).value.sval)).wrapping_add(1 as i32 as u64))
                as i32 as i32;
            a = (*a).next;
        }
        len += 512 as i32;
        s = emalloc_real(
            ((len + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
            b"append_statement\0" as *const u8 as *const i8,
            b"s\0" as *const u8 as *const i8,
            b"command.y\0" as *const u8 as *const i8,
            770 as i32,
        ) as *mut i8;
        arg = mk_cmdarg(argtype::D_string);
        (*arg).value.sval = s;
        (*arg).a_count = len;
        slen = (::core::mem::size_of::<[i8; 16]>() as u64).wrapping_sub(1 as i32 as u64)
            as i32;
        memcpy(
            s as *mut libc::c_void,
            start_EVAL.as_ptr() as *const libc::c_void,
            slen as u64,
        );
        a = stmt_list;
        while !a.is_null() {
            len = strlen((*a).value.sval) as i32;
            memcpy(
                s.offset(slen as isize) as *mut libc::c_void,
                (*a).value.sval as *const libc::c_void,
                len as u64,
            );
            slen += len;
            if !((*a).next).is_null() {
                let fresh0 = slen;
                slen = slen + 1;
                *s.offset(fresh0 as isize) = ',' as i32 as i8;
            }
            a = (*a).next;
        }
        let fresh1 = slen;
        slen = slen + 1;
        *s.offset(fresh1 as isize) = ')' as i32 as i8;
        let fresh2 = slen;
        slen = slen + 1;
        *s.offset(fresh2 as isize) = '{' as i32 as i8;
        *s.offset(slen as isize) = '\0' as i32 as i8;
        return arg;
    }
    len = (strlen(stmt)).wrapping_add(1 as i32 as u64) as i32;
    s = (*stmt_list).value.sval;
    slen = strlen(s) as i32;
    ssize = (*stmt_list).a_count;
    if len > ssize - slen {
        ssize = slen + len + 512 as i32;
        s = erealloc_real(
            s as *mut libc::c_void,
            ((ssize + 1 as i32) as u64)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            b"append_statement\0" as *const u8 as *const i8,
            b"s\0" as *const u8 as *const i8,
            b"command.y\0" as *const u8 as *const i8,
            797 as i32,
        ) as *mut i8;
        (*stmt_list).value.sval = s;
        (*stmt_list).a_count = ssize;
    }
    memcpy(
        s.offset(slen as isize) as *mut libc::c_void,
        stmt as *const libc::c_void,
        len as u64,
    );
    slen += len;
    if slen >= 2 as i32 && *s.offset((slen - 2 as i32) as isize) as i32 != '\n' as i32 {
        *s.offset((slen - 1 as i32) as isize) = '\n' as i32 as i8;
        *s.offset(slen as isize) = '\0' as i32 as i8;
    }
    if stmt == end_EVAL.as_ptr() as *mut i8 {
        (*stmt_list).value.sval = erealloc_real(
            (*stmt_list).value.sval as *mut libc::c_void,
            (slen + 1 as i32) as size_t,
            b"append_statement\0" as *const u8 as *const i8,
            b"stmt_list->a_string\0" as *const u8 as *const i8,
            b"command.y\0" as *const u8 as *const i8,
            809 as i32,
        ) as *mut i8;
    }
    return stmt_list;
}
static mut yytranslate: [yytype_int8; 304] = [
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
    58 as i32 as yytype_int8,
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
    57 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
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
    51 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    49 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    52 as i32 as yytype_int8,
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
    55 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    56 as i32 as yytype_int8,
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
];
#[no_mangle]
pub static mut zz_debug_cmdtab: [cmdtoken; 43] = unsafe {
    [
        {
            let mut init = cmdtoken {
                name: b"backtrace\0" as *const u8 as *const i8,
                abbrvn: b"bt\0" as *const u8 as *const i8,
                type_0: argtype::D_backtrace,
                lex_class: 258 as i32,
                cf_ptr: Some(
                    do_backtrace as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"backtrace [N] - print trace of all or N innermost (outermost if N < 0) frames\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"break\0" as *const u8 as *const i8,
                abbrvn: b"b\0" as *const u8 as *const i8,
                type_0: argtype::D_break,
                lex_class: 259 as i32,
                cf_ptr: Some(
                    do_breakpoint as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"break [[filename:]N|function] - set breakpoint at the specified location\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"clear\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_clear,
                lex_class: 260 as i32,
                cf_ptr: Some(do_clear as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"clear [[filename:]N|function] - delete breakpoints previously set\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"commands\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_commands,
                lex_class: 296 as i32,
                cf_ptr: Some(
                    do_commands as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"commands [num] - starts a list of commands to be executed at a breakpoint(watchpoint) hit\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"condition\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_condition,
                lex_class: 302 as i32,
                cf_ptr: Some(
                    do_condition as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"condition num [expr] - set or clear breakpoint or watchpoint condition\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"continue\0" as *const u8 as *const i8,
                abbrvn: b"c\0" as *const u8 as *const i8,
                type_0: argtype::D_continue,
                lex_class: 261 as i32,
                cf_ptr: Some(
                    do_continue as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"continue [COUNT] - continue program being debugged\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"delete\0" as *const u8 as *const i8,
                abbrvn: b"d\0" as *const u8 as *const i8,
                type_0: argtype::D_delete,
                lex_class: 262 as i32,
                cf_ptr: Some(
                    do_delete_breakpoint as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"delete [breakpoints] [range] - delete specified breakpoints\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"disable\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_disable,
                lex_class: 263 as i32,
                cf_ptr: Some(
                    do_disable_breakpoint
                        as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"disable [breakpoints] [range] - disable specified breakpoints\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"display\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_display,
                lex_class: 285 as i32,
                cf_ptr: Some(
                    do_display as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"display [var] - print value of variable each time the program stops\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"down\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_down,
                lex_class: 264 as i32,
                cf_ptr: Some(do_down as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"down [N] - move N frames down the stack\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"dump\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_dump,
                lex_class: 289 as i32,
                cf_ptr: Some(
                    do_dump_instructions as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"dump [filename] - dump instructions to file or stdout\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"enable\0" as *const u8 as *const i8,
                abbrvn: b"e\0" as *const u8 as *const i8,
                type_0: argtype::D_enable,
                lex_class: 265 as i32,
                cf_ptr: Some(
                    do_enable_breakpoint as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"enable [once|del] [breakpoints] [range] - enable specified breakpoints\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"end\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_end,
                lex_class: 297 as i32,
                cf_ptr: Some(
                    do_commands as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"end - end a list of commands or awk statements\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"eval\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_eval,
                lex_class: 301 as i32,
                cf_ptr: Some(do_eval as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"eval stmt|[p1, p2, ...] - evaluate awk statement(s)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"exit\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_quit,
                lex_class: 276 as i32,
                cf_ptr: Some(do_quit as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"exit - (same as quit) exit debugger\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"finish\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_finish,
                lex_class: 266 as i32,
                cf_ptr: Some(do_finish as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"finish - execute until selected stack frame returns\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"frame\0" as *const u8 as *const i8,
                abbrvn: b"f\0" as *const u8 as *const i8,
                type_0: argtype::D_frame,
                lex_class: 267 as i32,
                cf_ptr: Some(do_frame as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"frame [N] - select and print stack frame number N\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"help\0" as *const u8 as *const i8,
                abbrvn: b"h\0" as *const u8 as *const i8,
                type_0: argtype::D_help,
                lex_class: 268 as i32,
                cf_ptr: Some(do_help as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"help [command] - print list of commands or explanation of command\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"ignore\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_ignore,
                lex_class: 269 as i32,
                cf_ptr: Some(
                    do_ignore_breakpoint as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"ignore N COUNT - set ignore-count of breakpoint number N to COUNT\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"info\0" as *const u8 as *const i8,
                abbrvn: b"i\0" as *const u8 as *const i8,
                type_0: argtype::D_info,
                lex_class: 270 as i32,
                cf_ptr: Some(do_info as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"info topic - source|sources|variables|functions|break|frame|args|locals|display|watch\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"list\0" as *const u8 as *const i8,
                abbrvn: b"l\0" as *const u8 as *const i8,
                type_0: argtype::D_list,
                lex_class: 271 as i32,
                cf_ptr: Some(do_list as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"list [-|+|[filename:]lineno|function|range] - list specified line(s)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"next\0" as *const u8 as *const i8,
                abbrvn: b"n\0" as *const u8 as *const i8,
                type_0: argtype::D_next,
                lex_class: 272 as i32,
                cf_ptr: Some(do_next as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"next [COUNT] - step program, proceeding through subroutine calls\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"nexti\0" as *const u8 as *const i8,
                abbrvn: b"ni\0" as *const u8 as *const i8,
                type_0: argtype::D_nexti,
                lex_class: 273 as i32,
                cf_ptr: Some(do_nexti as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"nexti [COUNT] - step one instruction, but proceed through subroutine calls\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"option\0" as *const u8 as *const i8,
                abbrvn: b"o\0" as *const u8 as *const i8,
                type_0: argtype::D_option,
                lex_class: 295 as i32,
                cf_ptr: Some(do_option as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"option [name[=value]] - set or display debugger option(s)\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"print\0" as *const u8 as *const i8,
                abbrvn: b"p\0" as *const u8 as *const i8,
                type_0: argtype::D_print,
                lex_class: 274 as i32,
                cf_ptr: Some(
                    do_print_var as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"print var [var] - print value of a variable or array\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"printf\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_printf,
                lex_class: 275 as i32,
                cf_ptr: Some(
                    do_print_f as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"printf format, [arg], ... - formatted output\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"quit\0" as *const u8 as *const i8,
                abbrvn: b"q\0" as *const u8 as *const i8,
                type_0: argtype::D_quit,
                lex_class: 276 as i32,
                cf_ptr: Some(do_quit as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"quit - exit debugger\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"return\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_return,
                lex_class: 277 as i32,
                cf_ptr: Some(do_return as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"return [value] - make selected stack frame return to its caller\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"run\0" as *const u8 as *const i8,
                abbrvn: b"r\0" as *const u8 as *const i8,
                type_0: argtype::D_run,
                lex_class: 278 as i32,
                cf_ptr: Some(do_run as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"run - start or restart executing program\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"set\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_set,
                lex_class: 279 as i32,
                cf_ptr: Some(
                    do_set_var as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"set var = value - assign value to a scalar variable\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"silent\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_silent,
                lex_class: 298 as i32,
                cf_ptr: Some(
                    do_commands as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"silent - suspends usual message when stopped at a breakpoint/watchpoint\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"source\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_source,
                lex_class: 299 as i32,
                cf_ptr: Some(do_source as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"source file - execute commands from file\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"step\0" as *const u8 as *const i8,
                abbrvn: b"s\0" as *const u8 as *const i8,
                type_0: argtype::D_step,
                lex_class: 280 as i32,
                cf_ptr: Some(do_step as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"step [COUNT] - step program until it reaches a different source line\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"stepi\0" as *const u8 as *const i8,
                abbrvn: b"si\0" as *const u8 as *const i8,
                type_0: argtype::D_stepi,
                lex_class: 281 as i32,
                cf_ptr: Some(do_stepi as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"stepi [COUNT] - step one instruction exactly\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"tbreak\0" as *const u8 as *const i8,
                abbrvn: b"t\0" as *const u8 as *const i8,
                type_0: argtype::D_tbreak,
                lex_class: 282 as i32,
                cf_ptr: Some(
                    do_tmp_breakpoint as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"tbreak [[filename:]N|function] - set a temporary breakpoint\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"trace\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_trace,
                lex_class: 290 as i32,
                cf_ptr: Some(
                    do_trace_instruction as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"trace on|off - print instruction before executing\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"undisplay\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_undisplay,
                lex_class: 286 as i32,
                cf_ptr: Some(
                    do_undisplay as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"undisplay [N] - remove variable(s) from automatic display list\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"until\0" as *const u8 as *const i8,
                abbrvn: b"u\0" as *const u8 as *const i8,
                type_0: argtype::D_until,
                lex_class: 284 as i32,
                cf_ptr: Some(do_until as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"until [[filename:]N|function] - execute until program reaches a different line or line N within current frame\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"unwatch\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_unwatch,
                lex_class: 288 as i32,
                cf_ptr: Some(
                    do_unwatch as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"unwatch [N] - remove variable(s) from watch list\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"up\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_up,
                lex_class: 283 as i32,
                cf_ptr: Some(do_up as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"up [N] - move N frames up the stack\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"watch\0" as *const u8 as *const i8,
                abbrvn: b"w\0" as *const u8 as *const i8,
                type_0: argtype::D_watch,
                lex_class: 287 as i32,
                cf_ptr: Some(do_watch as unsafe extern "C" fn(*mut CMDARG, i32) -> i32),
                help_txt: b"watch var - set a watchpoint for a variable\0" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: b"where\0" as *const u8 as *const i8,
                abbrvn: b"\0" as *const u8 as *const i8,
                type_0: argtype::D_backtrace,
                lex_class: 258 as i32,
                cf_ptr: Some(
                    do_backtrace as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                ),
                help_txt: b"where [N] - (same as backtrace) print trace of all or N innermost (outermost if N < 0) frames\0"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = cmdtoken {
                name: 0 as *const i8,
                abbrvn: 0 as *const i8,
                type_0: argtype::D_illegal,
                lex_class: 0 as i32,
                cf_ptr: None,
                help_txt: 0 as *const i8,
            };
            init
        },
    ]
};
static mut yypact: [yytype_int16; 203] = [
    -(151 as i32) as yytype_int16,
    145 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(34 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    50 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    10 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(10 as i32) as yytype_int16,
    59 as i32 as yytype_int16,
    -(9 as i32) as yytype_int16,
    43 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    50 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(8 as i32) as yytype_int16,
    -(6 as i32) as yytype_int16,
    14 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    22 as i32 as yytype_int16,
    23 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    59 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    59 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    64 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(34 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    24 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    13 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    59 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    80 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    67 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    48 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    -(20 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(20 as i32) as yytype_int16,
    -(20 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    70 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    16 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    84 as i32 as yytype_int16,
    85 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    73 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    40 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    48 as i32 as yytype_int16,
    59 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    91 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    42 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    81 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    92 as i32 as yytype_int16,
    94 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    86 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    6 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(34 as i32) as yytype_int16,
    75 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    6 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    74 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    48 as i32 as yytype_int16,
    31 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    52 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(17 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    69 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    95 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(34 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    17 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    71 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    6 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    69 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 203] = [
    2 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    18 as i32 as yytype_uint8,
    20 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    7 as i32 as yytype_uint8,
    15 as i32 as yytype_uint8,
    14 as i32 as yytype_uint8,
    17 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    32 as i32 as yytype_uint8,
    19 as i32 as yytype_uint8,
    101 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    117 as i32 as yytype_uint8,
    8 as i32 as yytype_uint8,
    9 as i32 as yytype_uint8,
    38 as i32 as yytype_uint8,
    40 as i32 as yytype_uint8,
    30 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    31 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    10 as i32 as yytype_uint8,
    11 as i32 as yytype_uint8,
    21 as i32 as yytype_uint8,
    16 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    12 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    13 as i32 as yytype_uint8,
    97 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    79 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    61 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    22 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    156 as i32 as yytype_uint8,
    3 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    149 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    24 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    4 as i32 as yytype_uint8,
    6 as i32 as yytype_uint8,
    151 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    44 as i32 as yytype_uint8,
    84 as i32 as yytype_uint8,
    129 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    37 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    103 as i32 as yytype_uint8,
    128 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    102 as i32 as yytype_uint8,
    29 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    35 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    118 as i32 as yytype_uint8,
    119 as i32 as yytype_uint8,
    121 as i32 as yytype_uint8,
    42 as i32 as yytype_uint8,
    122 as i32 as yytype_uint8,
    120 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    99 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    43 as i32 as yytype_uint8,
    95 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    56 as i32 as yytype_uint8,
    62 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    148 as i32 as yytype_uint8,
    57 as i32 as yytype_uint8,
    58 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    63 as i32 as yytype_uint8,
    33 as i32 as yytype_uint8,
    55 as i32 as yytype_uint8,
    153 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    34 as i32 as yytype_uint8,
    150 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    45 as i32 as yytype_uint8,
    89 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    5 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    152 as i32 as yytype_uint8,
    104 as i32 as yytype_uint8,
    133 as i32 as yytype_uint8,
    132 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    36 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    141 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    108 as i32 as yytype_uint8,
    39 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    116 as i32 as yytype_uint8,
    112 as i32 as yytype_uint8,
    114 as i32 as yytype_uint8,
    41 as i32 as yytype_uint8,
    113 as i32 as yytype_uint8,
    144 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    100 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    96 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    74 as i32 as yytype_uint8,
    78 as i32 as yytype_uint8,
    71 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    28 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    154 as i32 as yytype_uint8,
    155 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    27 as i32 as yytype_uint8,
    25 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    87 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    124 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    123 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    143 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    142 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    145 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
    66 as i32 as yytype_uint8,
    76 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    23 as i32 as yytype_uint8,
    72 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    92 as i32 as yytype_uint8,
    90 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    134 as i32 as yytype_uint8,
    140 as i32 as yytype_uint8,
    107 as i32 as yytype_uint8,
    110 as i32 as yytype_uint8,
    115 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    73 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    26 as i32 as yytype_uint8,
    138 as i32 as yytype_uint8,
    0 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    135 as i32 as yytype_uint8,
];
#[no_mangle]
pub static mut zz_debug_argtab: [argtoken; 15] = [
    {
        let mut init = argtoken {
            name: b"args\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_ARGS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"break\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_BREAK,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"del\0" as *const u8 as *const i8,
            cmd: argtype::D_enable,
            value: nametypeval::A_DEL,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"display\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_DISPLAY,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"frame\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_FRAME,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"functions\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_FUNCTIONS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"locals\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_LOCALS,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"off\0" as *const u8 as *const i8,
            cmd: argtype::D_trace,
            value: nametypeval::A_TRACE_OFF,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"on\0" as *const u8 as *const i8,
            cmd: argtype::D_trace,
            value: nametypeval::A_TRACE_ON,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"once\0" as *const u8 as *const i8,
            cmd: argtype::D_enable,
            value: nametypeval::A_ONCE,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"source\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_SOURCE,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"sources\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_SOURCES,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"variables\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_VARIABLES,
        };
        init
    },
    {
        let mut init = argtoken {
            name: b"watch\0" as *const u8 as *const i8,
            cmd: argtype::D_info,
            value: nametypeval::A_WATCH,
        };
        init
    },
    {
        let mut init = argtoken {
            name: 0 as *const i8,
            cmd: argtype::D_illegal,
            value: nametypeval::A_NONE,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn get_command(mut ctype: i32) -> Func_cmd {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !(zz_debug_cmdtab[i as usize].name).is_null() {
        if zz_debug_cmdtab[i as usize].type_0 as u32 == ctype as u32 {
            return zz_debug_cmdtab[i as usize].cf_ptr;
        }
        i += 1;
        i;
    }
    return None;
}
static mut yypgoto: [yytype_int16; 55] = [
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(119 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    38 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(15 as i32) as yytype_int16,
    108 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(90 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(31 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(14 as i32) as yytype_int16,
    -(25 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(150 as i32) as yytype_int16,
    -(26 as i32) as yytype_int16,
    -(77 as i32) as yytype_int16,
    -(147 as i32) as yytype_int16,
    97 as i32 as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(5 as i32) as yytype_int16,
    -(151 as i32) as yytype_int16,
    -(3 as i32) as yytype_int16,
];
#[no_mangle]
pub unsafe extern "C" fn get_command_name(mut ctype: i32) -> *const i8 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !(zz_debug_cmdtab[i as usize].name).is_null() {
        if zz_debug_cmdtab[i as usize].type_0 as u32 == ctype as u32 {
            return zz_debug_cmdtab[i as usize].name;
        }
        i += 1;
        i;
    }
    return 0 as *const i8;
}
static mut yydefgoto: [yytype_uint8; 55] = [
    0 as i32 as yytype_uint8,
    1 as i32 as yytype_uint8,
    46 as i32 as yytype_uint8,
    47 as i32 as yytype_uint8,
    48 as i32 as yytype_uint8,
    49 as i32 as yytype_uint8,
    50 as i32 as yytype_uint8,
    98 as i32 as yytype_uint8,
    51 as i32 as yytype_uint8,
    111 as i32 as yytype_uint8,
    186 as i32 as yytype_uint8,
    52 as i32 as yytype_uint8,
    53 as i32 as yytype_uint8,
    80 as i32 as yytype_uint8,
    81 as i32 as yytype_uint8,
    83 as i32 as yytype_uint8,
    82 as i32 as yytype_uint8,
    85 as i32 as yytype_uint8,
    86 as i32 as yytype_uint8,
    149 as i32 as yytype_uint8,
    175 as i32 as yytype_uint8,
    93 as i32 as yytype_uint8,
    146 as i32 as yytype_uint8,
    147 as i32 as yytype_uint8,
    176 as i32 as yytype_uint8,
    177 as i32 as yytype_uint8,
    91 as i32 as yytype_uint8,
    59 as i32 as yytype_uint8,
    60 as i32 as yytype_uint8,
    109 as i32 as yytype_uint8,
    153 as i32 as yytype_uint8,
    196 as i32 as yytype_uint8,
    139 as i32 as yytype_uint8,
    88 as i32 as yytype_uint8,
    136 as i32 as yytype_uint8,
    70 as i32 as yytype_uint8,
    64 as i32 as yytype_uint8,
    125 as i32 as yytype_uint8,
    126 as i32 as yytype_uint8,
    130 as i32 as yytype_uint8,
    131 as i32 as yytype_uint8,
    77 as i32 as yytype_uint8,
    65 as i32 as yytype_uint8,
    66 as i32 as yytype_uint8,
    67 as i32 as yytype_uint8,
    188 as i32 as yytype_uint8,
    164 as i32 as yytype_uint8,
    165 as i32 as yytype_uint8,
    127 as i32 as yytype_uint8,
    137 as i32 as yytype_uint8,
    94 as i32 as yytype_uint8,
    105 as i32 as yytype_uint8,
    68 as i32 as yytype_uint8,
    106 as i32 as yytype_uint8,
    54 as i32 as yytype_uint8,
];
unsafe extern "C" fn mk_cmdarg(mut type_0: argtype) -> *mut CMDARG {
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    arg = ezalloc_real(
        ::core::mem::size_of::<CMDARG>() as u64,
        b"mk_cmdarg\0" as *const u8 as *const i8,
        b"arg\0" as *const u8 as *const i8,
        b"command.y\0" as *const u8 as *const i8,
        962 as i32,
    ) as *mut CMDARG;
    (*arg).type_0 = type_0;
    return arg;
}
static mut yytable: [yytype_int16; 204] = [
    55 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    76 as i32 as yytype_int16,
    78 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    121 as i32 as yytype_int16,
    138 as i32 as yytype_int16,
    174 as i32 as yytype_int16,
    140 as i32 as yytype_int16,
    141 as i32 as yytype_int16,
    71 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    79 as i32 as yytype_int16,
    92 as i32 as yytype_int16,
    62 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    189 as i32 as yytype_int16,
    143 as i32 as yytype_int16,
    198 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    128 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    101 as i32 as yytype_int16,
    45 as i32 as yytype_int16,
    61 as i32 as yytype_int16,
    194 as i32 as yytype_int16,
    69 as i32 as yytype_int16,
    72 as i32 as yytype_int16,
    87 as i32 as yytype_int16,
    182 as i32 as yytype_int16,
    89 as i32 as yytype_int16,
    187 as i32 as yytype_int16,
    95 as i32 as yytype_int16,
    185 as i32 as yytype_int16,
    108 as i32 as yytype_int16,
    169 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    115 as i32 as yytype_int16,
    99 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    190 as i32 as yytype_int16,
    95 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    110 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    112 as i32 as yytype_int16,
    90 as i32 as yytype_int16,
    202 as i32 as yytype_int16,
    116 as i32 as yytype_int16,
    144 as i32 as yytype_int16,
    145 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    129 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    96 as i32 as yytype_int16,
    97 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    117 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    -(75 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    -(127 as i32) as yytype_int16,
    133 as i32 as yytype_int16,
    -(147 as i32) as yytype_int16,
    -(127 as i32) as yytype_int16,
    102 as i32 as yytype_int16,
    200 as i32 as yytype_int16,
    -(69 as i32) as yytype_int16,
    113 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    201 as i32 as yytype_int16,
    192 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    73 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    154 as i32 as yytype_int16,
    114 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    155 as i32 as yytype_int16,
    103 as i32 as yytype_int16,
    104 as i32 as yytype_int16,
    122 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    132 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    74 as i32 as yytype_int16,
    75 as i32 as yytype_int16,
    157 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    107 as i32 as yytype_int16,
    118 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    123 as i32 as yytype_int16,
    160 as i32 as yytype_int16,
    161 as i32 as yytype_int16,
    133 as i32 as yytype_int16,
    158 as i32 as yytype_int16,
    124 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    156 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    159 as i32 as yytype_int16,
    180 as i32 as yytype_int16,
    162 as i32 as yytype_int16,
    119 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    142 as i32 as yytype_int16,
    150 as i32 as yytype_int16,
    151 as i32 as yytype_int16,
    134 as i32 as yytype_int16,
    135 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    181 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    167 as i32 as yytype_int16,
    171 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    172 as i32 as yytype_int16,
    178 as i32 as yytype_int16,
    195 as i32 as yytype_int16,
    173 as i32 as yytype_int16,
    148 as i32 as yytype_int16,
    183 as i32 as yytype_int16,
    84 as i32 as yytype_int16,
    193 as i32 as yytype_int16,
    191 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    179 as i32 as yytype_int16,
    100 as i32 as yytype_int16,
    2 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    184 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
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
    36 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    197 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    40 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
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
    45 as i32 as yytype_int16,
];
unsafe extern "C" fn append_cmdarg(mut arg: *mut CMDARG) {
    static mut savetail: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
    if arg_list.is_null() {
        arg_list = arg;
    } else {
        (*savetail).next = arg;
    }
    savetail = arg;
}
#[no_mangle]
pub unsafe extern "C" fn free_cmdarg(mut list: *mut CMDARG) {
    let mut arg: *mut CMDARG = 0 as *mut CMDARG;
    let mut nexta: *mut CMDARG = 0 as *mut CMDARG;
    arg = list;
    while !arg.is_null() {
        nexta = (*arg).next;
        match (*arg).type_0 as u32 {
            45 | 49 | 48 | 44 => {
                if !((*arg).value.sval).is_null() {
                    pma_free((*arg).value.sval as *mut libc::c_void);
                }
            }
            46 | 47 => {
                unref((*arg).value.nodeval);
            }
            _ => {}
        }
        pma_free(arg as *mut libc::c_void);
        arg = nexta;
    }
}
static mut yycheck: [yytype_int16; 204] = [
    3 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    81 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    83 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    85 as i32 as yytype_int16,
    86 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    165 as i32 as yytype_int16,
    163 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    173 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    149 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    153 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    126 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    63 as i32 as yytype_int16,
    44 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    191 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    199 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    67 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    56 as i32 as yytype_int16,
    58 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    196 as i32 as yytype_int16,
    168 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    48 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    52 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    113 as i32 as yytype_int16,
    57 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    118 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    120 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    36 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    54 as i32 as yytype_int16,
    51 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    55 as i32 as yytype_int16,
    53 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    50 as i32 as yytype_int16,
    38 as i32 as yytype_int16,
    37 as i32 as yytype_int16,
    39 as i32 as yytype_int16,
    49 as i32 as yytype_int16,
    98 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    30 as i32 as yytype_int16,
    170 as i32 as yytype_int16,
    166 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    146 as i32 as yytype_int16,
    47 as i32 as yytype_int16,
    0 as i32 as yytype_int16,
    1 as i32 as yytype_int16,
    152 as i32 as yytype_int16,
    3 as i32 as yytype_int16,
    4 as i32 as yytype_int16,
    5 as i32 as yytype_int16,
    6 as i32 as yytype_int16,
    7 as i32 as yytype_int16,
    8 as i32 as yytype_int16,
    9 as i32 as yytype_int16,
    10 as i32 as yytype_int16,
    11 as i32 as yytype_int16,
    12 as i32 as yytype_int16,
    13 as i32 as yytype_int16,
    14 as i32 as yytype_int16,
    15 as i32 as yytype_int16,
    16 as i32 as yytype_int16,
    17 as i32 as yytype_int16,
    18 as i32 as yytype_int16,
    19 as i32 as yytype_int16,
    20 as i32 as yytype_int16,
    21 as i32 as yytype_int16,
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
    186 as i32 as yytype_int16,
    -(1 as i32) as yytype_int16,
    40 as i32 as yytype_int16,
    41 as i32 as yytype_int16,
    42 as i32 as yytype_int16,
    43 as i32 as yytype_int16,
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
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    -(1 as i32) as yytype_int16,
    58 as i32 as yytype_int16,
];
unsafe extern "C" fn zzerror(mut mesg: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fprintf(
        out_fp,
        dcgettext(0 as *const i8, b"error: \0" as *const u8 as *const i8, 5 as i32),
    );
    vfprintf(out_fp, mesg, args_0.as_va_list());
    fprintf(out_fp, b"\n\0" as *const u8 as *const i8);
    dbg_errcount += 1;
    dbg_errcount;
    repeat_idx = -(1 as i32);
}
static mut yystos: [yytype_int8; 203] = [
    0 as i32 as yytype_int8,
    60 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
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
    40 as i32 as yytype_int8,
    41 as i32 as yytype_int8,
    42 as i32 as yytype_int8,
    43 as i32 as yytype_int8,
    44 as i32 as yytype_int8,
    45 as i32 as yytype_int8,
    46 as i32 as yytype_int8,
    47 as i32 as yytype_int8,
    58 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    65 as i32 as yytype_int8,
    67 as i32 as yytype_int8,
    70 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    95 as i32 as yytype_int8,
    101 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    103 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    94 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    101 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    72 as i32 as yytype_int8,
    73 as i32 as yytype_int8,
    75 as i32 as yytype_int8,
    74 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    76 as i32 as yytype_int8,
    77 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    92 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    85 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    80 as i32 as yytype_int8,
    109 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    66 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    109 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    110 as i32 as yytype_int8,
    112 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    68 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
    51 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    101 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    51 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    52 as i32 as yytype_int8,
    57 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    97 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    98 as i32 as yytype_int8,
    99 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    53 as i32 as yytype_int8,
    54 as i32 as yytype_int8,
    93 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    91 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    49 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    81 as i32 as yytype_int8,
    82 as i32 as yytype_int8,
    84 as i32 as yytype_int8,
    78 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    36 as i32 as yytype_int8,
    51 as i32 as yytype_int8,
    89 as i32 as yytype_int8,
    42 as i32 as yytype_int8,
    48 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    101 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    55 as i32 as yytype_int8,
    105 as i32 as yytype_int8,
    106 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    38 as i32 as yytype_int8,
    49 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    79 as i32 as yytype_int8,
    83 as i32 as yytype_int8,
    84 as i32 as yytype_int8,
    37 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    79 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    79 as i32 as yytype_int8,
    69 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    104 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    105 as i32 as yytype_int8,
    106 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    98 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    39 as i32 as yytype_int8,
    90 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    50 as i32 as yytype_int8,
    56 as i32 as yytype_int8,
    79 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
];
unsafe extern "C" fn zzlex() -> i32 {
    static mut lexptr: *mut i8 = 0 as *const i8 as *mut i8;
    static mut lexend: *mut i8 = 0 as *const i8 as *mut i8;
    let mut c: i32 = 0;
    let mut tokstart: *mut i8 = 0 as *mut i8;
    let mut toklen: size_t = 0;
    zzlval = 0 as *mut libc::c_void as *mut CMDARG;
    if dbg_errcount > 0 as i32 as i64 && lexptr_begin.is_null() {
        dbg_errcount = 0 as i32 as i64;
        return '\n' as i32;
    }
    if lexptr_begin.is_null() {
        's_147: {
            loop {
                lexptr_begin = read_a_line
                    .expect("non-null function pointer")(dbg_prompt);
                if lexptr_begin.is_null() {
                    if get_eof_status() == 2 as i32 {
                        exit(2 as i32);
                    }
                    if get_eof_status() == 1 as i32 {
                        static mut seen_eof: i32 = 0 as i32;
                        if seen_eof == 0 {
                            if *__errno_location() != 0 as i32 {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const i8,
                                        b"cannot read command: %s\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    strerror(*__errno_location()),
                                );
                                exit_val = 1 as i32;
                            }
                            seen_eof = 1 as i32;
                            return '\n' as i32;
                        } else {
                            let fresh3 = seen_eof;
                            seen_eof = seen_eof + 1;
                            if fresh3 == 1 as i32 {
                                cmd_idx = find_command(
                                    b"quit\0" as *const u8 as *const i8,
                                    4 as i32 as size_t,
                                );
                                return 276 as i32;
                            } else {
                                return '\n' as i32
                            }
                        }
                    }
                    if *__errno_location() != 0 as i32 {
                        d_error(
                            dcgettext(
                                0 as *const i8,
                                b"cannot read command: %s\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            strerror(*__errno_location()),
                        );
                    }
                    if pop_cmd_src() == 0 as i32 {
                        continue;
                    }
                    exit(2 as i32);
                } else {
                    !in_commands && !in_eval && input_from_tty as i32 != 0;
                    lexptr = lexptr_begin;
                    lexend = lexptr.offset(strlen(lexptr) as isize);
                    if *lexptr as i32 == '\0' as i32 && repeat_idx >= 0 as i32
                        && input_from_tty as i32 != 0 && !in_eval
                    {
                        cmd_idx = repeat_idx;
                        return zz_debug_cmdtab[cmd_idx as usize].lex_class;
                    }
                    repeat_idx = -(1 as i32);
                    break 's_147;
                }
            }
        }
    }
    c = *lexptr as i32;
    while c == ' ' as i32 || c == '\t' as i32 {
        lexptr = lexptr.offset(1);
        c = *lexptr as i32;
    }
    if !input_from_tty && c == '#' as i32 {
        return '\n' as i32;
    }
    tokstart = lexptr;
    if lexptr >= lexend {
        return '\n' as i32;
    }
    if cmd_idx < 0 as i32 {
        if c == '?' as i32 && *tokstart.offset(1 as i32 as isize) as i32 == '\0' as i32
            && !in_eval
        {
            lexptr = lexptr.offset(1);
            lexptr;
            cmd_idx = find_command(
                b"help\0" as *const u8 as *const i8,
                4 as i32 as size_t,
            );
            return 268 as i32;
        }
        while c != '\0' as i32 && c != ' ' as i32 && c != '\t' as i32 {
            if !is_alpha(c) && !in_eval {
                zzerror(
                    dcgettext(
                        0 as *const i8,
                        b"invalid character in command\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return '\n' as i32;
            }
            lexptr = lexptr.offset(1);
            c = *lexptr as i32;
        }
        toklen = lexptr.offset_from(tokstart) as i64 as size_t;
        if in_eval {
            if toklen == 3 as i32 as u64
                && *tokstart.offset(3 as i32 as isize) as i32 == '\0' as i32
                && *tokstart.offset(0 as i32 as isize) as i32 == 'e' as i32
                && *tokstart.offset(1 as i32 as isize) as i32 == 'n' as i32
                && *tokstart.offset(2 as i32 as isize) as i32 == 'd' as i32
            {
                cmd_idx = find_command(tokstart, toklen);
                return 297 as i32;
            }
            lexptr = lexend;
            return 303 as i32;
        }
        cmd_idx = find_command(tokstart, toklen);
        if cmd_idx >= 0 as i32 {
            if in_commands as i32 != 0
                && zz_debug_cmdtab[cmd_idx as usize].type_0 as u32
                    != argtype::D_eval as i32 as u32
            {
                let mut arg: *mut CMDARG = 0 as *mut CMDARG;
                arg = mk_cmdarg(argtype::D_string);
                (*arg).value.sval = estrdup(
                    lexptr_begin,
                    lexend.offset_from(lexptr_begin) as i64 as size_t,
                );
                append_cmdarg(arg);
            }
            return zz_debug_cmdtab[cmd_idx as usize].lex_class;
        } else {
            zzerror(
                dcgettext(
                    0 as *const i8,
                    b"unknown command - `%.*s', try help\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                toklen,
                tokstart,
            );
            return '\n' as i32;
        }
    }
    c = *lexptr as i32;
    if zz_debug_cmdtab[cmd_idx as usize].type_0 as u32 == argtype::D_option as i32 as u32
    {
        if c == '=' as i32 {
            let fresh4 = lexptr;
            lexptr = lexptr.offset(1);
            return *fresh4 as i32;
        }
    } else if c == '-' as i32 || c == '+' as i32 || c == ':' as i32 || c == '|' as i32 {
        let fresh5 = lexptr;
        lexptr = lexptr.offset(1);
        return *fresh5 as i32;
    }
    if c == '"' as i32 {
        let mut str: *mut i8 = 0 as *mut i8;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut flags: i32 = 2 as i32;
        let mut esc_seen: bool = 0 as i32 != 0;
        toklen = lexend.offset_from(lexptr) as i64 as size_t;
        str = emalloc_real(
            toklen.wrapping_add(1 as i32 as u64),
            b"yylex\0" as *const u8 as *const i8,
            b"str\0" as *const u8 as *const i8,
            b"command.y\0" as *const u8 as *const i8,
            1181 as i32,
        ) as *mut i8;
        p = str;
        loop {
            lexptr = lexptr.offset(1);
            c = *lexptr as i32;
            if !(c != '"' as i32) {
                break;
            }
            's_398: {
                if !(lexptr == lexend) {
                    if c == '\\' as i32 {
                        lexptr = lexptr.offset(1);
                        c = *lexptr as i32;
                        esc_seen = 1 as i32 != 0;
                        if want_nodeval as i32 != 0 || c != '"' as i32 {
                            let fresh6 = p;
                            p = p.offset(1);
                            *fresh6 = '\\' as i32 as i8;
                        }
                    }
                    if !(lexptr == lexend) {
                        let fresh7 = p;
                        p = p.offset(1);
                        *fresh7 = c as i8;
                        break 's_398;
                    }
                }
                pma_free(str as *mut libc::c_void);
                zzerror(
                    dcgettext(
                        0 as *const i8,
                        b"unterminated string\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                return '\n' as i32;
            }
        }
        lexptr = lexptr.offset(1);
        lexptr;
        *p = '\0' as i32 as i8;
        if !want_nodeval {
            zzlval = mk_cmdarg(argtype::D_string);
            (*zzlval).value.sval = str;
            append_cmdarg(zzlval);
            return 292 as i32;
        } else {
            if esc_seen {
                flags |= 1 as i32;
            }
            zzlval = mk_cmdarg(argtype::D_node);
            (*zzlval).value.nodeval = make_str_node(
                str,
                p.offset_from(str) as i64 as size_t,
                flags,
            );
            append_cmdarg(zzlval);
            return 293 as i32;
        }
    }
    if !want_nodeval {
        loop {
            lexptr = lexptr.offset(1);
            c = *lexptr as i32;
            if !(c != '\0' as i32 && c != ':' as i32 && c != '-' as i32
                && c != ' ' as i32 && c != '\t' as i32 && c != '=' as i32)
            {
                break;
            }
        }
        if *(*__ctype_b_loc())
            .offset(*tokstart.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
            && zz_debug_cmdtab[cmd_idx as usize].type_0 as u32
                != argtype::D_option as i32 as u32
        {
            let mut end: *mut i8 = 0 as *mut i8;
            let mut l: i64 = 0;
            *__errno_location() = 0 as i32;
            l = strtol(tokstart, &mut end, 0 as i32);
            if *__errno_location() != 0 as i32 {
                zzerror(
                    dcgettext(
                        0 as *const i8,
                        b"%s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    strerror(*__errno_location()),
                );
                *__errno_location() = 0 as i32;
                return '\n' as i32;
            }
            if lexptr == end {
                zzlval = mk_cmdarg(argtype::D_int);
                (*zzlval).value.lval = l;
                append_cmdarg(zzlval);
                return 291 as i32;
            }
        }
        zzlval = mk_cmdarg(argtype::D_string);
        (*zzlval).value.sval = estrdup(
            tokstart,
            lexptr.offset_from(tokstart) as i64 as size_t,
        );
        append_cmdarg(zzlval);
        return 292 as i32;
    }
    if *(*__ctype_b_loc())
        .offset(*tokstart.offset(0 as i32 as isize) as u8 as i32 as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        let mut r: *mut NODE = 0 as *mut NODE;
        *__errno_location() = 0 as i32;
        r = make_number
            .expect("non-null function pointer")(strtod(tokstart, &mut lexptr));
        if *__errno_location() != 0 as i32 {
            zzerror(strerror(*__errno_location()));
            unref(r);
            *__errno_location() = 0 as i32;
            return '\n' as i32;
        }
        zzlval = mk_cmdarg(argtype::D_node);
        (*zzlval).value.nodeval = r;
        append_cmdarg(zzlval);
        return 293 as i32;
    }
    c = *lexptr as i32;
    if c == '$' as i32 || c == '@' as i32 || c == '[' as i32 || c == ']' as i32
        || c == ',' as i32 || c == '=' as i32
    {
        let fresh8 = lexptr;
        lexptr = lexptr.offset(1);
        return *fresh8 as i32;
    }
    if !is_letter(c) {
        zzerror(
            dcgettext(
                0 as *const i8,
                b"invalid character\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return '\n' as i32;
    }
    while is_identchar(c) {
        lexptr = lexptr.offset(1);
        c = *lexptr as i32;
    }
    toklen = lexptr.offset_from(tokstart) as i64 as size_t;
    zzlval = mk_cmdarg(argtype::D_variable);
    (*zzlval).value.sval = estrdup(tokstart, toklen);
    append_cmdarg(zzlval);
    return 294 as i32;
}
static mut yyr1: [yytype_int8; 157] = [
    0 as i32 as yytype_int8,
    59 as i32 as yytype_int8,
    60 as i32 as yytype_int8,
    60 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    61 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    62 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    63 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    64 as i32 as yytype_int8,
    65 as i32 as yytype_int8,
    65 as i32 as yytype_int8,
    66 as i32 as yytype_int8,
    67 as i32 as yytype_int8,
    68 as i32 as yytype_int8,
    69 as i32 as yytype_int8,
    68 as i32 as yytype_int8,
    70 as i32 as yytype_int8,
    70 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    72 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    73 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    74 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    75 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    76 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    77 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    78 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    71 as i32 as yytype_int8,
    79 as i32 as yytype_int8,
    80 as i32 as yytype_int8,
    80 as i32 as yytype_int8,
    81 as i32 as yytype_int8,
    81 as i32 as yytype_int8,
    82 as i32 as yytype_int8,
    82 as i32 as yytype_int8,
    82 as i32 as yytype_int8,
    82 as i32 as yytype_int8,
    83 as i32 as yytype_int8,
    83 as i32 as yytype_int8,
    83 as i32 as yytype_int8,
    84 as i32 as yytype_int8,
    85 as i32 as yytype_int8,
    85 as i32 as yytype_int8,
    85 as i32 as yytype_int8,
    86 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    87 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    89 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    90 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    88 as i32 as yytype_int8,
    91 as i32 as yytype_int8,
    91 as i32 as yytype_int8,
    92 as i32 as yytype_int8,
    92 as i32 as yytype_int8,
    93 as i32 as yytype_int8,
    93 as i32 as yytype_int8,
    94 as i32 as yytype_int8,
    94 as i32 as yytype_int8,
    95 as i32 as yytype_int8,
    95 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    96 as i32 as yytype_int8,
    97 as i32 as yytype_int8,
    97 as i32 as yytype_int8,
    97 as i32 as yytype_int8,
    97 as i32 as yytype_int8,
    98 as i32 as yytype_int8,
    98 as i32 as yytype_int8,
    99 as i32 as yytype_int8,
    99 as i32 as yytype_int8,
    99 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    100 as i32 as yytype_int8,
    101 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    102 as i32 as yytype_int8,
    103 as i32 as yytype_int8,
    103 as i32 as yytype_int8,
    103 as i32 as yytype_int8,
    103 as i32 as yytype_int8,
    104 as i32 as yytype_int8,
    104 as i32 as yytype_int8,
    104 as i32 as yytype_int8,
    105 as i32 as yytype_int8,
    105 as i32 as yytype_int8,
    106 as i32 as yytype_int8,
    106 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    107 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    108 as i32 as yytype_int8,
    109 as i32 as yytype_int8,
    109 as i32 as yytype_int8,
    110 as i32 as yytype_int8,
    110 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    111 as i32 as yytype_int8,
    112 as i32 as yytype_int8,
    112 as i32 as yytype_int8,
    112 as i32 as yytype_int8,
    113 as i32 as yytype_int8,
];
static mut yyr2: [yytype_int8; 157] = [
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    4 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    5 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    3 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    0 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    2 as i32 as yytype_int8,
    1 as i32 as yytype_int8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const i8,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut *mut CMDARG,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const i8;
    }
}
#[no_mangle]
pub static mut zzchar: i32 = 0;
#[no_mangle]
pub static mut zzlval: *mut CMDARG = 0 as *const CMDARG as *mut CMDARG;
#[no_mangle]
pub static mut zznerrs: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn zzparse() -> i32 {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0 as i32;
    let mut yyerrstatus: i32 = 0 as i32;
    let mut yystacksize: i64 = 200 as i32 as i64;
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [*mut CMDARG; 200] = [0 as *mut CMDARG; 200];
    let mut yyvs: *mut *mut CMDARG = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut *mut CMDARG = yyvs;
    let mut yyn: i32 = 0;
    let mut yyresult: i32 = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: *mut CMDARG = 0 as *mut CMDARG;
    let mut yylen: i32 = 0 as i32;
    zzchar = -(2 as i32);
    's_46: loop {
        (0 as i32 != 0 && (0 as i32 <= yystate && yystate < 203 as i32)) as i32;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as i32 as isize)) <= yyssp {
            let mut yysize: i64 = yyssp.offset_from(yyss) as i64 + 1 as i32 as i64;
            if 10000 as i32 as i64 <= yystacksize {
                current_block = 16842697970582414543;
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
                        + ::core::mem::size_of::<*mut CMDARG>() as u64 as i64)
                    + (::core::mem::size_of::<yyalloc>() as u64 as i64
                        - 1 as i32 as i64)) as u64,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 16842697970582414543;
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
                &mut (*yyptr).yyvs_alloc as *mut *mut CMDARG as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut CMDARG>() as u64)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<*mut CMDARG>() as u64 as i64
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
                current_block = 14500194797246379994;
                break;
            }
        }
        if yystate == 2 as i32 {
            current_block = 16033749021571576439;
            break;
        }
        yyn = yypact[yystate as usize] as i32;
        if yyn == -(151 as i32) {
            current_block = 2309065947381692012;
        } else {
            if zzchar == -(2 as i32) {
                zzchar = zzlex();
            }
            if zzchar <= 0 as i32 {
                zzchar = 0 as i32;
                yytoken = YYSYMBOL_YYEOF;
                current_block = 6174974146017752131;
            } else if zzchar == 256 as i32 {
                zzchar = 257 as i32;
                yytoken = YYSYMBOL_YYerror;
                current_block = 3309994273773591571;
            } else {
                yytoken = (if 0 as i32 <= zzchar && zzchar <= 303 as i32 {
                    yytranslate[zzchar as usize] as yysymbol_kind_t as i32
                } else {
                    YYSYMBOL_YYUNDEF as i32
                }) as yysymbol_kind_t;
                current_block = 6174974146017752131;
            }
            match current_block {
                3309994273773591571 => {}
                _ => {
                    yyn += yytoken as i32;
                    if yyn < 0 as i32 || (203 as i32) < yyn
                        || yycheck[yyn as usize] as i32 != yytoken as i32
                    {
                        current_block = 2309065947381692012;
                    } else {
                        yyn = yytable[yyn as usize] as i32;
                        if yyn <= 0 as i32 {
                            yyn = -yyn;
                            current_block = 13876667199569387813;
                        } else {
                            if yyerrstatus != 0 {
                                yyerrstatus -= 1;
                                yyerrstatus;
                            }
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = zzlval;
                            zzchar = -(2 as i32);
                            current_block = 14300186675387049666;
                        }
                    }
                }
            }
        }
        match current_block {
            2309065947381692012 => {
                yyn = yydefact[yystate as usize] as i32;
                if yyn == 0 as i32 {
                    yytoken = (if zzchar == -(2 as i32) {
                        YYSYMBOL_YYEMPTY as i32
                    } else if 0 as i32 <= zzchar && zzchar <= 303 as i32 {
                        yytranslate[zzchar as usize] as yysymbol_kind_t as i32
                    } else {
                        YYSYMBOL_YYUNDEF as i32
                    }) as yysymbol_kind_t;
                    if yyerrstatus == 0 {
                        zznerrs += 1;
                        zznerrs;
                        zzerror(b"syntax error\0" as *const u8 as *const i8);
                    }
                    if yyerrstatus == 3 as i32 {
                        if zzchar <= 0 as i32 {
                            if zzchar == 0 as i32 {
                                current_block = 14500194797246379994;
                                break;
                            }
                        } else {
                            yydestruct(
                                b"Error: discarding\0" as *const u8 as *const i8,
                                yytoken,
                                &mut zzlval,
                            );
                            zzchar = -(2 as i32);
                        }
                    }
                    current_block = 3309994273773591571;
                } else {
                    current_block = 13876667199569387813;
                }
            }
            _ => {}
        }
        match current_block {
            13876667199569387813 => {
                yylen = yyr2[yyn as usize] as i32;
                yyval = *yyvsp.offset((1 as i32 - yylen) as isize);
                match yyn {
                    3 => {
                        cmd_idx = -(1 as i32);
                        want_nodeval = 0 as i32 != 0;
                        if !lexptr_begin.is_null() {
                            input_from_tty as i32 != 0
                                && *lexptr_begin.offset(0 as i32 as isize) as i32
                                    != '\0' as i32;
                            pma_free(lexptr_begin as *mut libc::c_void);
                            lexptr_begin = 0 as *mut i8;
                        }
                        if !arg_list.is_null() {
                            free_cmdarg(arg_list);
                            arg_list = 0 as *mut CMDARG;
                        }
                    }
                    5 => {
                        if dbg_errcount == 0 as i32 as i64 && cmd_idx >= 0 as i32 {
                            let mut cmdfunc: Func_cmd = None;
                            let mut terminate: bool = 0 as i32 != 0;
                            let mut args: *mut CMDARG = 0 as *mut CMDARG;
                            let mut ctype: i32 = 0 as i32;
                            ctype = (*zz_debug_cmdtab
                                .as_mut_ptr()
                                .offset(cmd_idx as isize))
                                .type_0 as i32;
                            if (ctype == argtype::D_list as i32
                                || ctype == argtype::D_next as i32
                                || ctype == argtype::D_step as i32
                                || ctype == argtype::D_nexti as i32
                                || ctype == argtype::D_stepi as i32
                                || ctype == argtype::D_continue as i32)
                                && arg_list.is_null() && !in_commands
                                && input_from_tty as i32 != 0
                            {
                                repeat_idx = cmd_idx;
                            } else {
                                repeat_idx = -(1 as i32);
                            }
                            cmdfunc = (*zz_debug_cmdtab
                                .as_mut_ptr()
                                .offset(cmd_idx as isize))
                                .cf_ptr;
                            if in_commands {
                                cmdfunc = Some(
                                    do_commands as unsafe extern "C" fn(*mut CMDARG, i32) -> i32,
                                );
                            }
                            cmd_idx = -(1 as i32);
                            want_nodeval = 0 as i32 != 0;
                            args = arg_list;
                            arg_list = 0 as *mut CMDARG;
                            terminate = (Some(
                                cmdfunc.expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")(args, ctype) != 0;
                            if !in_commands || ctype == argtype::D_commands as i32 {
                                free_cmdarg(args);
                            }
                            if terminate {
                                current_block = 16033749021571576439;
                                break;
                            }
                        }
                    }
                    6 => {
                        yyerrstatus = 0 as i32;
                    }
                    22 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    23 => {
                        if dbg_errcount == 0 as i32 as i64 {
                            if input_from_tty {
                                dbg_prompt = eval_prompt;
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Type (g)awk statement(s). End with the command `end'\n\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                                rl_inhibit_completion = 1 as i32;
                            }
                            cmd_idx = -(1 as i32);
                            in_eval = 1 as i32 != 0;
                        }
                    }
                    24 => {
                        yyval = append_statement(
                            arg_list,
                            start_EVAL.as_ptr() as *mut i8,
                        );
                        if read_a_line
                            == Some(
                                read_commands_string
                                    as unsafe extern "C" fn(*const i8) -> *mut i8,
                            )
                        {
                            *((*yyval).value.sval).offset(0 as i32 as isize) = '\0'
                                as i32 as i8;
                        }
                        free_cmdarg(arg_list);
                        arg_list = 0 as *mut CMDARG;
                    }
                    25 => {
                        yyval = append_statement(
                            *yyvsp.offset(-(1 as i32) as isize),
                            lexptr_begin,
                        );
                    }
                    26 => {
                        yyval = *yyvsp.offset(-(1 as i32) as isize);
                    }
                    27 => {
                        arg_list = append_statement(
                            *yyvsp.offset(-(1 as i32) as isize),
                            end_EVAL.as_ptr() as *mut i8,
                        );
                        if read_a_line
                            == Some(
                                read_commands_string
                                    as unsafe extern "C" fn(*const i8) -> *mut i8,
                            )
                        {
                            let mut str: *mut i8 = (*arg_list).value.sval;
                            let mut len: size_t = strlen(str);
                            *str.offset(len.wrapping_sub(2 as i32 as u64) as isize) = '\0'
                                as i32 as i8;
                        }
                        if input_from_tty {
                            dbg_prompt = if in_commands as i32 != 0 {
                                commands_prompt
                            } else {
                                dgawk_prompt
                            };
                            rl_inhibit_completion = 0 as i32;
                        }
                        cmd_idx = find_command(
                            b"eval\0" as *const u8 as *const i8,
                            4 as i32 as size_t,
                        );
                        in_eval = 0 as i32 != 0;
                    }
                    28 => {
                        let mut n: *mut NODE = 0 as *mut NODE;
                        let mut arg: *mut CMDARG = 0 as *mut CMDARG;
                        n = (**yyvsp.offset(0 as i32 as isize)).value.nodeval;
                        arg = append_statement(
                            0 as *mut CMDARG,
                            start_EVAL.as_ptr() as *mut i8,
                        );
                        append_statement(arg, (*n).sub.val.sp);
                        append_statement(arg, end_EVAL.as_ptr() as *mut i8);
                        free_cmdarg(arg_list);
                        arg_list = arg;
                    }
                    34 => {
                        if (*zz_debug_cmdtab.as_mut_ptr().offset(cmd_idx as isize))
                            .lex_class == 267 as i32
                            && !(*yyvsp.offset(0 as i32 as isize)).is_null()
                            && (**yyvsp.offset(0 as i32 as isize)).value.lval
                                < 0 as i32 as i64
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"invalid frame number: %d\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.lval,
                            );
                        }
                    }
                    35 => {
                        let mut idx: i32 = find_argument(
                            *yyvsp.offset(0 as i32 as isize),
                        );
                        if idx < 0 as i32 {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"info: invalid option - `%s'\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        } else {
                            pma_free(
                                (**yyvsp.offset(0 as i32 as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh9 = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .sval;
                            *fresh9 = 0 as *mut i8;
                            (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_argument;
                            (**yyvsp.offset(0 as i32 as isize)).value.lval = (*zz_debug_argtab
                                .as_mut_ptr()
                                .offset(idx as isize))
                                .value as i64;
                        }
                    }
                    38 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    40 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    46 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    49 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    51 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    53 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    57 => {
                        if in_cmd_src((**yyvsp.offset(0 as i32 as isize)).value.sval)
                            != 0
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"source: `%s': already sourced\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        }
                    }
                    58 => {
                        if !input_from_tty {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"save: `%s': command not permitted\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        }
                    }
                    59 => {
                        let mut type_0: i32 = 0 as i32;
                        let mut num: i32 = 0;
                        if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                            num = (**yyvsp.offset(0 as i32 as isize)).value.lval as i32;
                        }
                        if !(dbg_errcount != 0 as i32 as i64) {
                            if in_commands {
                                zzerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"cannot use command `commands' for breakpoint/watchpoint commands\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            } else if (*yyvsp.offset(0 as i32 as isize)).is_null()
                                && {
                                    type_0 = has_break_or_watch_point(&mut num, 1 as i32 != 0);
                                    type_0 == 0
                                }
                            {
                                zzerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"no breakpoint/watchpoint has been set yet\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            } else if !(*yyvsp.offset(0 as i32 as isize)).is_null()
                                && {
                                    type_0 = has_break_or_watch_point(&mut num, 0 as i32 != 0);
                                    type_0 == 0
                                }
                            {
                                zzerror(
                                    dcgettext(
                                        0 as *const i8,
                                        b"invalid breakpoint/watchpoint number\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                        }
                        if type_0 != 0 {
                            in_commands = 1 as i32 != 0;
                            if input_from_tty {
                                dbg_prompt = commands_prompt;
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const i8,
                                        b"Type commands for when %s %d is hit, one per line.\n\0"
                                            as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                    if type_0 == argtype::D_break as i32 {
                                        b"breakpoint\0" as *const u8 as *const i8
                                    } else {
                                        b"watchpoint\0" as *const u8 as *const i8
                                    },
                                    num,
                                );
                                fprintf(
                                    out_fp,
                                    dcgettext(
                                        0 as *const i8,
                                        b"End with the command `end'\n\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                        }
                    }
                    60 => {
                        if !in_commands {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"`end' valid only in command `commands' or `eval'\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        } else {
                            if input_from_tty {
                                dbg_prompt = dgawk_prompt;
                            }
                            in_commands = 0 as i32 != 0;
                        }
                    }
                    61 => {
                        if !in_commands {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"`silent' valid only in command `commands'\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                    }
                    62 => {
                        let mut idx_0: i32 = find_argument(
                            *yyvsp.offset(0 as i32 as isize),
                        );
                        if idx_0 < 0 as i32 {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"trace: invalid option - `%s'\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        } else {
                            pma_free(
                                (**yyvsp.offset(0 as i32 as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh10 = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .sval;
                            *fresh10 = 0 as *mut i8;
                            (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_argument;
                            (**yyvsp.offset(0 as i32 as isize)).value.lval = (*zz_debug_argtab
                                .as_mut_ptr()
                                .offset(idx_0 as isize))
                                .value as i64;
                        }
                    }
                    63 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    64 => {
                        let mut type_1: i32 = 0;
                        let mut num_0: i32 = (**yyvsp.offset(-(2 as i32) as isize))
                            .value
                            .lval as i32;
                        type_1 = has_break_or_watch_point(&mut num_0, 0 as i32 != 0);
                        if type_1 == 0 {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"condition: invalid breakpoint/watchpoint number\0"
                                        as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                    }
                    65 => {
                        if in_commands {
                            let mut arg_0: *mut CMDARG = 0 as *mut CMDARG;
                            arg_0 = mk_cmdarg(argtype::D_string);
                            (*arg_0).value.sval = estrdup(
                                b"eval\0" as *const u8 as *const i8,
                                4 as i32 as size_t,
                            );
                            (*arg_0).next = arg_list;
                            arg_list = arg_0;
                        }
                    }
                    66 => {
                        if !(*yyvsp.offset(0 as i32 as isize)).is_null() {
                            let mut n_0: *mut NODE = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .nodeval;
                            (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_string;
                            let ref mut fresh11 = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .sval;
                            *fresh11 = (*n_0).sub.val.sp;
                            let ref mut fresh12 = (*(n_0 as *mut block_item)).freep;
                            *fresh12 = nextfree[block_id::BLOCK_NODE as i32 as usize]
                                .freep;
                            nextfree[block_id::BLOCK_NODE as i32 as usize].freep = n_0
                                as *mut block_item;
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    68 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    69 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    74 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    75 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    77 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    78 => {
                        let mut n_1: *mut NODE = 0 as *mut NODE;
                        n_1 = (**yyvsp.offset(0 as i32 as isize)).value.nodeval;
                        if (*n_1).flags as u32 & flagvals::STRING as i32 as u32
                            == 0 as i32 as u32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"argument not a string\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                    }
                    79 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    80 => {
                        if find_option((**yyvsp.offset(0 as i32 as isize)).value.sval)
                            < 0 as i32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"option: invalid parameter - `%s'\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        }
                    }
                    81 => {
                        if find_option((**yyvsp.offset(-(2 as i32) as isize)).value.sval)
                            < 0 as i32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"option: invalid parameter - `%s'\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(-(2 as i32) as isize)).value.sval,
                            );
                        }
                    }
                    82 => {
                        let mut n_2: *mut NODE = 0 as *mut NODE;
                        n_2 = lookup((**yyvsp.offset(0 as i32 as isize)).value.sval);
                        if n_2.is_null()
                            || (*n_2).type_0 as u32 != nodevals::Node_func as i32 as u32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"no such function - `%s'\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(0 as i32 as isize)).value.sval,
                            );
                        } else {
                            (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_func;
                            pma_free(
                                (**yyvsp.offset(0 as i32 as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh13 = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .sval;
                            *fresh13 = 0 as *mut i8;
                            let ref mut fresh14 = (**yyvsp.offset(0 as i32 as isize))
                                .value
                                .nodeval;
                            *fresh14 = n_2;
                        }
                    }
                    83 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    88 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    89 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    92 => {
                        want_nodeval = 1 as i32 != 0;
                    }
                    95 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    97 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    99 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    104 => {
                        let mut idx_1: i32 = find_argument(
                            *yyvsp.offset(-(1 as i32) as isize),
                        );
                        if idx_1 < 0 as i32 {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"enable: invalid option - `%s'\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(-(1 as i32) as isize)).value.sval,
                            );
                        } else {
                            pma_free(
                                (**yyvsp.offset(-(1 as i32) as isize)).value.sval
                                    as *mut libc::c_void,
                            );
                            let ref mut fresh15 = (**yyvsp.offset(-(1 as i32) as isize))
                                .value
                                .sval;
                            *fresh15 = 0 as *mut i8;
                            (**yyvsp.offset(-(1 as i32) as isize)).type_0 = argtype::D_argument;
                            (**yyvsp.offset(-(1 as i32) as isize)).value.lval = (*zz_debug_argtab
                                .as_mut_ptr()
                                .offset(idx_1 as isize))
                                .value as i64;
                        }
                    }
                    106 => {
                        (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_array;
                        (**yyvsp.offset(0 as i32 as isize)).a_count = 0 as i32;
                    }
                    107 => {
                        (**yyvsp.offset(-(1 as i32) as isize)).type_0 = argtype::D_array;
                        (**yyvsp.offset(-(1 as i32) as isize)).a_count = num_dim;
                    }
                    117 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    118 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    119 => {
                        let mut a: *mut CMDARG = 0 as *mut CMDARG;
                        a = mk_cmdarg(argtype::D_int);
                        (*a).value.lval = -(1 as i32) as i64;
                        append_cmdarg(a);
                    }
                    126 => {
                        if (**yyvsp.offset(-(2 as i32) as isize)).value.lval
                            > (**yyvsp.offset(0 as i32 as isize)).value.lval
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"invalid range specification: %d - %d\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                                (**yyvsp.offset(-(2 as i32) as isize)).value.lval,
                                (**yyvsp.offset(0 as i32 as isize)).value.lval,
                            );
                        } else {
                            (**yyvsp.offset(-(2 as i32) as isize)).type_0 = argtype::D_range;
                        }
                        yyval = *yyvsp.offset(-(2 as i32) as isize);
                    }
                    127 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    134 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    135 => {
                        yyval = *yyvsp.offset(-(2 as i32) as isize);
                    }
                    137 => {
                        let mut a_0: *mut CMDARG = 0 as *mut CMDARG;
                        let mut subs: *mut NODE = 0 as *mut NODE;
                        let mut count: i32 = 0 as i32;
                        a_0 = *yyvsp.offset(-(1 as i32) as isize);
                        while !a_0.is_null() {
                            count += 1;
                            count;
                            a_0 = (*a_0).next;
                        }
                        subs = concat_args(*yyvsp.offset(-(1 as i32) as isize), count);
                        free_cmdarg((**yyvsp.offset(-(1 as i32) as isize)).next);
                        let ref mut fresh16 = (**yyvsp.offset(-(1 as i32) as isize))
                            .next;
                        *fresh16 = 0 as *mut cmd_argument;
                        (**yyvsp.offset(-(1 as i32) as isize)).type_0 = argtype::D_node;
                        let ref mut fresh17 = (**yyvsp.offset(-(1 as i32) as isize))
                            .value
                            .nodeval;
                        *fresh17 = subs;
                        yyval = *yyvsp.offset(-(1 as i32) as isize);
                    }
                    139 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                        num_dim = 1 as i32;
                    }
                    140 => {
                        yyval = *yyvsp.offset(-(1 as i32) as isize);
                        num_dim += 1;
                        num_dim;
                    }
                    142 => {
                        let mut n_3: *mut NODE = (**yyvsp.offset(0 as i32 as isize))
                            .value
                            .nodeval;
                        if (*n_3).flags as u32 & flagvals::NUMBER as i32 as u32
                            == 0 as i32 as u32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"non-numeric value for field number\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        } else {
                            (**yyvsp.offset(0 as i32 as isize)).type_0 = argtype::D_field;
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    143 => {
                        (**yyvsp.offset(-(1 as i32) as isize)).type_0 = argtype::D_subscript;
                        (**yyvsp.offset(-(1 as i32) as isize)).a_count = num_dim;
                        yyval = *yyvsp.offset(-(1 as i32) as isize);
                    }
                    144 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    145 => {
                        let mut n_4: *mut NODE = (**yyvsp.offset(0 as i32 as isize))
                            .value
                            .nodeval;
                        if (*n_4).flags as u32 & flagvals::NUMBER as i32 as u32
                            == 0 as i32 as u32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"non-numeric value found, numeric expected\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    146 => {
                        let mut n_5: *mut NODE = (**yyvsp.offset(0 as i32 as isize))
                            .value
                            .nodeval;
                        if (*n_5).flags as u32 & flagvals::NUMBER as i32 as u32
                            == 0 as i32 as u32
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"non-numeric value found, numeric expected\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        } else {
                            negate_num(n_5);
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    147 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    148 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    149 => {
                        yyval = 0 as *mut CMDARG;
                    }
                    150 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    151 => {
                        if (**yyvsp.offset(0 as i32 as isize)).value.lval
                            == 0 as i32 as i64
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"non-zero integer value\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    152 => {
                        if (**yyvsp.offset(0 as i32 as isize)).value.lval
                            == 0 as i32 as i64
                        {
                            zzerror(
                                dcgettext(
                                    0 as *const i8,
                                    b"non-zero integer value\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    153 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    154 => {
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    155 => {
                        (**yyvsp.offset(0 as i32 as isize)).value.lval = -(**yyvsp
                            .offset(0 as i32 as isize))
                            .value
                            .lval;
                        yyval = *yyvsp.offset(0 as i32 as isize);
                    }
                    156 => {
                        if !lexptr_begin.is_null() {
                            input_from_tty as i32 != 0
                                && *lexptr_begin.offset(0 as i32 as isize) as i32
                                    != '\0' as i32;
                            pma_free(lexptr_begin as *mut libc::c_void);
                            lexptr_begin = 0 as *mut i8;
                        }
                    }
                    _ => {}
                }
                yyvsp = yyvsp.offset(-(yylen as isize));
                yyssp = yyssp.offset(-(yylen as isize));
                yylen = 0 as i32;
                yyvsp = yyvsp.offset(1);
                *yyvsp = yyval;
                let yylhs: i32 = yyr1[yyn as usize] as i32 - 59 as i32;
                let yyi: i32 = yypgoto[yylhs as usize] as i32 + *yyssp as i32;
                yystate = if 0 as i32 <= yyi && yyi <= 203 as i32
                    && yycheck[yyi as usize] as i32 == *yyssp as i32
                {
                    yytable[yyi as usize] as i32
                } else {
                    yydefgoto[yylhs as usize] as i32
                };
            }
            3309994273773591571 => {
                yyerrstatus = 3 as i32;
                loop {
                    yyn = yypact[yystate as usize] as i32;
                    if !(yyn == -(151 as i32)) {
                        yyn += YYSYMBOL_YYerror as i32;
                        if 0 as i32 <= yyn && yyn <= 203 as i32
                            && yycheck[yyn as usize] as i32 == YYSYMBOL_YYerror as i32
                        {
                            yyn = yytable[yyn as usize] as i32;
                            if (0 as i32) < yyn {
                                break;
                            }
                        }
                    }
                    if yyssp == yyss {
                        current_block = 14500194797246379994;
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
                *yyvsp = zzlval;
                yystate = yyn;
            }
            _ => {}
        }
        yyssp = yyssp.offset(1);
        yyssp;
    }
    match current_block {
        16033749021571576439 => {
            yyresult = 0 as i32;
        }
        14500194797246379994 => {
            yyresult = 1 as i32;
        }
        _ => {
            zzerror(b"memory exhausted\0" as *const u8 as *const i8);
            yyresult = 2 as i32;
        }
    }
    if zzchar != -(2 as i32) {
        yytoken = (if 0 as i32 <= zzchar && zzchar <= 303 as i32 {
            yytranslate[zzchar as usize] as yysymbol_kind_t as i32
        } else {
            YYSYMBOL_YYUNDEF as i32
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const i8,
            yytoken,
            &mut zzlval,
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
unsafe extern "C" fn find_argument(mut arg: *mut CMDARG) -> i32 {
    let mut idx: i32 = 0;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    name = (*arg).value.sval;
    len = strlen(name);
    idx = 0 as i32;
    loop {
        p = zz_debug_argtab[idx as usize].name as *mut i8;
        if p.is_null() {
            break;
        }
        if zz_debug_cmdtab[cmd_idx as usize].type_0 as u32
            == zz_debug_argtab[idx as usize].cmd as u32 && *p as i32 == *name as i32
            && strlen(p) == len && strncmp(p, name, len) == 0 as i32
        {
            return idx;
        }
        idx += 1;
        idx;
    }
    return -(1 as i32);
}
unsafe extern "C" fn concat_args(mut arg: *mut CMDARG, mut count: i32) -> *mut NODE {
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut subsep: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut len: i64 = 0;
    let mut subseplen: i64 = 0;
    let mut i: i32 = 0;
    if count == 1 as i32 {
        n = force_string_fmt((*arg).value.nodeval, CONVFMT, CONVFMTidx);
        return dupnode(n);
    }
    tmp = emalloc_real(
        (count as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
        b"concat_args\0" as *const u8 as *const i8,
        b"tmp\0" as *const u8 as *const i8,
        b"command.y\0" as *const u8 as *const i8,
        1381 as i32,
    ) as *mut *mut NODE;
    subseplen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen as i64;
    subsep = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
    len = -subseplen;
    i = 0 as i32;
    while i < count {
        n = force_string_fmt((*arg).value.nodeval, CONVFMT, CONVFMTidx);
        len = (len as u64)
            .wrapping_add(((*n).sub.val.slen).wrapping_add(subseplen as u64)) as i64
            as i64;
        let ref mut fresh18 = *tmp.offset(i as isize);
        *fresh18 = n;
        arg = (*arg).next;
        i += 1;
        i;
    }
    str = emalloc_real(
        (len + 1 as i32 as i64) as size_t,
        b"concat_args\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"command.y\0" as *const u8 as *const i8,
        1393 as i32,
    ) as *mut i8;
    n = *tmp.offset(0 as i32 as isize);
    memcpy(
        str as *mut libc::c_void,
        (*n).sub.val.sp as *const libc::c_void,
        (*n).sub.val.slen,
    );
    p = str.offset((*n).sub.val.slen as isize);
    i = 1 as i32;
    while i < count {
        if subseplen == 1 as i32 as i64 {
            let fresh19 = p;
            p = p.offset(1);
            *fresh19 = *subsep;
        } else if subseplen > 0 as i32 as i64 {
            memcpy(
                p as *mut libc::c_void,
                subsep as *const libc::c_void,
                subseplen as u64,
            );
            p = p.offset(subseplen as isize);
        }
        n = *tmp.offset(i as isize);
        memcpy(
            p as *mut libc::c_void,
            (*n).sub.val.sp as *const libc::c_void,
            (*n).sub.val.slen,
        );
        p = p.offset((*n).sub.val.slen as isize);
        i += 1;
        i;
    }
    *str.offset(len as isize) = '\0' as i32 as i8;
    pma_free(tmp as *mut libc::c_void);
    return make_str_node(str, len as size_t, 2 as i32);
}
unsafe extern "C" fn find_command(mut token: *const i8, mut toklen: size_t) -> i32 {
    let mut name: *const i8 = 0 as *const i8;
    let mut abrv: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut try_exact: bool = 1 as i32 != 0;
    let mut abrv_match: i32 = -(1 as i32);
    let mut partial_match: i32 = -(1 as i32);
    k = (::core::mem::size_of::<[cmdtoken; 43]>() as u64)
        .wrapping_div(::core::mem::size_of::<cmdtoken>() as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    i = 0 as i32;
    while i < k {
        name = zz_debug_cmdtab[i as usize].name as *mut i8;
        if try_exact as i32 != 0 && *token as i32 == *name as i32
            && toklen == strlen(name) && strncmp(name, token, toklen) == 0 as i32
        {
            return i;
        }
        if *name as i32 > *token as i32 || i == k - 1 as i32 {
            try_exact = 0 as i32 != 0;
        }
        if abrv_match < 0 as i32 {
            abrv = zz_debug_cmdtab[i as usize].abbrvn;
            if *abrv.offset(0 as i32 as isize) as i32
                == *token.offset(0 as i32 as isize) as i32
            {
                if toklen == 1 as i32 as u64 && *abrv.offset(1 as i32 as isize) == 0 {
                    abrv_match = i;
                } else if toklen == 2 as i32 as u64
                    && *abrv.offset(1 as i32 as isize) as i32
                        == *token.offset(1 as i32 as isize) as i32
                {
                    abrv_match = i;
                }
            }
        }
        if !try_exact && abrv_match >= 0 as i32 {
            return abrv_match;
        }
        if partial_match < 0 as i32 {
            if *token as i32 == *name as i32 && toklen < strlen(name)
                && strncmp(name, token, toklen) == 0 as i32
            {
                if (i == k - 1 as i32
                    || strncmp(
                        zz_debug_cmdtab[(i + 1 as i32) as usize].name,
                        token,
                        toklen,
                    ) != 0 as i32)
                    && (i == 0 as i32
                        || strncmp(
                            zz_debug_cmdtab[(i - 1 as i32) as usize].name,
                            token,
                            toklen,
                        ) != 0 as i32)
                {
                    partial_match = i;
                }
            }
        }
        i += 1;
        i;
    }
    return partial_match;
}
#[no_mangle]
pub unsafe extern "C" fn do_help(mut arg: *mut CMDARG, mut cmd: i32) -> i32 {
    let mut i: i32 = 0;
    if arg.is_null() {
        if _setjmp(pager_quit_tag.as_mut_ptr()) == 0 as i32 {
            i = 0 as i32;
            while !(zz_debug_cmdtab[i as usize].name).is_null() {
                gprintf(
                    out_fp,
                    b"%s:\n\0" as *const u8 as *const i8,
                    zz_debug_cmdtab[i as usize].name,
                );
                gprintf(
                    out_fp,
                    b"\t%s\n\0" as *const u8 as *const i8,
                    dcgettext(
                        0 as *const i8,
                        zz_debug_cmdtab[i as usize].help_txt,
                        5 as i32,
                    ),
                );
                i += 1;
                i;
            }
        }
    } else if (*arg).type_0 as u32 == argtype::D_string as i32 as u32 {
        let mut name: *mut i8 = 0 as *mut i8;
        name = (*arg).value.sval;
        i = find_command(name, strlen(name));
        if i >= 0 as i32 {
            fprintf(
                out_fp,
                b"%s\n\0" as *const u8 as *const i8,
                zz_debug_cmdtab[i as usize].help_txt,
            );
            if strcmp(
                zz_debug_cmdtab[i as usize].name,
                b"option\0" as *const u8 as *const i8,
            ) == 0 as i32
            {
                option_help();
            }
        } else {
            fprintf(
                out_fp,
                dcgettext(
                    0 as *const i8,
                    b"undefined command: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                name,
            );
        }
    }
    return 0 as i32;
}