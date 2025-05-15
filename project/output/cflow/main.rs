use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type table_entry;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut rpl_optarg: *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn posix_output_handler(
        cmd: cflow_output_command,
        outfile: *mut FILE,
        line: i32,
        data: *mut libc::c_void,
        handler_data: *mut libc::c_void,
    ) -> i32;
    fn gnu_output_handler(
        cmd: cflow_output_command,
        outfile: *mut FILE,
        line: i32,
        data: *mut libc::c_void,
        handler_data: *mut libc::c_void,
    ) -> i32;
    fn output_init();
    fn select_output_driver(name: *const i8) -> i32;
    fn register_output(
        name: *const i8,
        handler: Option<
            unsafe extern "C" fn(
                cflow_output_command,
                *mut FILE,
                i32,
                *mut libc::c_void,
                *mut libc::c_void,
            ) -> i32,
        >,
        handler_data: *mut libc::c_void,
    ) -> i32;
    static mut token_stack_length: i32;
    static mut input_file_count: u32;
    fn lookup(_: *const i8) -> *mut Symbol;
    fn install(_: *mut i8, _: i32) -> *mut Symbol;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    fn source(name: *mut i8) -> i32;
    fn init_lex(debug_level: i32);
    fn set_preprocessor(arg: *const i8);
    fn pp_option(arg: *const i8);
    fn init_parse();
    fn yyparse() -> i32;
    fn output();
    fn sourcerc(_: *mut i32, _: *mut *mut *mut i8);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn argp_parse(
        __argp: *const argp,
        _: i32,
        _: *mut *mut i8,
        __flags: u32,
        __arg_index: *mut i32,
        __input: *mut libc::c_void,
    ) -> error_t;
    fn argp_error(__state: *const argp_state, __fmt: *const i8, _: ...);
    fn argp_version_setup(name: *const i8, authors: *const *const i8);
    fn set_program_name(argv0: *const i8);
}
pub type size_t = u64;
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type error_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symtype {
    SymUndefined,
    SymToken,
    SymIdentifier,
}
impl symtype {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            symtype::SymUndefined => 0,
            symtype::SymToken => 1,
            symtype::SymIdentifier => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> symtype {
        match value {
            0 => symtype::SymUndefined,
            1 => symtype::SymToken,
            2 => symtype::SymIdentifier,
            _ => panic!("Invalid value for symtype: {}", value),
        }
    }
}
impl AddAssign<u32> for symtype {
    fn add_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for symtype {
    fn sub_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for symtype {
    fn mul_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for symtype {
    fn div_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for symtype {
    fn rem_assign(&mut self, rhs: u32) {
        *self = symtype::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for symtype {
    type Output = symtype;
    fn add(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for symtype {
    type Output = symtype;
    fn sub(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for symtype {
    type Output = symtype;
    fn mul(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for symtype {
    type Output = symtype;
    fn div(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for symtype {
    type Output = symtype;
    fn rem(self, rhs: u32) -> symtype {
        symtype::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum storage {
    ExternStorage,
    ExplicitExternStorage,
    StaticStorage,
    AutoStorage,
    AnyStorage,
}
impl storage {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            storage::ExternStorage => 0,
            storage::ExplicitExternStorage => 1,
            storage::StaticStorage => 2,
            storage::AutoStorage => 3,
            storage::AnyStorage => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> storage {
        match value {
            0 => storage::ExternStorage,
            1 => storage::ExplicitExternStorage,
            2 => storage::StaticStorage,
            3 => storage::AutoStorage,
            4 => storage::AnyStorage,
            _ => panic!("Invalid value for storage: {}", value),
        }
    }
}
impl AddAssign<u32> for storage {
    fn add_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for storage {
    fn sub_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for storage {
    fn mul_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for storage {
    fn div_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for storage {
    fn rem_assign(&mut self, rhs: u32) {
        *self = storage::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for storage {
    type Output = storage;
    fn add(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for storage {
    type Output = storage;
    fn sub(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for storage {
    type Output = storage;
    fn mul(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for storage {
    type Output = storage;
    fn div(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for storage {
    type Output = storage;
    fn rem(self, rhs: u32) -> storage {
        storage::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum symbol_flag {
    symbol_none,
    symbol_temp,
    symbol_parm,
    symbol_alias,
}
impl symbol_flag {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            symbol_flag::symbol_none => 0,
            symbol_flag::symbol_temp => 1,
            symbol_flag::symbol_parm => 2,
            symbol_flag::symbol_alias => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> symbol_flag {
        match value {
            0 => symbol_flag::symbol_none,
            1 => symbol_flag::symbol_temp,
            2 => symbol_flag::symbol_parm,
            3 => symbol_flag::symbol_alias,
            _ => panic!("Invalid value for symbol_flag: {}", value),
        }
    }
}
impl AddAssign<u32> for symbol_flag {
    fn add_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for symbol_flag {
    fn sub_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for symbol_flag {
    fn mul_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for symbol_flag {
    fn div_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for symbol_flag {
    fn rem_assign(&mut self, rhs: u32) {
        *self = symbol_flag::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for symbol_flag {
    type Output = symbol_flag;
    fn add(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for symbol_flag {
    type Output = symbol_flag;
    fn sub(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for symbol_flag {
    type Output = symbol_flag;
    fn mul(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for symbol_flag {
    type Output = symbol_flag;
    fn div(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for symbol_flag {
    type Output = symbol_flag;
    fn rem(self, rhs: u32) -> symbol_flag {
        symbol_flag::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut i8,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: i32,
    pub expand_line: i32,
    pub token_type: i32,
    pub source: *mut i8,
    pub def_line: i32,
    pub ref_line: *mut linked_list,
    pub level: i32,
    pub decl: *mut i8,
    pub storage: storage,
    pub arity: i32,
    pub recursive: i32,
    pub ord: size_t,
    pub caller: *mut linked_list,
    pub callee: *mut linked_list,
}
pub type Symbol = symbol;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cflow_output_command {
    cflow_output_init,
    cflow_output_begin,
    cflow_output_end,
    cflow_output_newline,
    cflow_output_separator,
    cflow_output_symbol,
    cflow_output_text,
}
impl cflow_output_command {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cflow_output_command::cflow_output_init => 0,
            cflow_output_command::cflow_output_begin => 1,
            cflow_output_command::cflow_output_end => 2,
            cflow_output_command::cflow_output_newline => 3,
            cflow_output_command::cflow_output_separator => 4,
            cflow_output_command::cflow_output_symbol => 5,
            cflow_output_command::cflow_output_text => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> cflow_output_command {
        match value {
            0 => cflow_output_command::cflow_output_init,
            1 => cflow_output_command::cflow_output_begin,
            2 => cflow_output_command::cflow_output_end,
            3 => cflow_output_command::cflow_output_newline,
            4 => cflow_output_command::cflow_output_separator,
            5 => cflow_output_command::cflow_output_symbol,
            6 => cflow_output_command::cflow_output_text,
            _ => panic!("Invalid value for cflow_output_command: {}", value),
        }
    }
}
impl AddAssign<u32> for cflow_output_command {
    fn add_assign(&mut self, rhs: u32) {
        *self = cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cflow_output_command {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cflow_output_command {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cflow_output_command {
    fn div_assign(&mut self, rhs: u32) {
        *self = cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cflow_output_command {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cflow_output_command {
    type Output = cflow_output_command;
    fn add(self, rhs: u32) -> cflow_output_command {
        cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cflow_output_command {
    type Output = cflow_output_command;
    fn sub(self, rhs: u32) -> cflow_output_command {
        cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cflow_output_command {
    type Output = cflow_output_command;
    fn mul(self, rhs: u32) -> cflow_output_command {
        cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cflow_output_command {
    type Output = cflow_output_command;
    fn div(self, rhs: u32) -> cflow_output_command {
        cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cflow_output_command {
    type Output = cflow_output_command;
    fn rem(self, rhs: u32) -> cflow_output_command {
        cflow_output_command::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
pub struct argp_option {
    pub name: *const i8,
    pub key: i32,
    pub arg: *const i8,
    pub flags: i32,
    pub doc: *const i8,
    pub group: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const i8,
    pub doc: *const i8,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(i32, *const i8, *mut libc::c_void) -> *mut i8,
    >,
    pub argp_domain: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: i32,
    pub header: *const i8,
    pub group: i32,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub next: i32,
    pub flags: u32,
    pub arg_num: u32,
    pub quoted: i32,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut i8,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum option_code {
    OPT_DEFINES = 256,
    OPT_LEVEL_INDENT,
    OPT_DEBUG,
    OPT_PREPROCESS,
    OPT_NO_PREPROCESS,
    OPT_EMACS,
    OPT_NO_USE_INDENTATION,
    OPT_NO_ANSI,
    OPT_NO_TREE,
    OPT_NO_BRIEF,
    OPT_NO_EMACS,
    OPT_NO_VERBOSE,
    OPT_NO_NUMBER,
    OPT_NO_PRINT_LEVEL,
    OPT_NO_REVERSE,
    OPT_OMIT_ARGUMENTS,
    OPT_NO_OMIT_ARGUMENTS,
    OPT_OMIT_SYMBOL_NAMES,
    OPT_NO_OMIT_SYMBOL_NAMES,
}
impl option_code {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            option_code::OPT_DEFINES => 256,
            option_code::OPT_LEVEL_INDENT => 257,
            option_code::OPT_DEBUG => 258,
            option_code::OPT_PREPROCESS => 259,
            option_code::OPT_NO_PREPROCESS => 260,
            option_code::OPT_EMACS => 261,
            option_code::OPT_NO_USE_INDENTATION => 262,
            option_code::OPT_NO_ANSI => 263,
            option_code::OPT_NO_TREE => 264,
            option_code::OPT_NO_BRIEF => 265,
            option_code::OPT_NO_EMACS => 266,
            option_code::OPT_NO_VERBOSE => 267,
            option_code::OPT_NO_NUMBER => 268,
            option_code::OPT_NO_PRINT_LEVEL => 269,
            option_code::OPT_NO_REVERSE => 270,
            option_code::OPT_OMIT_ARGUMENTS => 271,
            option_code::OPT_NO_OMIT_ARGUMENTS => 272,
            option_code::OPT_OMIT_SYMBOL_NAMES => 273,
            option_code::OPT_NO_OMIT_SYMBOL_NAMES => 274,
        }
    }
    fn from_libc_c_uint(value: u32) -> option_code {
        match value {
            256 => option_code::OPT_DEFINES,
            257 => option_code::OPT_LEVEL_INDENT,
            258 => option_code::OPT_DEBUG,
            259 => option_code::OPT_PREPROCESS,
            260 => option_code::OPT_NO_PREPROCESS,
            261 => option_code::OPT_EMACS,
            262 => option_code::OPT_NO_USE_INDENTATION,
            263 => option_code::OPT_NO_ANSI,
            264 => option_code::OPT_NO_TREE,
            265 => option_code::OPT_NO_BRIEF,
            266 => option_code::OPT_NO_EMACS,
            267 => option_code::OPT_NO_VERBOSE,
            268 => option_code::OPT_NO_NUMBER,
            269 => option_code::OPT_NO_PRINT_LEVEL,
            270 => option_code::OPT_NO_REVERSE,
            271 => option_code::OPT_OMIT_ARGUMENTS,
            272 => option_code::OPT_NO_OMIT_ARGUMENTS,
            273 => option_code::OPT_OMIT_SYMBOL_NAMES,
            274 => option_code::OPT_NO_OMIT_SYMBOL_NAMES,
            _ => panic!("Invalid value for option_code: {}", value),
        }
    }
}
impl AddAssign<u32> for option_code {
    fn add_assign(&mut self, rhs: u32) {
        *self = option_code::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for option_code {
    fn sub_assign(&mut self, rhs: u32) {
        *self = option_code::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for option_code {
    fn mul_assign(&mut self, rhs: u32) {
        *self = option_code::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for option_code {
    fn div_assign(&mut self, rhs: u32) {
        *self = option_code::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for option_code {
    fn rem_assign(&mut self, rhs: u32) {
        *self = option_code::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for option_code {
    type Output = option_code;
    fn add(self, rhs: u32) -> option_code {
        option_code::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for option_code {
    type Output = option_code;
    fn sub(self, rhs: u32) -> option_code {
        option_code::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for option_code {
    type Output = option_code;
    fn mul(self, rhs: u32) -> option_code {
        option_code::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for option_code {
    type Output = option_code;
    fn div(self, rhs: u32) -> option_code {
        option_code::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for option_code {
    type Output = option_code;
    fn rem(self, rhs: u32) -> option_code {
        option_code::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_type {
    pub str_0: *mut i8,
    pub min_match: i32,
    pub type_0: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut argp_program_bug_address: *const i8 = b"<bug-cflow@gnu.org>\0"
    as *const u8 as *const i8;
static mut doc: [i8; 180] = unsafe {
    *::core::mem::transmute::<
        &[u8; 180],
        &mut [i8; 180],
    >(
        b"generate a program flowgraph\x0B* The effect of each option marked with an asterisk is reversed if the option's long name is prefixed with `no-'. For example, --no-cpp cancels --cpp.\0",
    )
};
#[no_mangle]
pub static mut program_authors: [*const i8; 2] = [
    b"Sergey Poznyakoff\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut options: [argp_option; 49] = [
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"General options:\0" as *const u8 as *const i8,
            group: 0 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"depth\0" as *const u8 as *const i8,
            key: 'd' as i32,
            arg: b"NUMBER\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set the depth at which the flowgraph is cut off\0" as *const u8
                as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"include\0" as *const u8 as *const i8,
            key: 'i' as i32,
            arg: b"CLASSES\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Include specified classes of symbols (see below). Prepend CLASSES with ^ or - to exclude them from the output\0"
                as *const u8 as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"format\0" as *const u8 as *const i8,
            key: 'f' as i32,
            arg: b"NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Use given output format NAME. Valid names are `gnu' (default) and `posix'\0"
                as *const u8 as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"reverse\0" as *const u8 as *const i8,
            key: 'r' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Print reverse call tree\0" as *const u8 as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"xref\0" as *const u8 as *const i8,
            key: 'x' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Produce cross-reference listing only\0" as *const u8 as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"print\0" as *const u8 as *const i8,
            key: 'P' as i32,
            arg: b"OPT\0" as *const u8 as *const i8,
            flags: 0x2 as i32,
            doc: b"Set printing option to OPT. Valid OPT values are: xref (or cross-ref), tree. Any unambiguous abbreviation of the above is also accepted\0"
                as *const u8 as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"output\0" as *const u8 as *const i8,
            key: 'o' as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set output file name (default -, meaning stdout)\0" as *const u8
                as *const i8,
            group: 0 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Symbols classes for --include argument\0" as *const u8 as *const i8,
            group: 0 as i32 + 2 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  x\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x8 as i32 | 0x20 as i32,
            doc: b"all data symbols, both external and static\0" as *const u8
                as *const i8,
            group: 0 as i32 + 3 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  _\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x8 as i32 | 0x20 as i32,
            doc: b"symbols whose names begin with an underscore\0" as *const u8
                as *const i8,
            group: 0 as i32 + 3 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  s\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x8 as i32 | 0x20 as i32,
            doc: b"static symbols\0" as *const u8 as *const i8,
            group: 0 as i32 + 3 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"  t\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x8 as i32 | 0x20 as i32,
            doc: b"typedefs (for cross-references only)\0" as *const u8 as *const i8,
            group: 0 as i32 + 3 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Parser control:\0" as *const u8 as *const i8,
            group: 10 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"use-indentation\0" as *const u8 as *const i8,
            key: 'S' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Rely on indentation\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-use-indentation\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_USE_INDENTATION as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ansi\0" as *const u8 as *const i8,
            key: 'a' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Accept only sources in ANSI C\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ansi\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_ANSI as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"pushdown\0" as *const u8 as *const i8,
            key: 'p' as i32,
            arg: b"NUMBER\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Set initial token stack size to NUMBER\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"symbol\0" as *const u8 as *const i8,
            key: 's' as i32,
            arg: b"SYMBOL:[=]TYPE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Register SYMBOL with given TYPE, or define an alias (if := is used). Valid types are: keyword (or kw), modifier, qualifier, identifier, type, wrapper. Any unambiguous abbreviation of the above is also accepted\0"
                as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"main\0" as *const u8 as *const i8,
            key: 'm' as i32,
            arg: b"NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Assume main function to be called NAME\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"define\0" as *const u8 as *const i8,
            key: 'D' as i32,
            arg: b"NAME[=DEFN]\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Predefine NAME as a macro\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"undefine\0" as *const u8 as *const i8,
            key: 'U' as i32,
            arg: b"NAME\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Cancel any previous definition of NAME\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"include-dir\0" as *const u8 as *const i8,
            key: 'I' as i32,
            arg: b"DIR\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Add the directory DIR to the list of directories to be searched for header files.\0"
                as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"preprocess\0" as *const u8 as *const i8,
            key: option_code::OPT_PREPROCESS as i32,
            arg: b"COMMAND\0" as *const u8 as *const i8,
            flags: 0x1 as i32,
            doc: b"* Run the specified preprocessor command\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"cpp\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x4 as i32,
            doc: 0 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-preprocess\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_PREPROCESS as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-cpp\0" as *const u8 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0x4 as i32 | 0x2 as i32,
            doc: 0 as *const i8,
            group: 10 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Output control:\0" as *const u8 as *const i8,
            group: 20 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"number\0" as *const u8 as *const i8,
            key: 'n' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Print line numbers\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-number\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_NUMBER as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"print-level\0" as *const u8 as *const i8,
            key: 'l' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Print nesting level along with the call tree\0" as *const u8
                as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-print-level\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_PRINT_LEVEL as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"level-indent\0" as *const u8 as *const i8,
            key: option_code::OPT_LEVEL_INDENT as i32,
            arg: b"ELEMENT\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"Control graph appearance\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"tree\0" as *const u8 as *const i8,
            key: 'T' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Draw ASCII art tree\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-tree\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_TREE as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"brief\0" as *const u8 as *const i8,
            key: 'b' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Brief output\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-brief\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_BRIEF as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"emacs\0" as *const u8 as *const i8,
            key: option_code::OPT_EMACS as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Additionally format output for use with GNU Emacs\0" as *const u8
                as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-emacs\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_EMACS as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"omit-arguments\0" as *const u8 as *const i8,
            key: option_code::OPT_OMIT_ARGUMENTS as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Do not print argument lists in function declarations\0" as *const u8
                as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ignore-arguments\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_OMIT_ARGUMENTS as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"omit-symbol-names\0" as *const u8 as *const i8,
            key: option_code::OPT_OMIT_SYMBOL_NAMES as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Do not print symbol names in declaration strings\0" as *const u8
                as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-omit-symbol-names\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_OMIT_SYMBOL_NAMES as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 20 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Informational options:\0" as *const u8 as *const i8,
            group: 30 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbose\0" as *const u8 as *const i8,
            key: 'v' as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"* Verbose error diagnostics\0" as *const u8 as *const i8,
            group: 30 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-verbose\0" as *const u8 as *const i8,
            key: option_code::OPT_NO_VERBOSE as i32,
            arg: 0 as *const i8,
            flags: 0x2 as i32,
            doc: b"\0" as *const u8 as *const i8,
            group: 30 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"debug\0" as *const u8 as *const i8,
            key: option_code::OPT_DEBUG as i32,
            arg: b"NUMBER\0" as *const u8 as *const i8,
            flags: 0x1 as i32,
            doc: b"Set debugging level\0" as *const u8 as *const i8,
            group: 30 as i32 + 1 as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0,
            arg: 0 as *const i8,
            flags: 0,
            doc: 0 as *const i8,
            group: 0,
        };
        init
    },
];
#[no_mangle]
pub static mut debug: i32 = 0;
#[no_mangle]
pub static mut outname: *mut i8 = b"-\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut print_option: i32 = 0 as i32;
#[no_mangle]
pub static mut verbose: i32 = 0;
#[no_mangle]
pub static mut use_indentation: i32 = 0;
#[no_mangle]
pub static mut record_defines: i32 = 0;
#[no_mangle]
pub static mut strict_ansi: i32 = 0;
#[no_mangle]
pub static mut print_line_numbers: i32 = 0;
#[no_mangle]
pub static mut print_levels: i32 = 0;
#[no_mangle]
pub static mut print_as_tree: i32 = 0;
#[no_mangle]
pub static mut brief_listing: i32 = 0;
#[no_mangle]
pub static mut reverse_tree: i32 = 0;
#[no_mangle]
pub static mut max_depth: i32 = 0;
#[no_mangle]
pub static mut emacs_option: i32 = 0;
#[no_mangle]
pub static mut omit_arguments_option: i32 = 0;
#[no_mangle]
pub static mut omit_symbol_names_option: i32 = 0;
#[no_mangle]
pub static mut symbol_map: i32 = 0;
#[no_mangle]
pub static mut level_indent: [*mut i8; 2] = [
    0 as *const i8 as *mut i8,
    0 as *const i8 as *mut i8,
];
#[no_mangle]
pub static mut level_end: [*mut i8; 2] = [
    b"\0" as *const u8 as *const i8 as *mut i8,
    b"\0" as *const u8 as *const i8 as *mut i8,
];
#[no_mangle]
pub static mut level_begin: *mut i8 = b"\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut preprocess_option: i32 = 0 as i32;
#[no_mangle]
pub static mut start_name: *mut i8 = b"main\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut arglist: *mut linked_list = 0 as *const linked_list as *mut linked_list;
unsafe extern "C" fn find_option_type(
    mut optype: *mut option_type,
    mut str: *const i8,
    mut len: i32,
) -> i32 {
    if len == 0 as i32 {
        len = strlen(str) as i32;
    }
    while !((*optype).str_0).is_null() {
        if len >= (*optype).min_match
            && memcmp(
                str as *const libc::c_void,
                (*optype).str_0 as *const libc::c_void,
                len as u64,
            ) == 0 as i32
        {
            return (*optype).type_0;
        }
        optype = optype.offset(1);
        optype;
    }
    return 0 as i32;
}
static mut symbol_optype: [option_type; 8] = [
    {
        let mut init = option_type {
            str_0: b"keyword\0" as *const u8 as *const i8 as *mut i8,
            min_match: 2 as i32,
            type_0: 257 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"kw\0" as *const u8 as *const i8 as *mut i8,
            min_match: 2 as i32,
            type_0: 257 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"modifier\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 265 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"identifier\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 260 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"type\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 270 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"wrapper\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 272 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"qualifier\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 273 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: 0 as *const i8 as *mut i8,
            min_match: 0,
            type_0: 0,
        };
        init
    },
];
unsafe extern "C" fn symbol_override(mut str: *const i8) {
    let mut ptr: *const i8 = 0 as *const i8;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    ptr = strchr(str, ':' as i32);
    if ptr.is_null() {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: no symbol type supplied\0" as *const u8 as *const i8,
                5 as i32,
            ),
            str,
        );
        return;
    } else {
        name = strndup(str, ptr.offset_from(str) as i64 as u64);
        if *ptr.offset(1 as i32 as isize) as i32 == '=' as i32 {
            let mut alias: *mut Symbol = lookup(ptr.offset(2 as i32 as isize));
            if alias.is_null() {
                alias = install(xstrdup(ptr.offset(2 as i32 as isize)), 0x1 as i32);
                (*alias).type_0 = symtype::SymToken;
                (*alias).token_type = 0 as i32;
                (*alias).source = 0 as *mut i8;
                (*alias).def_line = -(1 as i32);
                (*alias).ref_line = 0 as *mut linked_list;
            }
            sp = install(name, 0x1 as i32);
            (*sp).type_0 = symtype::SymToken;
            (*sp).alias = alias;
            (*sp).flag = symbol_flag::symbol_alias;
        } else {
            let mut type_0: i32 = find_option_type(
                symbol_optype.as_mut_ptr(),
                ptr.offset(1 as i32 as isize),
                0 as i32,
            );
            if type_0 == 0 as i32 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unknown symbol type: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    ptr.offset(1 as i32 as isize),
                );
                return;
            }
            sp = install(name, 0x1 as i32);
            (*sp).type_0 = symtype::SymToken;
            (*sp).token_type = type_0;
        }
        (*sp).source = 0 as *mut i8;
        (*sp).def_line = -(1 as i32);
        (*sp).ref_line = 0 as *mut linked_list;
    };
}
static mut print_optype: [option_type; 4] = [
    {
        let mut init = option_type {
            str_0: b"xref\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 0x1 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"cross-ref\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 0x1 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"tree\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 0x2 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: 0 as *const i8 as *mut i8,
            min_match: 0,
            type_0: 0,
        };
        init
    },
];
unsafe extern "C" fn set_print_option(mut str: *mut i8) {
    let mut opt: i32 = 0;
    opt = find_option_type(print_optype.as_mut_ptr(), str, 0 as i32);
    if opt == 0 as i32 {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"unknown print option: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            str,
        );
        return;
    }
    print_option |= opt;
}
unsafe extern "C" fn number(
    mut str_ptr: *mut *const i8,
    mut base: i32,
    mut count: i32,
) -> i32 {
    let mut c: i32 = 0;
    let mut n: i32 = 0;
    let mut i: u32 = 0;
    let mut str: *const i8 = *str_ptr;
    n = 0 as i32;
    while *str as i32 != 0 && count != 0 {
        let fresh0 = str;
        str = str.offset(1);
        c = *fresh0 as i32;
        if *(*__ctype_b_loc()).offset(c as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            i = (c - '0' as i32) as u32;
        } else {
            i = (({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = c;
                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(c);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(c as isize);
                }
                __res
            }) - 'A' as i32 + 10 as i32) as u32;
        }
        if i > base as u32 {
            break;
        }
        n = ((n * base) as u32).wrapping_add(i) as i32;
        count -= 1;
        count;
    }
    *str_ptr = str.offset(-(1 as i32 as isize));
    return n;
}
static mut level_indent_optype: [option_type; 6] = [
    {
        let mut init = option_type {
            str_0: b"begin\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 1 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"start\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 1 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"0\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 2 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"1\0" as *const u8 as *const i8 as *mut i8,
            min_match: 1 as i32,
            type_0: 3 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"end0\0" as *const u8 as *const i8 as *mut i8,
            min_match: 4 as i32,
            type_0: 4 as i32,
        };
        init
    },
    {
        let mut init = option_type {
            str_0: b"end1\0" as *const u8 as *const i8 as *mut i8,
            min_match: 4 as i32,
            type_0: 5 as i32,
        };
        init
    },
];
unsafe extern "C" fn parse_level_string(
    mut str: *const i8,
    mut return_ptr: *mut *mut i8,
) {
    static mut text: [i8; 216] = [0; 216];
    let mut p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut num: i32 = 0;
    p = text.as_mut_ptr();
    memset(
        text.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        ::core::mem::size_of::<[i8; 216]>() as u64,
    );
    text[(::core::mem::size_of::<[i8; 216]>() as u64).wrapping_sub(1 as i32 as u64)
        as usize] = 0 as i32 as i8;
    while *str != 0 {
        let mut current_block_31: u64;
        match *str as i32 {
            92 => {
                str = str.offset(1);
                match *str as i32 {
                    97 => {
                        let fresh1 = p;
                        p = p.offset(1);
                        *fresh1 = '\u{7}' as i32 as i8;
                    }
                    98 => {
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = '\u{8}' as i32 as i8;
                    }
                    101 => {
                        let fresh3 = p;
                        p = p.offset(1);
                        *fresh3 = '\u{1b}' as i32 as i8;
                    }
                    102 => {
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = '\u{c}' as i32 as i8;
                    }
                    110 => {
                        let fresh5 = p;
                        p = p.offset(1);
                        *fresh5 = '\n' as i32 as i8;
                    }
                    114 => {
                        let fresh6 = p;
                        p = p.offset(1);
                        *fresh6 = '\r' as i32 as i8;
                    }
                    116 => {
                        let fresh7 = p;
                        p = p.offset(1);
                        *fresh7 = '\t' as i32 as i8;
                    }
                    120 | 88 => {
                        str = str.offset(1);
                        str;
                        let fresh8 = p;
                        p = p.offset(1);
                        *fresh8 = number(&mut str, 16 as i32, 2 as i32) as i8;
                    }
                    48 => {
                        str = str.offset(1);
                        str;
                        let fresh9 = p;
                        p = p.offset(1);
                        *fresh9 = number(&mut str, 8 as i32, 3 as i32) as i8;
                    }
                    _ => {
                        let fresh10 = p;
                        p = p.offset(1);
                        *fresh10 = *str;
                    }
                }
                str = str.offset(1);
                str;
                current_block_31 = 6450636197030046351;
            }
            120 => {
                if p == text.as_mut_ptr() {
                    current_block_31 = 15525553228573972096;
                } else {
                    num = strtol(
                        str.offset(1 as i32 as isize),
                        &mut str as *mut *const i8 as *mut *mut i8,
                        10 as i32,
                    ) as i32;
                    c = *p.offset(-(1 as i32) as isize) as i32;
                    i = 1 as i32;
                    while i < num {
                        let fresh11 = p;
                        p = p.offset(1);
                        *fresh11 = c as i8;
                        if *p as i32 == 0 as i32 {
                            error(
                                1 as i32,
                                0 as i32,
                                dcgettext(
                                    0 as *const i8,
                                    b"level indent string is too long\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                            return;
                        }
                        i += 1;
                        i;
                    }
                    current_block_31 = 6450636197030046351;
                }
            }
            _ => {
                current_block_31 = 15525553228573972096;
            }
        }
        match current_block_31 {
            15525553228573972096 => {
                let fresh12 = str;
                str = str.offset(1);
                let fresh13 = p;
                p = p.offset(1);
                *fresh13 = *fresh12;
                if *p as i32 == 0 as i32 {
                    error(
                        1 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"level indent string is too long\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    return;
                }
            }
            _ => {}
        }
    }
    *p = 0 as i32 as i8;
    *return_ptr = strdup(text.as_mut_ptr());
}
unsafe extern "C" fn set_level_indent(mut str: *const i8) {
    let mut n: i64 = 0;
    let mut p: *const i8 = 0 as *const i8;
    let mut q: *mut i8 = 0 as *mut i8;
    n = strtol(str, &mut q, 0 as i32);
    if *q as i32 == 0 as i32 && n > 0 as i32 as i64 {
        let mut s: *mut i8 = xmalloc((n + 1 as i32 as i64) as size_t) as *mut i8;
        memset(s as *mut libc::c_void, ' ' as i32, (n - 1 as i32 as i64) as u64);
        *s.offset((n - 1 as i32 as i64) as isize) = 0 as i32 as i8;
        level_indent[1 as i32 as usize] = s;
        level_indent[0 as i32 as usize] = level_indent[1 as i32 as usize];
        return;
    }
    p = str;
    while *p as i32 != '=' as i32 {
        if *p as i32 == 0 as i32 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"level-indent syntax\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            return;
        }
        p = p.offset(1);
        p;
    }
    p = p.offset(1);
    p;
    match find_option_type(
        level_indent_optype.as_mut_ptr(),
        str,
        (p.offset_from(str) as i64 - 1 as i32 as i64) as i32,
    ) {
        1 => {
            parse_level_string(p, &mut level_begin);
        }
        2 => {
            parse_level_string(
                p,
                &mut *level_indent.as_mut_ptr().offset(0 as i32 as isize),
            );
        }
        3 => {
            parse_level_string(
                p,
                &mut *level_indent.as_mut_ptr().offset(1 as i32 as isize),
            );
        }
        4 => {
            parse_level_string(
                p,
                &mut *level_end.as_mut_ptr().offset(0 as i32 as isize),
            );
        }
        5 => {
            parse_level_string(
                p,
                &mut *level_end.as_mut_ptr().offset(1 as i32 as isize),
            );
        }
        _ => {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"unknown level indent option: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                str,
            );
        }
    };
}
unsafe extern "C" fn add_name(mut name: *const i8) {
    linked_list_append(&mut arglist, name as *mut libc::c_void);
}
unsafe extern "C" fn add_preproc_option(mut key: i32, mut arg: *const i8) {
    let mut opt: *mut i8 = xmalloc((3 as i32 as u64).wrapping_add(strlen(arg)))
        as *mut i8;
    sprintf(opt, b"-%c%s\0" as *const u8 as *const i8, key, arg);
    add_name(opt);
    preprocess_option = 1 as i32;
}
unsafe extern "C" fn parse_opt(
    mut key: i32,
    mut arg: *mut i8,
    mut state: *mut argp_state,
) -> error_t {
    let mut num: i32 = 0;
    match key {
        97 => {
            strict_ansi = 1 as i32;
        }
        263 => {
            strict_ansi = 0 as i32;
        }
        258 => {
            debug = if !arg.is_null() { atoi(arg) } else { 1 as i32 };
        }
        80 => {
            set_print_option(arg);
        }
        83 => {
            use_indentation = 1 as i32;
        }
        262 => {
            use_indentation = 0 as i32;
        }
        84 => {
            print_as_tree = 1 as i32;
            set_level_indent(b"0=  \0" as *const u8 as *const i8);
            set_level_indent(b"1=| \0" as *const u8 as *const i8);
            set_level_indent(b"end0=+-\0" as *const u8 as *const i8);
            set_level_indent(b"end1=\\\\-\0" as *const u8 as *const i8);
        }
        264 => {
            print_as_tree = 0 as i32;
            level_indent[1 as i32 as usize] = 0 as *mut i8;
            level_indent[0 as i32 as usize] = level_indent[1 as i32 as usize];
            level_end[0 as i32 as usize] = 0 as *mut i8;
            level_end[0 as i32 as usize] = level_end[0 as i32 as usize];
        }
        98 => {
            brief_listing = 1 as i32;
        }
        265 => {
            brief_listing = 0 as i32;
        }
        100 => {
            max_depth = atoi(arg);
            if max_depth < 0 as i32 {
                max_depth = 0 as i32;
            }
        }
        256 => {
            record_defines = 1 as i32;
        }
        261 => {
            emacs_option = 1 as i32;
        }
        266 => {
            emacs_option = 0 as i32;
        }
        102 => {
            if select_output_driver(arg) != 0 {
                argp_error(
                    state,
                    dcgettext(
                        0 as *const i8,
                        b"%s: No such output driver\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    rpl_optarg,
                );
            }
            output_init();
        }
        257 => {
            set_level_indent(arg);
        }
        105 => {
            num = 1 as i32;
            while *arg != 0 {
                match *arg as i32 {
                    45 | 94 => {
                        num = 0 as i32;
                    }
                    43 => {
                        num = 1 as i32;
                    }
                    120 | 95 | 115 | 116 | 117 => {
                        if num != 0 {
                            symbol_map
                                |= if *arg as i32 == 'x' as i32 {
                                    0x2 as i32
                                } else if *arg as i32 == '_' as i32 {
                                    0x8 as i32
                                } else if *arg as i32 == 's' as i32 {
                                    0x4 as i32
                                } else if *arg as i32 == 't' as i32 {
                                    0x10 as i32
                                } else if *arg as i32 == 'u' as i32 {
                                    0x20 as i32
                                } else {
                                    0 as i32
                                };
                        } else {
                            symbol_map
                                &= !if *arg as i32 == 'x' as i32 {
                                    0x2 as i32
                                } else if *arg as i32 == '_' as i32 {
                                    0x8 as i32
                                } else if *arg as i32 == 's' as i32 {
                                    0x4 as i32
                                } else if *arg as i32 == 't' as i32 {
                                    0x10 as i32
                                } else if *arg as i32 == 'u' as i32 {
                                    0x20 as i32
                                } else {
                                    0 as i32
                                };
                        }
                    }
                    _ => {
                        argp_error(
                            state,
                            dcgettext(
                                0 as *const i8,
                                b"Unknown symbol class: %c\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            *arg as i32,
                        );
                    }
                }
                arg = arg.offset(1);
                arg;
            }
        }
        271 => {
            omit_arguments_option = 1 as i32;
        }
        272 => {
            omit_arguments_option = 0 as i32;
        }
        273 => {
            omit_symbol_names_option = 1 as i32;
        }
        274 => {
            omit_symbol_names_option = 0 as i32;
        }
        108 => {
            print_levels = 1 as i32;
        }
        269 => {
            print_levels = 0 as i32;
        }
        109 => {
            start_name = strdup(arg);
        }
        110 => {
            print_line_numbers = 1 as i32;
        }
        268 => {
            print_line_numbers = 0 as i32;
        }
        111 => {
            outname = strdup(arg);
        }
        112 => {
            num = atoi(arg);
            if num > 0 as i32 {
                token_stack_length = num;
            }
        }
        114 => {
            reverse_tree = 1 as i32;
        }
        270 => {
            reverse_tree = 0 as i32;
        }
        115 => {
            symbol_override(arg);
        }
        118 => {
            verbose = 1 as i32;
        }
        267 => {
            verbose = 0 as i32;
        }
        120 => {
            print_option = 0x1 as i32;
        }
        259 => {
            preprocess_option = 1 as i32;
            set_preprocessor(
                if !arg.is_null() {
                    arg
                } else {
                    b"/usr/bin/cpp\0" as *const u8 as *const i8
                },
            );
        }
        260 => {
            preprocess_option = 0 as i32;
        }
        0 => {
            add_name(arg);
        }
        73 | 68 | 85 => {
            add_preproc_option(key, arg);
        }
        _ => return 7 as i32,
    }
    return 0 as i32;
}
static mut argp: argp = unsafe {
    {
        let mut init = argp {
            options: options.as_ptr() as *mut _,
            parser: Some(
                parse_opt
                    as unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
            ),
            args_doc: b"[FILE]...\0" as *const u8 as *const i8,
            doc: doc.as_ptr() as *mut _,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const i8,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn globals_only() -> i32 {
    return (symbol_map & 0x4 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn include_symbol(mut sym: *mut Symbol) -> i32 {
    let mut type_0: i32 = 0 as i32;
    if sym.is_null() {
        return 0 as i32;
    }
    if (*sym).type_0 as u32 == symtype::SymIdentifier as i32 as u32 {
        if *((*sym).name).offset(0 as i32 as isize) as i32 == '_' as i32
            && symbol_map & 0x8 as i32 == 0
        {
            return 0 as i32;
        }
        if (*sym).storage as u32 == storage::StaticStorage as i32 as u32 {
            type_0 |= 0x4 as i32;
        }
        if (*sym).arity == -(1 as i32)
            && (*sym).storage as u32 != storage::AutoStorage as i32 as u32
        {
            type_0 |= 0x2 as i32;
        } else if (*sym).arity >= 0 as i32 {
            type_0 |= 0x1 as i32;
        }
        if ((*sym).source).is_null() {
            type_0 |= 0x20 as i32;
        }
    } else if (*sym).type_0 as u32 == symtype::SymToken as i32 as u32 {
        if (*sym).token_type == 270 as i32 && !((*sym).source).is_null() {
            type_0 |= 0x10 as i32;
        } else {
            return 0 as i32
        }
    }
    return (symbol_map & type_0 == type_0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xalloc_die() -> ! {
    error(
        1 as i32,
        12 as i32,
        dcgettext(0 as *const i8, b"Exiting\0" as *const u8 as *const i8, 5 as i32),
    );
}
#[no_mangle]
pub unsafe extern "C" fn init() {
    if (level_indent[0 as i32 as usize]).is_null() {
        level_indent[0 as i32 as usize] = b"    \0" as *const u8 as *const i8 as *mut i8;
    }
    if (level_indent[1 as i32 as usize]).is_null() {
        level_indent[1 as i32 as usize] = level_indent[0 as i32 as usize];
    }
    if (level_end[0 as i32 as usize]).is_null() {
        level_end[0 as i32 as usize] = b"\0" as *const u8 as *const i8 as *mut i8;
    }
    if (level_end[1 as i32 as usize]).is_null() {
        level_end[1 as i32 as usize] = b"\0" as *const u8 as *const i8 as *mut i8;
    }
    init_lex((debug > 1 as i32) as i32);
    init_parse();
}
#[no_mangle]
pub static mut version_etc_copyright: [i8; 63] = unsafe {
    *::core::mem::transmute::<
        &[u8; 63],
        &[i8; 63],
    >(b"Copyright %s 2005, 2006, 2009, 2010, 2011 %d Sergey Poznyakoff\0")
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut index: i32 = 0;
    set_program_name(*argv.offset(0 as i32 as isize));
    argp_version_setup(
        b"cflow\0" as *const u8 as *const i8,
        program_authors.as_mut_ptr(),
    );
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"cflow\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"cflow\0" as *const u8 as *const i8);
    register_output(
        b"gnu\0" as *const u8 as *const i8,
        Some(
            gnu_output_handler
                as unsafe extern "C" fn(
                    cflow_output_command,
                    *mut FILE,
                    i32,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> i32,
        ),
        0 as *mut libc::c_void,
    );
    register_output(
        b"posix\0" as *const u8 as *const i8,
        Some(
            posix_output_handler
                as unsafe extern "C" fn(
                    cflow_output_command,
                    *mut FILE,
                    i32,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> i32,
        ),
        0 as *mut libc::c_void,
    );
    symbol_map = 0x1 as i32 | 0x4 as i32 | 0x20 as i32;
    if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null() {
        if select_output_driver(b"posix\0" as *const u8 as *const i8) != 0 {
            error(
                1 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: No such output driver\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                b"posix\0" as *const u8 as *const i8,
            );
        }
        output_init();
    }
    sourcerc(&mut argc, &mut argv);
    if argp_parse(
        &mut argp,
        argc,
        argv,
        0x8 as i32 as u32,
        &mut index,
        0 as *mut libc::c_void,
    ) != 0
    {
        exit(1 as i32);
    }
    if print_option == 0 as i32 {
        print_option = 0x2 as i32;
    }
    init();
    if !arglist.is_null() {
        let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
        p = (*arglist).head;
        while !p.is_null() {
            let mut s: *mut i8 = (*p).data as *mut i8;
            if *s.offset(0 as i32 as isize) as i32 == '-' as i32 {
                pp_option(s);
            } else if source(s) == 0 as i32 {
                yyparse();
            }
            p = (*p).next;
        }
    }
    argc -= index;
    argv = argv.offset(index as isize);
    loop {
        let fresh14 = argc;
        argc = argc - 1;
        if !(fresh14 != 0) {
            break;
        }
        let fresh15 = argv;
        argv = argv.offset(1);
        if source(*fresh15) == 0 as i32 {
            yyparse();
        }
    }
    if input_file_count == 0 as i32 as u32 {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"no input files\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    output();
    return 0 as i32;
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