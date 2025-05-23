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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn pma_free(ptr: *mut libc::c_void);
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    static mut IGNORECASE: bool;
    static mut CONVFMT: *const i8;
    static mut CONVFMTidx: i32;
    static mut SUBSEP_node: *mut NODE;
    static mut Nnull_string: *mut NODE;
    static mut Null_field: *mut NODE;
    static mut interpret: Option<unsafe extern "C" fn(*mut INSTRUCTION) -> i32>;
    static mut make_number: Option<unsafe extern "C" fn(libc::c_double) -> *mut NODE>;
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    static mut format_val: Option<
        unsafe extern "C" fn(*const i8, i32, *mut NODE) -> *mut NODE,
    >;
    static mut cmp_numbers: Option<
        unsafe extern "C" fn(*const NODE, *const NODE) -> i32,
    >;
    static str_array_func: array_funcs_t;
    static cint_array_func: array_funcs_t;
    static int_array_func: array_funcs_t;
    static mut nextfree: [block_header; 2];
    static mut do_flags: do_flag_values;
    static mut gawk_mb_cur_max: i32;
    static mut casetable: [i8; 0];
    static mut stack_ptr: *mut STACK_ITEM;
    static mut frame_ptr: *mut NODE;
    static mut stack_top: *mut STACK_ITEM;
    fn r_unref(tmp: *mut NODE);
    fn r_fatal(mesg: *const i8, _: ...);
    fn set_loc(file: *const i8, line: i32);
    fn more_blocks(id: i32) -> *mut libc::c_void;
    fn make_str_node(s: *const i8, len: size_t, flags: i32) -> *mut NODE;
    fn elem_new_to_scalar(n: *mut NODE) -> *mut NODE;
    fn bcfree(_: *mut INSTRUCTION);
    fn POP_CODE() -> *mut INSTRUCTION;
    fn grow_stack() -> *mut STACK_ITEM;
    fn PUSH_CODE(cp: *mut INSTRUCTION);
    fn bcalloc(op: OPCODE, size: i32, srcline: i32) -> *mut INSTRUCTION;
    fn lookup(name: *const i8) -> *mut NODE;
    fn strncasecmpmbs(_: *const u8, _: *const u8, _: size_t) -> i32;
    fn nodetype2str(type_0: NODETYPE) -> *const i8;
    fn flags2str(_: i32) -> *const i8;
    static mut lintfunc: Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
    fn r_dupnode(n: *mut NODE) -> *mut NODE;
    static mut func_table: *mut NODE;
    static mut symbol_table: *mut NODE;
    fn check_symtab_functab(dest: *mut NODE, fname: *const i8, msg: *const i8);
    static mut output_fp: *mut FILE;
    static mut fmt_list: *mut *mut NODE;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
pub type INSTRUCTION = exp_instruction;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum assoc_kind_t {
    ANONE = 0x0,
    AINDEX = 0x1,
    AVALUE = 0x2,
    AINUM = 0x4,
    AISTR = 0x8,
    AVNUM = 0x10,
    AVSTR = 0x20,
    AASC = 0x40,
    ADESC = 0x80,
    ADELETE = 0x100,
}
impl assoc_kind_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            assoc_kind_t::ANONE => 0x0,
            assoc_kind_t::AINDEX => 0x1,
            assoc_kind_t::AVALUE => 0x2,
            assoc_kind_t::AINUM => 0x4,
            assoc_kind_t::AISTR => 0x8,
            assoc_kind_t::AVNUM => 0x10,
            assoc_kind_t::AVSTR => 0x20,
            assoc_kind_t::AASC => 0x40,
            assoc_kind_t::ADESC => 0x80,
            assoc_kind_t::ADELETE => 0x100,
        }
    }
    fn from_libc_c_uint(value: u32) -> assoc_kind_t {
        match value {
            0x0 => assoc_kind_t::ANONE,
            0x1 => assoc_kind_t::AINDEX,
            0x2 => assoc_kind_t::AVALUE,
            0x4 => assoc_kind_t::AINUM,
            0x8 => assoc_kind_t::AISTR,
            0x10 => assoc_kind_t::AVNUM,
            0x20 => assoc_kind_t::AVSTR,
            0x40 => assoc_kind_t::AASC,
            0x80 => assoc_kind_t::ADESC,
            0x100 => assoc_kind_t::ADELETE,
            _ => panic!("Invalid value for assoc_kind_t: {}", value),
        }
    }
}
impl AddAssign<u32> for assoc_kind_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for assoc_kind_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for assoc_kind_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for assoc_kind_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for assoc_kind_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn add(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn sub(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn mul(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn div(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for assoc_kind_t {
    type Output = assoc_kind_t;
    fn rem(self, rhs: u32) -> assoc_kind_t {
        assoc_kind_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type qsort_compfunc = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qsort_funcs {
    pub name: *const i8,
    pub comp_func: qsort_compfunc,
    pub kind: assoc_kind_t,
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
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    return if (*n).flags as u32 & flagvals::NUMCUR as i32 as u32 != 0 as i32 as u32 {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn fixtype(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & (flagvals::NUMCUR as i32 | flagvals::USER_INPUT as i32) as u32
        == flagvals::USER_INPUT as i32 as u32
    {
        return force_number(n);
    }
    if (*n).flags as u32 & flagvals::INTIND as i32 as u32 != 0 as i32 as u32 {
        return force_string_fmt(n, CONVFMT, CONVFMTidx);
    }
    return n;
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
unsafe extern "C" fn in_array(mut a: *mut NODE, mut s: *mut NODE) -> *mut NODE {
    let mut ret: *mut *mut NODE = 0 as *mut *mut NODE;
    ret = ((*(*a).sub.nodep.l.lp).exists).expect("non-null function pointer")(a, s);
    return if !ret.is_null() { *ret } else { 0 as *mut NODE };
}
#[inline]
unsafe extern "C" fn POP_PARAM() -> *mut NODE {
    let fresh1 = stack_ptr;
    stack_ptr = stack_ptr.offset(-1);
    let mut t: *mut NODE = (*fresh1).rptr;
    return if (*t).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        t
    } else {
        force_array(t, 0 as i32 != 0)
    };
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
unsafe extern "C" fn dupnode(mut n: *mut NODE) -> *mut NODE {
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        (*n).valref += 1;
        (*n).valref;
        return n;
    }
    return r_dupnode(n);
}
#[no_mangle]
pub static mut success_node: *mut NODE = 0 as *const NODE as *mut NODE;
static mut SUBSEPlen: size_t = 0;
static mut SUBSEP: *mut i8 = 0 as *const i8 as *mut i8;
static mut indent_char: [i8; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &mut [i8; 5]>(b"    \0")
};
static mut null_array_func: array_funcs_t = unsafe {
    {
        let mut init = array_funcs_t {
            name: b"null\0" as *const u8 as *const i8,
            init: None,
            type_of: None,
            lookup: Some(
                null_lookup
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            exists: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            clear: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            remove: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            list: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            copy: Some(
                null_afunc
                    as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            dump: Some(
                null_dump as unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE,
            ),
            store: None,
        };
        init
    }
};
static mut array_types: [*const array_funcs_t; 10] = [0 as *const array_funcs_t; 10];
static mut num_array_types: i32 = 0 as i32;
unsafe extern "C" fn register_array_func(mut afunc: *const array_funcs_t) -> i32 {
    if !afunc.is_null() && num_array_types < 10 as i32 {
        if afunc != &str_array_func as *const array_funcs_t
            && ((*afunc).type_of).is_none()
        {
            return 0 as i32;
        }
        let fresh2 = num_array_types;
        num_array_types = num_array_types + 1;
        array_types[fresh2 as usize] = afunc;
        if ((*afunc).init).is_some() {
            (Some(((*afunc).init).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(0 as *mut exp_node, 0 as *mut exp_node);
        }
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn array_init() {
    register_array_func(&str_array_func);
    if do_flags as u32 & do_flag_values::DO_MPFR as i32 as u32 == 0 {
        register_array_func(&int_array_func);
        register_array_func(&cint_array_func);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_array() -> *mut NODE {
    let mut array: *mut NODE = 0 as *mut NODE;
    array = nextfree[block_id::BLOCK_NODE as i32 as usize].freep as *mut NODE;
    if !array.is_null() {
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = (*(array
            as *mut block_item))
            .freep;
    } else {
        array = more_blocks(block_id::BLOCK_NODE as i32) as *mut NODE;
    };
    memset(
        array as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<NODE>() as u64,
    );
    (*array).type_0 = nodevals::Node_var_array;
    (*array).sub.nodep.l.lp = &null_array_func;
    return array;
}
#[no_mangle]
pub unsafe extern "C" fn null_array(mut symbol: *mut NODE) {
    (*symbol).type_0 = nodevals::Node_var_array;
    (*symbol).sub.nodep.l.lp = &null_array_func;
    (*symbol).sub.nodep.r.bv = 0 as *mut *mut BUCKET;
    (*symbol).sub.nodep.reflags = reflagvals::from_libc_c_uint(0 as u32);
    (*symbol).sub.nodep.cnt = 0 as i32 as u64;
    (*symbol).sub.nodep.reserved = 0 as i32 as size_t;
    (*symbol).flags = flagvals::from_libc_c_uint(0 as u32);
}
unsafe extern "C" fn null_lookup(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    let mut i: i32 = 0;
    let mut afunc: *const array_funcs_t = 0 as *const array_funcs_t;
    i = num_array_types - 1 as i32;
    while i >= 1 as i32 {
        afunc = array_types[i as usize];
        if !(((*afunc).type_of).expect("non-null function pointer")(symbol, subs))
            .is_null()
        {
            break;
        }
        i -= 1;
        i;
    }
    if i == 0 as i32 || afunc.is_null() {
        afunc = array_types[0 as i32 as usize];
    }
    (*symbol).sub.nodep.l.lp = afunc;
    return ((*(*symbol).sub.nodep.l.lp).lookup)
        .expect("non-null function pointer")(symbol, subs);
}
#[no_mangle]
pub unsafe extern "C" fn null_afunc(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    return 0 as *mut *mut NODE;
}
unsafe extern "C" fn null_dump(
    mut symbol: *mut NODE,
    mut subs: *mut NODE,
) -> *mut *mut NODE {
    fprintf(
        output_fp,
        b"array `%s' is empty\n\0" as *const u8 as *const i8,
        array_vname(symbol),
    );
    return 0 as *mut *mut NODE;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_copy(
    mut symbol: *mut NODE,
    mut newsymb: *mut NODE,
) -> *mut NODE {
    ((*(*newsymb).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(newsymb, 0 as *mut exp_node);
    ((*(*symbol).sub.nodep.l.lp).copy)
        .expect("non-null function pointer")(symbol, newsymb);
    (*newsymb).sub.nodep.l.lp = (*symbol).sub.nodep.l.lp;
    (*newsymb).flags = (*symbol).flags;
    return newsymb;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_dump(mut symbol: *mut NODE, mut ndump: *mut NODE) {
    if ((*(*symbol).sub.nodep.l.lp).dump).is_some() {
        ((*(*symbol).sub.nodep.l.lp).dump)
            .expect("non-null function pointer")(symbol, ndump);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_aname(mut symbol: *const NODE) -> *const i8 {
    static mut aname: *mut i8 = 0 as *const i8 as *mut i8;
    static mut alen: size_t = 0;
    static mut max_alen: size_t = 0;
    if !((*symbol).sub.nodep.x.extra).is_null() {
        let mut slen: size_t = 0;
        make_aname((*symbol).sub.nodep.x.extra);
        slen = strlen((*symbol).sub.nodep.name);
        if alen.wrapping_add(slen).wrapping_add(4 as i32 as u64) > max_alen {
            max_alen = alen
                .wrapping_add(slen)
                .wrapping_add(4 as i32 as u64)
                .wrapping_add(256 as i32 as u64);
            aname = erealloc_real(
                aname as *mut libc::c_void,
                max_alen
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                b"make_aname\0" as *const u8 as *const i8,
                b"aname\0" as *const u8 as *const i8,
                b"array.c\0" as *const u8 as *const i8,
                213 as i32,
            ) as *mut i8;
        }
        alen = (alen as u64)
            .wrapping_add(
                sprintf(
                    aname.offset(alen as isize),
                    b"[\"%s\"]\0" as *const u8 as *const i8,
                    (*symbol).sub.nodep.name,
                ) as u64,
            ) as size_t as size_t;
    } else {
        alen = strlen((*symbol).sub.nodep.name);
        if aname.is_null() {
            max_alen = alen.wrapping_add(256 as i32 as u64);
            aname = emalloc_real(
                max_alen
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                b"make_aname\0" as *const u8 as *const i8,
                b"aname\0" as *const u8 as *const i8,
                b"array.c\0" as *const u8 as *const i8,
                220 as i32,
            ) as *mut i8;
        } else if alen > max_alen {
            max_alen = alen.wrapping_add(256 as i32 as u64);
            aname = erealloc_real(
                aname as *mut libc::c_void,
                max_alen
                    .wrapping_add(1 as i32 as u64)
                    .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                b"make_aname\0" as *const u8 as *const i8,
                b"aname\0" as *const u8 as *const i8,
                b"array.c\0" as *const u8 as *const i8,
                223 as i32,
            ) as *mut i8;
        }
        memcpy(
            aname as *mut libc::c_void,
            (*symbol).sub.nodep.name as *const libc::c_void,
            alen.wrapping_add(1 as i32 as u64),
        );
    }
    return aname;
}
#[no_mangle]
pub unsafe extern "C" fn array_vname(mut symbol: *const NODE) -> *const i8 {
    static mut message: *mut i8 = 0 as *const i8 as *mut i8;
    static mut msglen: size_t = 0 as i32 as size_t;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut n: i32 = 0;
    let mut save_symbol: *const NODE = symbol;
    let mut from: *const i8 = dcgettext(
        0 as *const i8,
        b"from %s\0" as *const u8 as *const i8,
        5 as i32,
    );
    let mut aname: *const i8 = 0 as *const i8;
    if (*symbol).type_0 as u32 != nodevals::Node_array_ref as i32 as u32
        || (*(*symbol).sub.nodep.l.lptr).type_0 as u32
            != nodevals::Node_var_array as i32 as u32
    {
        if (*symbol).type_0 as u32 != nodevals::Node_var_array as i32 as u32
            || ((*symbol).sub.nodep.x.extra).is_null()
        {
            return (*symbol).sub.nodep.name;
        }
        return make_aname(symbol);
    }
    len = 2 as i32 as size_t;
    n = 0 as i32;
    while (*symbol).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
        len = (len as u64).wrapping_add(strlen((*symbol).sub.nodep.name)) as size_t
            as size_t;
        n += 1;
        n;
        symbol = (*symbol).sub.nodep.r.rptr;
    }
    if ((*symbol).sub.nodep.x.extra).is_null() {
        aname = (*symbol).sub.nodep.name;
    } else {
        aname = make_aname(symbol);
    }
    len = (len as u64).wrapping_add(strlen(aname)) as size_t as size_t;
    len = (len as u64).wrapping_add((n as u64).wrapping_mul(strlen(from))) as size_t
        as size_t;
    if message.is_null() {
        message = emalloc_real(
            len,
            b"array_vname\0" as *const u8 as *const i8,
            b"message\0" as *const u8 as *const i8,
            b"array.c\0" as *const u8 as *const i8,
            285 as i32,
        ) as *mut i8;
        msglen = len;
    } else if len > msglen {
        message = erealloc_real(
            message as *mut libc::c_void,
            len,
            b"array_vname\0" as *const u8 as *const i8,
            b"message\0" as *const u8 as *const i8,
            b"array.c\0" as *const u8 as *const i8,
            288 as i32,
        ) as *mut i8;
        msglen = len;
    }
    symbol = save_symbol;
    s = message;
    s = s
        .offset(
            sprintf(s, b"%s (\0" as *const u8 as *const i8, (*symbol).sub.nodep.name)
                as isize,
        );
    loop {
        symbol = (*symbol).sub.nodep.r.rptr;
        if (*symbol).type_0 as u32 != nodevals::Node_array_ref as i32 as u32 {
            break;
        }
        s = s.offset(sprintf(s, from, (*symbol).sub.nodep.name) as isize);
        s = s.offset(sprintf(s, b", \0" as *const u8 as *const i8) as isize);
    }
    s = s.offset(sprintf(s, from, aname) as isize);
    strcpy(s, b")\0" as *const u8 as *const i8);
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn force_array(
    mut symbol: *mut NODE,
    mut canfatal: bool,
) -> *mut NODE {
    let mut save_symbol: *mut NODE = symbol;
    let mut isparam: bool = 0 as i32 != 0;
    if (*symbol).type_0 as u32 == nodevals::Node_param_list as i32 as u32 {
        symbol = *((*frame_ptr).sub.nodep.r.av)
            .offset((*symbol).sub.nodep.l.ll as isize);
        save_symbol = symbol;
        isparam = 1 as i32 != 0;
        if (*symbol).type_0 as u32 == nodevals::Node_array_ref as i32 as u32 {
            symbol = (*symbol).sub.nodep.l.lptr;
        }
    }
    let mut current_block_17: u64;
    match (*symbol).type_0 as u32 {
        7 => {
            pma_free((*symbol).sub.val.sp as *mut libc::c_void);
            (*symbol).sub.val.sp = 0 as *mut i8;
            (*symbol).sub.val.slen = 0 as i32 as size_t;
            current_block_17 = 10124197035094333396;
        }
        6 => {
            current_block_17 = 10124197035094333396;
        }
        5 => {
            current_block_17 = 5689001924483802034;
        }
        12 | _ => {
            if canfatal {
                if (*symbol).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"array.c\0" as *const u8 as *const i8, 355 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use a scalar value as array\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                if isparam {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"array.c\0" as *const u8 as *const i8, 357 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use scalar parameter `%s' as an array\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (*save_symbol).sub.nodep.name,
                    );
                } else {
                    (set_loc
                        as unsafe extern "C" fn(
                            *const i8,
                            i32,
                        ) -> ())(b"array.c\0" as *const u8 as *const i8, 360 as i32);
                    (Some(
                        (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        dcgettext(
                            0 as *const i8,
                            b"attempt to use scalar `%s' as an array\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        (*save_symbol).sub.nodep.name,
                    );
                }
                current_block_17 = 5689001924483802034;
            } else {
                current_block_17 = 5689001924483802034;
            }
        }
    }
    match current_block_17 {
        10124197035094333396 => {
            (*symbol).sub.nodep.rn = 0 as *mut exp_node;
            null_array(symbol);
            (*symbol).sub.nodep.x.extra = 0 as *mut exp_node;
        }
        _ => {}
    }
    return symbol;
}
#[no_mangle]
pub unsafe extern "C" fn set_SUBSEP() {
    (*SUBSEP_node).sub.nodep.l.lptr = force_string_fmt(
        (*SUBSEP_node).sub.nodep.l.lptr,
        CONVFMT,
        CONVFMTidx,
    );
    SUBSEP = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.sp;
    SUBSEPlen = (*(*SUBSEP_node).sub.nodep.l.lptr).sub.val.slen;
}
#[no_mangle]
pub unsafe extern "C" fn concat_exp(mut nargs: i32, mut do_subsep: bool) -> *mut NODE {
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut subseplen: size_t = 0 as i32 as size_t;
    let mut i: i32 = 0;
    extern "C" {
        static mut args_array: *mut *mut NODE;
    }
    if nargs == 1 as i32 {
        return force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    }
    if do_subsep {
        subseplen = SUBSEPlen;
    }
    len = 0 as i32 as size_t;
    i = 1 as i32;
    while i <= nargs {
        r = (*stack_ptr).rptr;
        if (*r).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
            loop {
                i -= 1;
                if !(i > 0 as i32) {
                    break;
                }
                DEREF(*args_array.offset(i as isize));
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 407 as i32);
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
                array_vname(r),
            );
        }
        r = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
        let ref mut fresh3 = *args_array.offset(i as isize);
        *fresh3 = r;
        len = (len as u64).wrapping_add((*r).sub.val.slen) as size_t as size_t;
        i += 1;
        i;
    }
    len = (len as u64).wrapping_add(((nargs - 1 as i32) as u64).wrapping_mul(subseplen))
        as size_t as size_t;
    str = emalloc_real(
        len.wrapping_add(1 as i32 as u64),
        b"concat_exp\0" as *const u8 as *const i8,
        b"str\0" as *const u8 as *const i8,
        b"array.c\0" as *const u8 as *const i8,
        415 as i32,
    ) as *mut i8;
    r = *args_array.offset(nargs as isize);
    memcpy(
        str as *mut libc::c_void,
        (*r).sub.val.sp as *const libc::c_void,
        (*r).sub.val.slen,
    );
    s = str.offset((*r).sub.val.slen as isize);
    DEREF(r);
    i = nargs - 1 as i32;
    while i > 0 as i32 {
        if subseplen == 1 as i32 as u64 {
            let fresh4 = s;
            s = s.offset(1);
            *fresh4 = *SUBSEP;
        } else if subseplen > 0 as i32 as u64 {
            memcpy(s as *mut libc::c_void, SUBSEP as *const libc::c_void, subseplen);
            s = s.offset(subseplen as isize);
        }
        r = *args_array.offset(i as isize);
        memcpy(
            s as *mut libc::c_void,
            (*r).sub.val.sp as *const libc::c_void,
            (*r).sub.val.slen,
        );
        s = s.offset((*r).sub.val.slen as isize);
        DEREF(r);
        i -= 1;
        i;
    }
    return make_str_node(str, len, 2 as i32);
}
unsafe extern "C" fn adjust_fcall_stack(mut symbol: *mut NODE, mut nsubs: i32) {
    let mut func: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut n: *mut NODE = 0 as *mut NODE;
    let mut sp: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut pcount: i32 = 0;
    func = (*frame_ptr).sub.nodep.x.extra;
    if func.is_null() {
        return;
    }
    pcount = (*func).sub.nodep.l.ll as i32;
    sp = (*frame_ptr).sub.nodep.r.av;
    while pcount > 0 as i32 {
        let fresh5 = sp;
        sp = sp.offset(1);
        r = *fresh5;
        if !((*r).type_0 as u32 != nodevals::Node_array_ref as i32 as u32
            || (*(*r).sub.nodep.l.lptr).type_0 as u32
                != nodevals::Node_var_array as i32 as u32)
        {
            n = (*r).sub.nodep.l.lptr;
            if n == symbol && !((*symbol).sub.nodep.x.extra).is_null()
                && nsubs > 0 as i32
            {
                null_array(r);
                (*r).sub.nodep.x.extra = 0 as *mut exp_node;
            } else {
                n = (*n).sub.nodep.x.extra;
                while !n.is_null() {
                    if n == symbol {
                        null_array(r);
                        (*r).sub.nodep.x.extra = 0 as *mut exp_node;
                        break;
                    } else {
                        n = (*n).sub.nodep.x.extra;
                    }
                }
            }
        }
        pcount -= 1;
        pcount;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_delete(mut symbol: *mut NODE, mut nsubs: i32) {
    let mut val: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut i: i32 = 0;
    val = 0 as *mut NODE;
    subs = val;
    if nsubs == 0 as i32 {
        adjust_fcall_stack(symbol, 0 as i32);
        ((*(*symbol).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
        return;
    }
    i = nsubs;
    while i > 0 as i32 {
        subs = (*stack_ptr.offset(-((i - 1 as i32) as isize))).rptr;
        if (*subs).type_0 as u32 != nodevals::Node_val as i32 as u32 {
            loop {
                let mut s: *mut NODE = (*stack_ptr.offset(-((i - 1 as i32) as isize)))
                    .rptr;
                if (*s).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    force_string_fmt(s, CONVFMT, CONVFMTidx);
                    DEREF(s);
                }
                i -= 1;
                if !(i > 0 as i32) {
                    break;
                }
            }
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 574 as i32);
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
                array_vname(subs),
            );
        }
        val = in_array(symbol, subs);
        if val.is_null() {
            if do_flags as u32
                & (do_flag_values::DO_LINT_INVALID as i32
                    | do_flag_values::DO_LINT_ALL as i32) as u32 != 0
            {
                subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"array.c\0" as *const u8 as *const i8, 581 as i32);
                (Some(lintfunc.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"delete: index `%.*s' not in array `%s'\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    (*subs).sub.val.slen as i32,
                    (*subs).sub.val.sp,
                    array_vname(symbol),
                );
            }
            loop {
                let mut s_0: *mut NODE = (*stack_ptr.offset(-((i - 1 as i32) as isize)))
                    .rptr;
                if (*s_0).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                    force_string_fmt(s_0, CONVFMT, CONVFMTidx);
                    DEREF(s_0);
                }
                i -= 1;
                if !(i > 0 as i32) {
                    break;
                }
            }
            return;
        }
        if i > 1 as i32 {
            if (*val).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
                loop {
                    let mut s_1: *mut NODE = (*stack_ptr
                        .offset(-((i - 1 as i32) as isize)))
                        .rptr;
                    if (*s_1).type_0 as u32 == nodevals::Node_val as i32 as u32 {
                        force_string_fmt(s_1, CONVFMT, CONVFMTidx);
                        DEREF(s_1);
                    }
                    i -= 1;
                    if !(i > 0 as i32) {
                        break;
                    }
                }
                subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"array.c\0" as *const u8 as *const i8, 595 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"attempt to use scalar `%s[\"%.*s\"]' as an array\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    array_vname(symbol),
                    (*subs).sub.val.slen as i32,
                    (*subs).sub.val.sp,
                );
            }
            symbol = val;
            DEREF(subs);
        }
        i -= 1;
        i;
    }
    if (*val).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        adjust_fcall_stack(val, nsubs);
        ((*(*val).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(val, 0 as *mut exp_node);
        pma_free((*val).sub.nodep.name as *mut libc::c_void);
        let ref mut fresh6 = (*(val as *mut block_item)).freep;
        *fresh6 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = val as *mut block_item;
    } else {
        unref(val);
    }
    ((*(*symbol).sub.nodep.l.lp).remove)
        .expect("non-null function pointer")(symbol, subs);
    DEREF(subs);
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        null_array(symbol);
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_delete_loop(mut symbol: *mut NODE, mut lhs: *mut *mut NODE) {
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut akind: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut i8,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: reflagvals::from_libc_c_uint(0 as u32),
            },
        },
        type_0: nodevals::Node_illegal,
        flags: flagvals::from_libc_c_uint(0 as u32),
        valref: 0,
    };
    akind.flags = flagvals::from_libc_c_uint(
        (assoc_kind_t::AINDEX as i32 | assoc_kind_t::ADELETE as i32) as u32,
    );
    list = ((*(*symbol).sub.nodep.l.lp).list)
        .expect("non-null function pointer")(symbol, &mut akind);
    if (*symbol).sub.nodep.reflags as u32 == 0 as i32 as u32 {
        return;
    }
    unref(*lhs);
    *lhs = *list.offset(0 as i32 as isize);
    pma_free(list as *mut libc::c_void);
    adjust_fcall_stack(symbol, 0 as i32);
    ((*(*symbol).sub.nodep.l.lp).clear)
        .expect("non-null function pointer")(symbol, 0 as *mut exp_node);
}
unsafe extern "C" fn value_info(mut n: *mut NODE) {
    if n == Nnull_string || n == Null_field {
        fprintf(output_fp, b"<(null)>\0" as *const u8 as *const i8);
        return;
    }
    if (*n).flags as u32 & (flagvals::STRING as i32 | flagvals::STRCUR as i32) as u32
        != 0 as i32 as u32
    {
        fprintf(output_fp, b"<\0" as *const u8 as *const i8);
        fprintf(
            output_fp,
            b"\"%.*s\"\0" as *const u8 as *const i8,
            (*n).sub.val.slen as i32,
            (*n).sub.val.sp,
        );
        if (*n).flags as u32 & (flagvals::NUMBER as i32 | flagvals::NUMCUR as i32) as u32
            != 0 as i32 as u32
        {
            fprintf(
                output_fp,
                b":%.*g\0" as *const u8 as *const i8,
                -(1 as i32),
                (*n).sub.val.fltnum,
            );
        }
        fprintf(output_fp, b">\0" as *const u8 as *const i8);
    } else {
        fprintf(
            output_fp,
            b"<%.*g>\0" as *const u8 as *const i8,
            -(1 as i32),
            (*n).sub.val.fltnum,
        );
    }
    fprintf(output_fp, b":%s\0" as *const u8 as *const i8, flags2str((*n).flags as i32));
    if (*n).flags as u32 & flagvals::MALLOC as i32 as u32 != 0 as i32 as u32 {
        fprintf(output_fp, b":%ld\0" as *const u8 as *const i8, (*n).valref);
    } else {
        fprintf(output_fp, b":\0" as *const u8 as *const i8);
    }
    if (*n).flags as u32 & (flagvals::STRING as i32 | flagvals::STRCUR as i32) as u32
        == flagvals::STRCUR as i32 as u32
    {
        let mut len: size_t = 0;
        fprintf(output_fp, b"][\0" as *const u8 as *const i8);
        fprintf(output_fp, b"stfmt=%d, \0" as *const u8 as *const i8, (*n).sub.val.idx);
        len = (**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.slen;
        *((**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.sp)
            .offset(len as isize) = '\0' as i32 as i8;
        fprintf(
            output_fp,
            b"FMT=\"%s\"\0" as *const u8 as *const i8,
            if (*n).sub.val.idx == -(1 as i32) {
                b"<unused>\0" as *const u8 as *const i8
            } else {
                (**fmt_list.offset((*n).sub.val.idx as isize)).sub.val.sp
            },
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn indent(mut indent_level: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < indent_level {
        fprintf(output_fp, b"%s\0" as *const u8 as *const i8, indent_char.as_mut_ptr());
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn assoc_info(
    mut subs: *mut NODE,
    mut val: *mut NODE,
    mut ndump: *mut NODE,
    mut aname: *const i8,
) {
    let mut indent_level: i32 = (*ndump).sub.nodep.x.xl as i32;
    indent_level += 1;
    indent_level;
    indent(indent_level);
    fprintf(output_fp, b"I: [%s:\0" as *const u8 as *const i8, aname);
    if (*subs).flags as u32
        & (flagvals::MPFN as i32 | flagvals::MPZN as i32 | flagvals::INTIND as i32)
            as u32 == flagvals::INTIND as i32 as u32
    {
        fprintf(
            output_fp,
            b"<%ld>\0" as *const u8 as *const i8,
            (*subs).sub.val.fltnum as i64,
        );
    } else {
        value_info(subs);
    }
    fprintf(output_fp, b"]\n\0" as *const u8 as *const i8);
    indent(indent_level);
    match (*val).type_0 as u32 {
        1 => {
            fprintf(output_fp, b"V: [scalar: \0" as *const u8 as *const i8);
            value_info(val);
        }
        4 => {
            fprintf(output_fp, b"V: [scalar: \0" as *const u8 as *const i8);
            value_info((*val).sub.nodep.l.lptr);
        }
        5 => {
            fprintf(output_fp, b"V: [\0" as *const u8 as *const i8);
            (*ndump).sub.nodep.x.xl += 1;
            (*ndump).sub.nodep.x.xl;
            (*ndump).sub.nodep.l.ll -= 1;
            (*ndump).sub.nodep.l.ll;
            assoc_dump(val, ndump);
            (*ndump).sub.nodep.l.ll += 1;
            (*ndump).sub.nodep.l.ll;
            (*ndump).sub.nodep.x.xl -= 1;
            (*ndump).sub.nodep.x.xl;
            indent(indent_level);
        }
        9 => {
            fprintf(output_fp, b"V: [user_defined_function\0" as *const u8 as *const i8);
        }
        10 => {
            fprintf(output_fp, b"V: [external_function\0" as *const u8 as *const i8);
        }
        11 => {
            fprintf(output_fp, b"V: [builtin_function\0" as *const u8 as *const i8);
        }
        _ => {
            r_fatal(
                b"internal error: file %s, line %d: unexpected node type %s\0"
                    as *const u8 as *const i8,
                b"array.c\0" as *const u8 as *const i8,
                779 as i32,
                nodetype2str((*val).type_0),
            );
        }
    }
    fprintf(output_fp, b"]\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn do_adump(mut nargs: i32) -> *mut NODE {
    let mut symbol: *mut NODE = 0 as *mut NODE;
    let mut tmp: *mut NODE = 0 as *mut NODE;
    static mut ndump: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut i8,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: reflagvals::from_libc_c_uint(0 as u32),
            },
        },
        type_0: nodevals::Node_illegal,
        flags: flagvals::from_libc_c_uint(0 as u32),
        valref: 0,
    };
    let mut depth: i64 = 0 as i32 as i64;
    if nargs == 2 as i32 {
        tmp = force_number(POP_SCALAR());
        depth = (*tmp).sub.val.fltnum as i64;
        DEREF(tmp);
    }
    symbol = POP_PARAM();
    if (*symbol).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"array.c\0" as *const u8 as *const i8, 808 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: first argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"adump\0" as *const u8 as *const i8,
        );
    }
    ndump.type_0 = nodevals::Node_dump_array;
    ndump.sub.nodep.l.ll = depth;
    ndump.sub.nodep.x.xl = 0 as i32 as i64;
    assoc_dump(symbol, &mut ndump);
    return make_number.expect("non-null function pointer")(0 as i32 as libc::c_double);
}
unsafe extern "C" fn asort_actual(
    mut nargs: i32,
    mut ctxt: sort_context_t,
) -> *mut NODE {
    let mut array: *mut NODE = 0 as *mut NODE;
    let mut dest: *mut NODE = 0 as *mut NODE;
    let mut result: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut subs: *mut NODE = 0 as *mut NODE;
    let mut s: *mut NODE = 0 as *mut NODE;
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut ptr: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut num_elems: u64 = 0;
    let mut i: u64 = 0;
    let mut sort_str: *const i8 = 0 as *const i8;
    let mut save: i8 = 0;
    let mut name: *const i8 = if ctxt as u32 == sort_context_t::ASORT as i32 as u32 {
        b"asort\0" as *const u8 as *const i8
    } else {
        b"asorti\0" as *const u8 as *const i8
    };
    if nargs == 3 as i32 {
        s = force_string_fmt(POP_SCALAR(), CONVFMT, CONVFMTidx);
    } else {
        s = dupnode(Nnull_string);
    }
    s = force_string_fmt(s, CONVFMT, CONVFMTidx);
    sort_str = (*s).sub.val.sp;
    save = *((*s).sub.val.sp).offset((*s).sub.val.slen as isize);
    *((*s).sub.val.sp).offset((*s).sub.val.slen as isize) = '\0' as i32 as i8;
    if (*s).sub.val.slen == 0 as i32 as u64 {
        if ctxt as u32 == sort_context_t::ASORT as i32 as u32 {
            sort_str = b"@val_type_asc\0" as *const u8 as *const i8;
        } else {
            sort_str = b"@ind_str_asc\0" as *const u8 as *const i8;
        }
    }
    if nargs >= 2 as i32 {
        dest = POP_PARAM();
        if (*dest).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 850 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"%s: second argument is not an array\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                name,
            );
        }
        check_symtab_functab(
            dest,
            name,
            dcgettext(
                0 as *const i8,
                b"%s: cannot use %s as second argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    array = POP_PARAM();
    if (*array).type_0 as u32 != nodevals::Node_var_array as i32 as u32 {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"array.c\0" as *const u8 as *const i8, 858 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: first argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    } else if array == symbol_table && dest.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"array.c\0" as *const u8 as *const i8, 861 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: first argument cannot be SYMTAB without a second argument\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    } else if array == func_table && dest.is_null() {
        (set_loc
            as unsafe extern "C" fn(
                *const i8,
                i32,
            ) -> ())(b"array.c\0" as *const u8 as *const i8, 863 as i32);
        (Some(
            (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                .expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(
            dcgettext(
                0 as *const i8,
                b"%s: first argument cannot be FUNCTAB without a second argument\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
    }
    if !dest.is_null() {
        static mut warned: bool = 0 as i32 != 0;
        if nargs == 2 as i32 && array == dest && !warned {
            warned = 1 as i32 != 0;
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 870 as i32);
            (Some(lintfunc.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"asort/asorti: using the same array as source and destination without a third argument is silly.\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        r = (*dest).sub.nodep.x.extra;
        while !r.is_null() {
            if r == array {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"array.c\0" as *const u8 as *const i8, 875 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"%s: cannot use a subarray of first argument for second argument\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
            }
            r = (*r).sub.nodep.x.extra;
        }
        r = (*array).sub.nodep.x.extra;
        while !r.is_null() {
            if r == dest {
                (set_loc
                    as unsafe extern "C" fn(
                        *const i8,
                        i32,
                    ) -> ())(b"array.c\0" as *const u8 as *const i8, 880 as i32);
                (Some(
                    (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    dcgettext(
                        0 as *const i8,
                        b"%s: cannot use a subarray of second argument for first argument\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    name,
                );
            }
            r = (*r).sub.nodep.x.extra;
        }
    }
    list = assoc_list(array, sort_str, ctxt);
    *((*s).sub.val.sp).offset((*s).sub.val.slen as isize) = save;
    DEREF(s);
    num_elems = (*array).sub.nodep.reflags as u64;
    if num_elems == 0 as i32 as u64 || list.is_null() {
        if !dest.is_null() && dest != array {
            ((*(*dest).sub.nodep.l.lp).clear)
                .expect("non-null function pointer")(dest, 0 as *mut exp_node);
        }
        if !list.is_null() {
            pma_free(list as *mut libc::c_void);
        }
        return make_number
            .expect("non-null function pointer")(0 as i32 as libc::c_double);
    }
    if !dest.is_null() && dest != array {
        ((*(*dest).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(dest, 0 as *mut exp_node);
        result = dest;
    } else {
        result = make_array();
        (*result).sub.nodep.name = (*array).sub.nodep.name;
        (*result).sub.nodep.x.extra = (*array).sub.nodep.x.extra;
    }
    if ctxt as u32 == sort_context_t::ASORTI as i32 as u32 {
        i = 1 as i32 as u64;
        ptr = list;
        while i <= num_elems {
            subs = make_number.expect("non-null function pointer")(i as libc::c_double);
            assoc_set(result, subs, *ptr);
            i = i.wrapping_add(1);
            i;
            ptr = ptr.offset(2 as i32 as isize);
        }
    } else {
        i = 1 as i32 as u64;
        ptr = list;
        while i <= num_elems {
            subs = make_number.expect("non-null function pointer")(i as libc::c_double);
            let fresh7 = ptr;
            ptr = ptr.offset(1);
            r = *fresh7;
            unref(r);
            let fresh8 = ptr;
            ptr = ptr.offset(1);
            r = *fresh8;
            let mut value: *mut NODE = 0 as *mut NODE;
            match (*r).type_0 as u32 {
                1 => {
                    value = dupnode(r);
                }
                4 => {
                    value = dupnode((*r).sub.nodep.l.lptr);
                }
                6 | 7 => {
                    value = dupnode(Nnull_string);
                }
                11 | 9 | 10 => {
                    value = make_str_node(
                        (*r).sub.nodep.name,
                        strlen((*r).sub.nodep.name),
                        0 as i32,
                    );
                }
                5 => {
                    let mut arr: *mut NODE = 0 as *mut NODE;
                    arr = make_array();
                    subs = force_string_fmt(subs, CONVFMT, CONVFMTidx);
                    (*arr).sub.nodep.name = (*subs).sub.val.sp;
                    *((*arr).sub.nodep.name).offset((*subs).sub.val.slen as isize) = '\0'
                        as i32 as i8;
                    (*subs).sub.val.sp = 0 as *mut i8;
                    (*subs).flags = ::core::mem::transmute::<
                        u32,
                        flagvals,
                    >((*subs).flags as u32 & !(flagvals::STRCUR as i32) as u32);
                    (*arr).sub.nodep.x.extra = array;
                    value = assoc_copy(r, arr);
                }
                _ => {
                    r_fatal(
                        b"internal error: file %s, line %d: asort_actual: got unexpected type %s\0"
                            as *const u8 as *const i8,
                        b"array.c\0" as *const u8 as *const i8,
                        972 as i32,
                        nodetype2str((*r).type_0),
                    );
                }
            }
            assoc_set(result, subs, value);
            i = i.wrapping_add(1);
            i;
        }
    }
    pma_free(list as *mut libc::c_void);
    if result != dest {
        ((*(*array).sub.nodep.l.lp).clear)
            .expect("non-null function pointer")(array, 0 as *mut exp_node);
        *array = *result;
        let ref mut fresh9 = (*(result as *mut block_item)).freep;
        *fresh9 = nextfree[block_id::BLOCK_NODE as i32 as usize].freep;
        nextfree[block_id::BLOCK_NODE as i32 as usize].freep = result as *mut block_item;
    }
    return make_number.expect("non-null function pointer")(num_elems as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn do_asort(mut nargs: i32) -> *mut NODE {
    return asort_actual(nargs, sort_context_t::ASORT);
}
#[no_mangle]
pub unsafe extern "C" fn do_asorti(mut nargs: i32) -> *mut NODE {
    return asort_actual(nargs, sort_context_t::ASORTI);
}
unsafe extern "C" fn cmp_strings(mut n1: *const NODE, mut n2: *const NODE) -> i32 {
    let mut s1: *mut i8 = 0 as *mut i8;
    let mut s2: *mut i8 = 0 as *mut i8;
    let mut len1: size_t = 0;
    let mut len2: size_t = 0;
    let mut ret: i32 = 0;
    s1 = (*n1).sub.val.sp;
    len1 = (*n1).sub.val.slen;
    s2 = (*n2).sub.val.sp;
    len2 = (*n2).sub.val.slen;
    if len1 == 0 as i32 as u64 {
        return if len2 == 0 as i32 as u64 { 0 as i32 } else { -(1 as i32) };
    }
    if len2 == 0 as i32 as u64 {
        return 1 as i32;
    }
    let lmin: size_t = if len1 < len2 { len1 } else { len2 };
    if IGNORECASE {
        let mut cp1: *const u8 = s1 as *const u8;
        let mut cp2: *const u8 = s2 as *const u8;
        if gawk_mb_cur_max > 1 as i32 {
            ret = strncasecmpmbs(cp1, cp2, lmin);
        } else {
            let mut count: size_t = lmin;
            ret = 0 as i32;
            loop {
                let fresh10 = count;
                count = count.wrapping_sub(1);
                if !(fresh10 > 0 as i32 as u64 && ret == 0 as i32) {
                    break;
                }
                ret = *casetable.as_mut_ptr().offset(*cp1 as isize) as i32
                    - *casetable.as_mut_ptr().offset(*cp2 as isize) as i32;
                cp1 = cp1.offset(1);
                cp1;
                cp2 = cp2.offset(1);
                cp2;
            }
        }
        if ret != 0 as i32 {
            return ret;
        }
    }
    ret = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, lmin);
    if ret != 0 as i32 || len1 == len2 {
        return ret;
    }
    return if len1 < len2 { -(1 as i32) } else { 1 as i32 };
}
unsafe extern "C" fn sort_up_index_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    t1 = *(p1 as *const *const NODE);
    t2 = *(p2 as *const *const NODE);
    return cmp_strings(t1, t2);
}
unsafe extern "C" fn sort_down_index_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    return -sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_up_index_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    let mut ret: i32 = 0;
    t1 = *(p1 as *const *const NODE);
    t2 = *(p2 as *const *const NODE);
    ret = cmp_numbers.expect("non-null function pointer")(t1, t2);
    if ret != 0 as i32 {
        return ret;
    }
    t1 = force_string_fmt(t1 as *mut NODE, CONVFMT, CONVFMTidx);
    t2 = force_string_fmt(t2 as *mut NODE, CONVFMT, CONVFMTidx);
    return cmp_strings(t1, t2);
}
unsafe extern "C" fn sort_down_index_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    return -sort_up_index_number(p1, p2);
}
unsafe extern "C" fn sort_up_value_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut t1: *const NODE = 0 as *const NODE;
    let mut t2: *const NODE = 0 as *const NODE;
    let mut ret: i32 = 0;
    t1 = *(p1 as *const *const NODE).offset(1 as i32 as isize);
    t2 = *(p2 as *const *const NODE).offset(1 as i32 as isize);
    if (*t1).type_0 as u32 != nodevals::Node_val as i32 as u32
        || (*t2).type_0 as u32 != nodevals::Node_val as i32 as u32
    {
        return sort_up_value_type(p1, p2);
    }
    ret = cmp_strings(t1, t2);
    if ret != 0 as i32 {
        return ret;
    }
    return sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_down_value_string(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    return -sort_up_value_string(p1, p2);
}
unsafe extern "C" fn sort_up_value_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut t1: *mut NODE = 0 as *mut NODE;
    let mut t2: *mut NODE = 0 as *mut NODE;
    let mut ret: i32 = 0;
    t1 = *(p1 as *const *mut NODE).offset(1 as i32 as isize);
    t2 = *(p2 as *const *mut NODE).offset(1 as i32 as isize);
    if (*t1).type_0 as u32 != nodevals::Node_val as i32 as u32
        || (*t2).type_0 as u32 != nodevals::Node_val as i32 as u32
    {
        return sort_up_value_type(p1, p2);
    }
    ret = cmp_numbers.expect("non-null function pointer")(t1, t2);
    if ret != 0 as i32 {
        return ret;
    }
    ret = cmp_strings(
        force_string_fmt(t1, CONVFMT, CONVFMTidx),
        force_string_fmt(t2, CONVFMT, CONVFMTidx),
    );
    if ret != 0 as i32 {
        return ret;
    }
    return sort_up_index_string(p1, p2);
}
unsafe extern "C" fn sort_down_value_number(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    return -sort_up_value_number(p1, p2);
}
unsafe extern "C" fn do_sort_up_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut n1: *mut NODE = 0 as *mut NODE;
    let mut n2: *mut NODE = 0 as *mut NODE;
    static mut element_types: [NODETYPE; 9] = [
        nodevals::Node_builtin_func,
        nodevals::Node_func,
        nodevals::Node_ext_func,
        nodevals::Node_var_new,
        nodevals::Node_elem_new,
        nodevals::Node_var,
        nodevals::Node_var_array,
        nodevals::Node_val,
        nodevals::Node_illegal,
    ];
    n1 = *(p1 as *const *mut NODE).offset(1 as i32 as isize);
    n2 = *(p2 as *const *mut NODE).offset(1 as i32 as isize);
    if (*n1).type_0 as u32 == nodevals::Node_var as i32 as u32
        && (*n2).type_0 as u32 == nodevals::Node_var as i32 as u32
    {
        n1 = (*n1).sub.nodep.l.lptr;
        n2 = (*n2).sub.nodep.l.lptr;
    }
    if (*n1).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        return ((*n2).type_0 as u32 != nodevals::Node_var_array as i32 as u32) as i32;
    }
    if (*n2).type_0 as u32 == nodevals::Node_var_array as i32 as u32 {
        return -(1 as i32);
    }
    if (*n1).type_0 as u32 != nodevals::Node_val as i32 as u32
        || (*n2).type_0 as u32 != nodevals::Node_val as i32 as u32
    {
        let mut n1_pos: i32 = 0;
        let mut n2_pos: i32 = 0;
        let mut i: i32 = 0;
        n2_pos = -(1 as i32);
        n1_pos = n2_pos;
        i = 0 as i32;
        while element_types[i as usize] as u32 != nodevals::Node_illegal as i32 as u32 {
            if (*n1).type_0 as u32 == element_types[i as usize] as u32 {
                n1_pos = i;
            }
            if (*n2).type_0 as u32 == element_types[i as usize] as u32 {
                n2_pos = i;
            }
            i += 1;
            i;
        }
        return n1_pos - n2_pos;
    }
    fixtype(n1);
    fixtype(n2);
    if (*n1).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
        && (*n2).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
    {
        return cmp_numbers.expect("non-null function pointer")(n1, n2);
    }
    if (*n1).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
        && (*n2).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
    {
        return -(1 as i32)
    } else if (*n1).flags as u32 & flagvals::STRING as i32 as u32 != 0 as i32 as u32
        && (*n2).flags as u32 & flagvals::NUMBER as i32 as u32 != 0 as i32 as u32
    {
        return 1 as i32
    }
    return cmp_strings(n1, n2);
}
unsafe extern "C" fn sort_up_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut rc: i32 = do_sort_up_value_type(p1, p2);
    return if rc != 0 { rc } else { sort_up_index_string(p1, p2) };
}
unsafe extern "C" fn sort_down_value_type(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    return -sort_up_value_type(p1, p2);
}
unsafe extern "C" fn sort_user_func(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut idx1: *mut NODE = 0 as *mut NODE;
    let mut idx2: *mut NODE = 0 as *mut NODE;
    let mut val1: *mut NODE = 0 as *mut NODE;
    let mut val2: *mut NODE = 0 as *mut NODE;
    let mut r: *mut NODE = 0 as *mut NODE;
    let mut ret: i32 = 0;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    idx1 = *(p1 as *const *mut NODE);
    idx2 = *(p2 as *const *mut NODE);
    val1 = *(p1 as *const *mut NODE).offset(1 as i32 as isize);
    val2 = *(p2 as *const *mut NODE).offset(1 as i32 as isize);
    code = (*(*stack_ptr).rptr).sub.nodep.r.iptr;
    (*idx1).valref += 1;
    (*idx1).valref;
    let ref mut fresh11 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh11 = idx1;
    if (*val1).type_0 as u32 == nodevals::Node_val as i32 as u32 {
        (*val1).valref += 1;
        (*val1).valref;
    }
    let ref mut fresh12 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh12 = val1;
    (*idx2).valref += 1;
    (*idx2).valref;
    let ref mut fresh13 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh13 = idx2;
    if (*val2).type_0 as u32 == nodevals::Node_val as i32 as u32 {
        (*val2).valref += 1;
        (*val2).valref;
    }
    let ref mut fresh14 = (*if stack_ptr < stack_top {
        stack_ptr = stack_ptr.offset(1);
        stack_ptr
    } else {
        grow_stack()
    })
        .rptr;
    *fresh14 = val2;
    (Some(interpret.expect("non-null function pointer")))
        .expect("non-null function pointer")(code);
    r = force_number(POP_SCALAR());
    ret = if (*r).sub.val.fltnum < 0.0f64 {
        -(1 as i32)
    } else {
        ((*r).sub.val.fltnum > 0.0f64) as i32
    };
    DEREF(r);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn assoc_list(
    mut symbol: *mut NODE,
    mut sort_str: *const i8,
    mut sort_ctxt: sort_context_t,
) -> *mut *mut NODE {
    static mut sort_funcs: [qsort_funcs; 11] = unsafe {
        [
            {
                let mut init = qsort_funcs {
                    name: b"@ind_str_asc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_up_index_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AISTR as i32
                            | assoc_kind_t::AASC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_num_asc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_up_index_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AINUM as i32
                            | assoc_kind_t::AASC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_str_asc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_up_value_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AVSTR as i32
                            | assoc_kind_t::AASC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_num_asc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_up_value_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AVNUM as i32
                            | assoc_kind_t::AASC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_str_desc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_down_index_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AISTR as i32
                            | assoc_kind_t::ADESC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@ind_num_desc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_down_index_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AINUM as i32
                            | assoc_kind_t::ADESC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_str_desc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_down_value_string
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AVSTR as i32
                            | assoc_kind_t::ADESC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_num_desc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_down_value_number
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AVNUM as i32
                            | assoc_kind_t::ADESC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_type_asc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_up_value_type
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AASC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@val_type_desc\0" as *const u8 as *const i8,
                    comp_func: Some(
                        sort_down_value_type
                            as unsafe extern "C" fn(
                                *const libc::c_void,
                                *const libc::c_void,
                            ) -> i32,
                    ),
                    kind: assoc_kind_t::from_libc_c_uint(
                        (assoc_kind_t::AVALUE as i32 | assoc_kind_t::ADESC as i32) as u32,
                    ),
                };
                init
            },
            {
                let mut init = qsort_funcs {
                    name: b"@unsorted\0" as *const u8 as *const i8,
                    comp_func: None,
                    kind: assoc_kind_t::AINDEX,
                };
                init
            },
        ]
    };
    let mut list: *mut *mut NODE = 0 as *mut *mut NODE;
    let mut akind: NODE = NODE {
        sub: C2RustUnnamed_1 {
            nodep: C2RustUnnamed_3 {
                l: C2RustUnnamed_10 {
                    lptr: 0 as *mut exp_node,
                },
                r: C2RustUnnamed_5 {
                    rptr: 0 as *mut exp_node,
                },
                x: C2RustUnnamed_4 {
                    extra: 0 as *mut exp_node,
                },
                name: 0 as *mut i8,
                reserved: 0,
                rn: 0 as *mut exp_node,
                cnt: 0,
                reflags: reflagvals::from_libc_c_uint(0 as u32),
            },
        },
        type_0: nodevals::Node_illegal,
        flags: flagvals::from_libc_c_uint(0 as u32),
        valref: 0,
    };
    let mut num_elems: u64 = 0;
    let mut j: u64 = 0;
    let mut elem_size: i32 = 0;
    let mut qi: i32 = 0;
    let mut cmp_func: qsort_compfunc = None;
    let mut code: *mut INSTRUCTION = 0 as *mut INSTRUCTION;
    extern "C" {
        static mut currule: i32;
    }
    let mut save_rule: i32 = 0 as i32;
    let mut assoc_kind: assoc_kind_t = assoc_kind_t::ANONE;
    elem_size = 1 as i32;
    qi = 0 as i32;
    j = (::core::mem::size_of::<[qsort_funcs; 11]>() as u64)
        .wrapping_div(::core::mem::size_of::<qsort_funcs>() as u64);
    while (qi as u64) < j {
        if strcmp(sort_funcs[qi as usize].name, sort_str) == 0 as i32 {
            break;
        }
        qi += 1;
        qi;
    }
    if (qi as u64) < j {
        cmp_func = sort_funcs[qi as usize].comp_func;
        assoc_kind = sort_funcs[qi as usize].kind;
        if (*symbol).sub.nodep.l.lp != &cint_array_func as *const array_funcs_t {
            assoc_kind = ::core::mem::transmute::<
                u32,
                assoc_kind_t,
            >(
                assoc_kind as u32
                    & !(assoc_kind_t::AASC as i32 | assoc_kind_t::ADESC as i32) as u32,
            );
        }
        if sort_ctxt as u32 != sort_context_t::SORTED_IN as i32 as u32
            || assoc_kind as u32 & assoc_kind_t::AVALUE as i32 as u32 != 0 as i32 as u32
        {
            assoc_kind = ::core::mem::transmute::<
                u32,
                assoc_kind_t,
            >(
                assoc_kind as u32
                    | (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32,
            );
            elem_size = 2 as i32;
        }
    } else {
        let mut f: *mut NODE = 0 as *mut NODE;
        let mut sp: *const i8 = 0 as *const i8;
        sp = sort_str;
        while *sp as i32 != '\0' as i32
            && *(*__ctype_b_loc()).offset(*sp as u8 as i32 as isize) as i32
                & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 == 0
        {
            sp = sp.offset(1);
            sp;
        }
        if sp == sort_str || *sp as i32 != '\0' as i32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 1410 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"`%s' is invalid as a function name\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                sort_str,
            );
        }
        f = lookup(sort_str);
        if f.is_null() || (*f).type_0 as u32 != nodevals::Node_func as i32 as u32 {
            (set_loc
                as unsafe extern "C" fn(
                    *const i8,
                    i32,
                ) -> ())(b"array.c\0" as *const u8 as *const i8, 1414 as i32);
            (Some(
                (Some(r_fatal as unsafe extern "C" fn(*const i8, ...) -> ()))
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                dcgettext(
                    0 as *const i8,
                    b"sort comparison function `%s' is not defined\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                sort_str,
            );
        }
        cmp_func = Some(
            sort_user_func
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        );
        assoc_kind = ::core::mem::transmute::<
            u32,
            assoc_kind_t,
        >(
            assoc_kind as u32
                | (assoc_kind_t::AVALUE as i32 | assoc_kind_t::AINDEX as i32) as u32,
        );
        elem_size = 2 as i32;
        code = bcalloc(opcodeval::Op_func_call, 2 as i32, 0 as i32);
        (*code).x.xn = f;
        (*code).d.name = 0 as *mut i8;
        (*code.offset(1 as i32 as isize)).x.xl = 4 as i32 as i64;
        (*code).nexti = bcalloc(opcodeval::Op_stop, 1 as i32, 0 as i32);
        save_rule = currule;
        currule = 0 as i32;
        PUSH_CODE(code);
    }
    akind.flags = flagvals::from_libc_c_uint(assoc_kind as u32 as u32);
    list = ((*(*symbol).sub.nodep.l.lp).list)
        .expect("non-null function pointer")(symbol, &mut akind);
    assoc_kind = akind.flags as assoc_kind_t;
    if !list.is_null() && cmp_func.is_some()
        && assoc_kind as u32
            & (assoc_kind_t::AASC as i32 | assoc_kind_t::ADESC as i32) as u32
            == 0 as i32 as u32
    {
        num_elems = (*symbol).sub.nodep.reflags as u64;
        qsort(
            list as *mut libc::c_void,
            num_elems,
            (elem_size as u64).wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
            cmp_func,
        );
        if sort_ctxt as u32 == sort_context_t::SORTED_IN as i32 as u32
            && assoc_kind as u32
                & (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
                == (assoc_kind_t::AINDEX as i32 | assoc_kind_t::AVALUE as i32) as u32
        {
            j = 1 as i32 as u64;
            while j < num_elems {
                let ref mut fresh15 = *list.offset(j as isize);
                *fresh15 = *list.offset((2 as i32 as u64).wrapping_mul(j) as isize);
                j = j.wrapping_add(1);
                j;
            }
            list = erealloc_real(
                list as *mut libc::c_void,
                num_elems.wrapping_mul(::core::mem::size_of::<*mut NODE>() as u64),
                b"assoc_list\0" as *const u8 as *const i8,
                b"list\0" as *const u8 as *const i8,
                b"array.c\0" as *const u8 as *const i8,
                1458 as i32,
            ) as *mut *mut NODE;
        }
    }
    if cmp_func
        == Some(
            sort_user_func
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        )
    {
        code = POP_CODE();
        currule = save_rule;
        bcfree((*code).nexti);
        bcfree(code);
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn new_array_element() -> *mut NODE {
    let mut n: *mut NODE = make_number.expect("non-null function pointer")(0.0f64);
    let mut sp: *mut i8 = 0 as *mut i8;
    sp = emalloc_real(
        2 as i32 as size_t,
        b"new_array_element\0" as *const u8 as *const i8,
        b"sp\0" as *const u8 as *const i8,
        b"array.c\0" as *const u8 as *const i8,
        1480 as i32,
    ) as *mut i8;
    let ref mut fresh16 = *sp.offset(1 as i32 as isize);
    *fresh16 = '\0' as i32 as i8;
    *sp.offset(0 as i32 as isize) = *fresh16;
    (*n).sub.val.sp = sp;
    (*n).sub.val.slen = 0 as i32 as size_t;
    (*n).sub.val.idx = -(1 as i32);
    (*n).flags = ::core::mem::transmute::<
        u32,
        flagvals,
    >(
        (*n).flags as u32
            | (flagvals::MALLOC as i32 | flagvals::STRING as i32
                | flagvals::STRCUR as i32) as u32,
    );
    (*n).type_0 = nodevals::Node_elem_new;
    (*n).valref = 1 as i32 as i64;
    return n;
}