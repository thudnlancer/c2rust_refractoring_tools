use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type table_entry;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> i32;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn isatty(__fd: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn _obstack_newchunk(_: *mut obstack, _: i32);
    fn _obstack_begin(
        _: *mut obstack,
        _: i32,
        _: i32,
        _: Option<unsafe extern "C" fn(i64) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    fn obstack_free(obstack: *mut obstack, block: *mut libc::c_void);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn error_at_line(
        __status: i32,
        __errnum: i32,
        __fname: *const i8,
        __lineno: u32,
        __format: *const i8,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut debug: i32;
    static mut preprocess_option: i32;
    fn lookup(_: *const i8) -> *mut Symbol;
    fn install(_: *mut i8, _: i32) -> *mut Symbol;
    fn delete_statics();
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type size_t = u64;
pub type __int16_t = libc::c_short;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: i64,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed,
    pub alignment_mask: i32,
    pub chunkfun: Option<
        unsafe extern "C" fn(*mut libc::c_void, i64) -> *mut _obstack_chunk,
    >,
    pub freefun: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut _obstack_chunk) -> (),
    >,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub tempint: i64,
    pub tempptr: *mut libc::c_void,
}
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type flex_int16_t = int16_t;
pub type yy_state_type = i32;
pub type YY_CHAR = u8;
pub type flex_int32_t = int32_t;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut i8,
    pub yy_buf_pos: *mut i8,
    pub yy_buf_size: yy_size_t,
    pub yy_n_chars: i32,
    pub yy_is_our_buffer: i32,
    pub yy_is_interactive: i32,
    pub yy_at_bol: i32,
    pub yy_bs_lineno: i32,
    pub yy_bs_column: i32,
    pub yy_fill_buffer: i32,
    pub yy_buffer_status: i32,
}
pub type yy_size_t = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTYPE {
    pub str_0: *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    _ISspace = 8192,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::_ISspace => 8192,
            C2RustUnnamed_0::_ISalnum => 8,
            C2RustUnnamed_0::_ISpunct => 4,
            C2RustUnnamed_0::_IScntrl => 2,
            C2RustUnnamed_0::_ISblank => 1,
            C2RustUnnamed_0::_ISgraph => 32768,
            C2RustUnnamed_0::_ISprint => 16384,
            C2RustUnnamed_0::_ISxdigit => 4096,
            C2RustUnnamed_0::_ISdigit => 2048,
            C2RustUnnamed_0::_ISalpha => 1024,
            C2RustUnnamed_0::_ISlower => 512,
            C2RustUnnamed_0::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            8192 => C2RustUnnamed_0::_ISspace,
            8 => C2RustUnnamed_0::_ISalnum,
            4 => C2RustUnnamed_0::_ISpunct,
            2 => C2RustUnnamed_0::_IScntrl,
            1 => C2RustUnnamed_0::_ISblank,
            32768 => C2RustUnnamed_0::_ISgraph,
            16384 => C2RustUnnamed_0::_ISprint,
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
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut string_stk: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const i8 as *mut i8,
    next_free: 0 as *const i8 as *mut i8,
    chunk_limit: 0 as *const i8 as *mut i8,
    temp: C2RustUnnamed { tempint: 0 },
    alignment_mask: 0,
    chunkfun: None,
    freefun: None,
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
#[no_mangle]
pub static mut line_num: i32 = 0;
#[no_mangle]
pub static mut filename: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut canonical_filename: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    str_0: 0 as *const i8 as *mut i8,
};
#[no_mangle]
pub static mut input_file_count: u32 = 0;
static mut prev_token: i32 = 0;
static mut keywords: [*mut i8; 13] = [
    b"break\0" as *const u8 as *const i8 as *mut i8,
    b"case\0" as *const u8 as *const i8 as *mut i8,
    b"continue\0" as *const u8 as *const i8 as *mut i8,
    b"default\0" as *const u8 as *const i8 as *mut i8,
    b"do\0" as *const u8 as *const i8 as *mut i8,
    b"else\0" as *const u8 as *const i8 as *mut i8,
    b"for\0" as *const u8 as *const i8 as *mut i8,
    b"goto\0" as *const u8 as *const i8 as *mut i8,
    b"if\0" as *const u8 as *const i8 as *mut i8,
    b"return\0" as *const u8 as *const i8 as *mut i8,
    b"sizeof\0" as *const u8 as *const i8 as *mut i8,
    b"switch\0" as *const u8 as *const i8 as *mut i8,
    b"while\0" as *const u8 as *const i8 as *mut i8,
];
static mut types: [*mut i8; 5] = [
    b"char\0" as *const u8 as *const i8 as *mut i8,
    b"double\0" as *const u8 as *const i8 as *mut i8,
    b"float\0" as *const u8 as *const i8 as *mut i8,
    b"int\0" as *const u8 as *const i8 as *mut i8,
    b"void\0" as *const u8 as *const i8 as *mut i8,
];
static mut qualifiers: [*mut i8; 9] = [
    b"long\0" as *const u8 as *const i8 as *mut i8,
    b"const\0" as *const u8 as *const i8 as *mut i8,
    b"register\0" as *const u8 as *const i8 as *mut i8,
    b"restrict\0" as *const u8 as *const i8 as *mut i8,
    b"short\0" as *const u8 as *const i8 as *mut i8,
    b"signed\0" as *const u8 as *const i8 as *mut i8,
    b"unsigned\0" as *const u8 as *const i8 as *mut i8,
    b"volatile\0" as *const u8 as *const i8 as *mut i8,
    b"inline\0" as *const u8 as *const i8 as *mut i8,
];
#[no_mangle]
pub unsafe extern "C" fn init_tokens() {
    let mut i: i32 = 0;
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[*mut i8; 13]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut i8>() as u64)
    {
        sp = install(keywords[i as usize], 0x1 as i32);
        (*sp).type_0 = symtype::SymToken;
        (*sp).token_type = 257 as i32;
        i += 1;
        i;
    }
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[*mut i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut i8>() as u64)
    {
        sp = install(types[i as usize], 0x1 as i32);
        (*sp).type_0 = symtype::SymToken;
        (*sp).token_type = 270 as i32;
        (*sp).source = 0 as *mut i8;
        (*sp).def_line = -(1 as i32);
        (*sp).ref_line = 0 as *mut linked_list;
        i += 1;
        i;
    }
    i = 0 as i32;
    while (i as u64)
        < (::core::mem::size_of::<[*mut i8; 9]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut i8>() as u64)
    {
        sp = install(qualifiers[i as usize], 0x1 as i32);
        (*sp).type_0 = symtype::SymToken;
        (*sp).token_type = 273 as i32;
        (*sp).source = 0 as *mut i8;
        (*sp).def_line = -(1 as i32);
        (*sp).ref_line = 0 as *mut linked_list;
        i += 1;
        i;
    }
    sp = install(b"...\0" as *const u8 as *const i8 as *mut i8, 0x1 as i32);
    (*sp).type_0 = symtype::SymToken;
    (*sp).token_type = 260 as i32;
    (*sp).source = 0 as *mut i8;
    (*sp).def_line = -(1 as i32);
    (*sp).ref_line = 0 as *mut linked_list;
}
#[no_mangle]
pub unsafe extern "C" fn init_lex(mut debug_level: i32) {
    yy_flex_debug = debug_level;
    _obstack_begin(
        &mut string_stk,
        0 as i32,
        0 as i32,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
            Option<unsafe extern "C" fn(i64) -> *mut libc::c_void>,
        >(Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void)),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())),
    );
    init_tokens();
}
#[no_mangle]
pub unsafe extern "C" fn ident() -> i32 {
    if prev_token != 264 as i32 {
        let mut sp: *mut Symbol = lookup(yytext);
        if !sp.is_null() && (*sp).type_0 as u32 == symtype::SymToken as i32 as u32 {
            yylval.str_0 = (*sp).name;
            return (*sp).token_type;
        }
    }
    let mut __o: *mut obstack = &mut string_stk;
    let mut __len: i32 = yyleng;
    if ((*__o).next_free).offset(__len as isize) > (*__o).chunk_limit {
        _obstack_newchunk(__o, __len);
    }
    memcpy(
        (*__o).next_free as *mut libc::c_void,
        yytext as *const libc::c_void,
        __len as u64,
    );
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    let mut __o_0: *mut obstack = &mut string_stk;
    if ((*__o_0).next_free).offset(1 as i32 as isize) > (*__o_0).chunk_limit {
        _obstack_newchunk(__o_0, 1 as i32);
    }
    let fresh0 = (*__o_0).next_free;
    (*__o_0).next_free = ((*__o_0).next_free).offset(1);
    *fresh0 = 0 as i32 as i8;
    yylval.str_0 = ({
        let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<i64>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                (((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<i64>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 + (*__o1).alignment_mask as i64
                    & !(*__o1).alignment_mask as i64) as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut i8;
    return 260 as i32;
}
#[no_mangle]
pub static mut pp_bin: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut pp_opts: *mut i8 = 0 as *const i8 as *mut i8;
static mut opt_stack: *mut obstack = 0 as *const obstack as *mut obstack;
#[no_mangle]
pub unsafe extern "C" fn set_preprocessor(mut arg: *const i8) {
    pp_bin = if !arg.is_null() { xstrdup(arg) } else { 0 as *mut i8 };
}
#[no_mangle]
pub unsafe extern "C" fn pp_option(mut arg: *const i8) {
    if opt_stack.is_null() {
        if pp_bin.is_null() {
            pp_bin = b"/usr/bin/cpp\0" as *const u8 as *const i8 as *mut i8;
        }
        opt_stack = xmalloc(::core::mem::size_of::<obstack>() as u64) as *mut obstack;
        _obstack_begin(
            opt_stack,
            0 as i32,
            0 as i32,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
                Option<unsafe extern "C" fn(i64) -> *mut libc::c_void>,
            >(Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void)),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())),
        );
    }
    let mut __o: *mut obstack = opt_stack;
    if ((*__o).next_free).offset(1 as i32 as isize) > (*__o).chunk_limit {
        _obstack_newchunk(__o, 1 as i32);
    }
    let fresh1 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh1 = ' ' as i32 as i8;
    let mut __o_0: *mut obstack = opt_stack;
    let mut __len: i32 = strlen(arg) as i32;
    if ((*__o_0).next_free).offset(__len as isize) > (*__o_0).chunk_limit {
        _obstack_newchunk(__o_0, __len);
    }
    memcpy(
        (*__o_0).next_free as *mut libc::c_void,
        arg as *const libc::c_void,
        __len as u64,
    );
    (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
}
#[no_mangle]
pub unsafe extern "C" fn pp_finalize() {
    let mut s: *mut i8 = ({
        let mut __o1: *mut obstack = opt_stack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<i64>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                (((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<i64>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 + (*__o1).alignment_mask as i64
                    & !(*__o1).alignment_mask as i64) as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut i8;
    if pp_opts.is_null() {
        pp_opts = xstrdup(s);
    } else {
        pp_opts = xrealloc(
            pp_opts as *mut libc::c_void,
            (strlen(pp_opts)).wrapping_add(strlen(s)).wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        strcat(pp_opts, s);
    }
    let mut __o: *mut obstack = opt_stack;
    let mut __obj: *mut libc::c_void = s as *mut libc::c_void;
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        obstack_free(__o, __obj);
    }
    free(opt_stack as *mut libc::c_void);
    opt_stack = 0 as *mut obstack;
}
static mut yy_buffer_stack_top: size_t = 0 as i32 as size_t;
static mut yy_buffer_stack_max: size_t = 0 as i32 as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
#[no_mangle]
pub unsafe extern "C" fn pp_open(mut name: *const i8) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0;
    if !opt_stack.is_null() {
        pp_finalize();
    }
    size = (strlen(pp_bin))
        .wrapping_add(1 as i32 as u64)
        .wrapping_add(strlen(name))
        .wrapping_add(1 as i32 as u64);
    if !pp_opts.is_null() {
        size = (size as u64).wrapping_add(strlen(pp_opts)) as size_t as size_t;
    }
    s = xmalloc(size) as *mut i8;
    strcpy(s, pp_bin);
    if !pp_opts.is_null() {
        strcat(s, pp_opts);
    }
    strcat(s, b" \0" as *const u8 as *const i8);
    strcat(s, name);
    if debug != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"Command line: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            s,
        );
    }
    fp = popen(s, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot execute `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            s,
        );
    }
    free(s as *mut libc::c_void);
    return fp;
}
static mut yy_hold_char: i8 = 0;
#[no_mangle]
pub unsafe extern "C" fn pp_close(mut fp: *mut FILE) {
    pclose(fp);
}
static mut yy_n_chars: i32 = 0;
#[no_mangle]
pub static mut yyleng: i32 = 0;
static mut yy_c_buf_p: *mut i8 = 0 as *const i8 as *mut i8;
static mut yy_init: i32 = 0 as i32;
static mut yy_start: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn yywrap() -> i32 {
    if yyin.is_null() {
        return 1 as i32;
    }
    if preprocess_option != 0 {
        pp_close(yyin);
    } else {
        fclose(yyin);
    }
    yyin = 0 as *mut FILE;
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    delete_statics();
    return 1 as i32;
}
static mut yy_did_buffer_switch_on_eof: i32 = 0;
static mut hit_eof: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn get_token() -> i32 {
    let mut tok: i32 = 0;
    if hit_eof != 0 {
        tok = 0 as i32;
    } else {
        tok = yylex();
        prev_token = tok;
        if tok == 0 {
            hit_eof = 1 as i32;
        }
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn source(mut name: *mut i8) -> i32 {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"r\0" as *const u8 as *const i8);
    if fp.is_null() {
        error(
            0 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot open `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            name,
        );
        return 1 as i32;
    }
    if preprocess_option != 0 {
        fclose(fp);
        fp = pp_open(name);
        if fp.is_null() {
            return 1 as i32;
        }
    }
    let mut __o: *mut obstack = &mut string_stk;
    let mut __len: i32 = (strlen(name)).wrapping_add(1 as i32 as u64) as i32;
    if ((*__o).next_free).offset(__len as isize) > (*__o).chunk_limit {
        _obstack_newchunk(__o, __len);
    }
    memcpy(
        (*__o).next_free as *mut libc::c_void,
        name as *const libc::c_void,
        __len as u64,
    );
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    filename = ({
        let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<i64>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                (((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<i64>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 + (*__o1).alignment_mask as i64
                    & !(*__o1).alignment_mask as i64) as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut i8;
    canonical_filename = filename;
    line_num = 1 as i32;
    input_file_count = input_file_count.wrapping_add(1);
    input_file_count;
    hit_eof = 0 as i32;
    yyrestart(fp);
    return 0 as i32;
}
unsafe extern "C" fn getnum(mut base: u32, mut count: i32) -> i32 {
    let mut c: i32 = 0;
    let mut n: i32 = 0;
    let mut i: u32 = 0;
    n = 0 as i32;
    while count != 0 {
        c = input();
        if *(*__ctype_b_loc()).offset(c as isize) as i32
            & C2RustUnnamed_0::_ISdigit as i32 as libc::c_ushort as i32 != 0
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
        if i > base {
            yyunput(c, yytext);
            break;
        } else {
            n = (n as u32).wrapping_mul(base).wrapping_add(i) as i32;
            count -= 1;
            count;
        }
    }
    return n;
}
#[no_mangle]
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut yylineno: i32 = 1 as i32;
#[no_mangle]
pub unsafe extern "C" fn backslash() -> i32 {
    let mut c: i32 = 0;
    c = input();
    match c {
        97 => return '\u{7}' as i32,
        98 => return '\u{8}' as i32,
        102 => return '\u{c}' as i32,
        110 => return '\n' as i32,
        114 => return '\r' as i32,
        116 => return '\t' as i32,
        120 => return getnum(16 as i32 as u32, 2 as i32),
        48 => return getnum(8 as i32 as u32, 3 as i32),
        _ => {}
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn update_loc() {
    let mut p: *mut i8 = 0 as *mut i8;
    p = (strchr(yytext, '#' as i32)).offset(1 as i32 as isize);
    while *p as i32 != 0
        && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        p = p.offset(1);
        p;
    }
    if *p.offset(0 as i32 as isize) as i32 == 'l' as i32 {
        p = p.offset(4 as i32 as isize);
    }
    line_num = strtoul(p, &mut p, 10 as i32) as i32;
    while *p as i32 != 0
        && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & C2RustUnnamed_0::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        p = p.offset(1);
        p;
    }
    if *p.offset(0 as i32 as isize) as i32 == '"' as i32 {
        let mut n: i32 = 0;
        p = p.offset(1);
        p;
        n = 0 as i32;
        while *p.offset(n as isize) as i32 != 0
            && *p.offset(n as isize) as i32 != '"' as i32
        {
            n += 1;
            n;
        }
        let mut __o: *mut obstack = &mut string_stk;
        let mut __len: i32 = n;
        if ((*__o).next_free).offset(__len as isize) > (*__o).chunk_limit {
            _obstack_newchunk(__o, __len);
        }
        memcpy(
            (*__o).next_free as *mut libc::c_void,
            p as *const libc::c_void,
            __len as u64,
        );
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        let mut __o_0: *mut obstack = &mut string_stk;
        if ((*__o_0).next_free).offset(1 as i32 as isize) > (*__o_0).chunk_limit {
            _obstack_newchunk(__o_0, 1 as i32);
        }
        let fresh2 = (*__o_0).next_free;
        (*__o_0).next_free = ((*__o_0).next_free).offset(1);
        *fresh2 = 0 as i32 as i8;
        filename = ({
            let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<i64>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    (((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<i64>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 + (*__o1).alignment_mask as i64
                        & !(*__o1).alignment_mask as i64) as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut i8;
    }
    if debug > 1 as i32 {
        printf(
            dcgettext(
                0 as *const i8,
                b"New location: %s:%d\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
            line_num,
        );
    }
}
static mut yy_accept: [flex_int16_t; 191] = [
    0 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    76 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    28 as i32 as flex_int16_t,
    42 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    21 as i32 as flex_int16_t,
    30 as i32 as flex_int16_t,
    32 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    44 as i32 as flex_int16_t,
    48 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    38 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    78 as i32 as flex_int16_t,
    79 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    4 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    68 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    75 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    73 as i32 as flex_int16_t,
    74 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    13 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    11 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    41 as i32 as flex_int16_t,
    35 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    53 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    29 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    2 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    25 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    63 as i32 as flex_int16_t,
    62 as i32 as flex_int16_t,
    62 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    49 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    47 as i32 as flex_int16_t,
    52 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    37 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    36 as i32 as flex_int16_t,
    39 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    4 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    6 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    7 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    13 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    12 as i32 as flex_int16_t,
    55 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    55 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    59 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    63 as i32 as flex_int16_t,
    33 as i32 as flex_int16_t,
    34 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    9 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    10 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    14 as i32 as flex_int16_t,
    20 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    9 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    57 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    19 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    15 as i32 as flex_int16_t,
    16 as i32 as flex_int16_t,
    18 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    17 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    8 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    8 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
];
static mut yy_ec: [flex_int32_t; 256] = [
    0 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    1 as i32,
    2 as i32,
    2 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    4 as i32,
    5 as i32,
    6 as i32,
    1 as i32,
    7 as i32,
    8 as i32,
    9 as i32,
    1 as i32,
    1 as i32,
    10 as i32,
    11 as i32,
    1 as i32,
    12 as i32,
    13 as i32,
    14 as i32,
    15 as i32,
    16 as i32,
    16 as i32,
    16 as i32,
    16 as i32,
    16 as i32,
    16 as i32,
    16 as i32,
    17 as i32,
    17 as i32,
    1 as i32,
    1 as i32,
    18 as i32,
    19 as i32,
    20 as i32,
    1 as i32,
    1 as i32,
    21 as i32,
    21 as i32,
    21 as i32,
    21 as i32,
    22 as i32,
    21 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    23 as i32,
    24 as i32,
    23 as i32,
    23 as i32,
    1 as i32,
    25 as i32,
    1 as i32,
    26 as i32,
    23 as i32,
    1 as i32,
    27 as i32,
    21 as i32,
    28 as i32,
    29 as i32,
    30 as i32,
    31 as i32,
    23 as i32,
    23 as i32,
    32 as i32,
    23 as i32,
    23 as i32,
    33 as i32,
    34 as i32,
    35 as i32,
    36 as i32,
    37 as i32,
    23 as i32,
    38 as i32,
    39 as i32,
    40 as i32,
    41 as i32,
    23 as i32,
    23 as i32,
    42 as i32,
    43 as i32,
    23 as i32,
    44 as i32,
    45 as i32,
    46 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
];
static mut yy_meta: [flex_int32_t; 47] = [
    0 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    1 as i32,
    3 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    4 as i32,
    5 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
    6 as i32,
    7 as i32,
    8 as i32,
    8 as i32,
    3 as i32,
    1 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    7 as i32,
    6 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
    1 as i32,
    1 as i32,
    1 as i32,
];
static mut yy_base: [flex_int16_t; 212] = [
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    47 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    62 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    469 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    424 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    417 as i32 as flex_int16_t,
    400 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    381 as i32 as flex_int16_t,
    370 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    88 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    93 as i32 as flex_int16_t,
    98 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    357 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    314 as i32 as flex_int16_t,
    291 as i32 as flex_int16_t,
    52 as i32 as flex_int16_t,
    291 as i32 as flex_int16_t,
    287 as i32 as flex_int16_t,
    294 as i32 as flex_int16_t,
    79 as i32 as flex_int16_t,
    110 as i32 as flex_int16_t,
    134 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    325 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    324 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    321 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    117 as i32 as flex_int16_t,
    120 as i32 as flex_int16_t,
    319 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    318 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    311 as i32 as flex_int16_t,
    123 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    128 as i32 as flex_int16_t,
    139 as i32 as flex_int16_t,
    145 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    305 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    314 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    164 as i32 as flex_int16_t,
    172 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    296 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    295 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    262 as i32 as flex_int16_t,
    260 as i32 as flex_int16_t,
    258 as i32 as flex_int16_t,
    96 as i32 as flex_int16_t,
    258 as i32 as flex_int16_t,
    254 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    125 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    173 as i32 as flex_int16_t,
    225 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    181 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    272 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    143 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    209 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    270 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    259 as i32 as flex_int16_t,
    226 as i32 as flex_int16_t,
    216 as i32 as flex_int16_t,
    225 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    221 as i32 as flex_int16_t,
    237 as i32 as flex_int16_t,
    227 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    244 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    184 as i32 as flex_int16_t,
    173 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    157 as i32 as flex_int16_t,
    141 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    254 as i32 as flex_int16_t,
    214 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    241 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    258 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    230 as i32 as flex_int16_t,
    247 as i32 as flex_int16_t,
    265 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    125 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    95 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    262 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    268 as i32 as flex_int16_t,
    273 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    74 as i32 as flex_int16_t,
    48 as i32 as flex_int16_t,
    44 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    291 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    42 as i32 as flex_int16_t,
    294 as i32 as flex_int16_t,
    288 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    274 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    309 as i32 as flex_int16_t,
    275 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    484 as i32 as flex_int16_t,
    334 as i32 as flex_int16_t,
    342 as i32 as flex_int16_t,
    350 as i32 as flex_int16_t,
    358 as i32 as flex_int16_t,
    366 as i32 as flex_int16_t,
    374 as i32 as flex_int16_t,
    382 as i32 as flex_int16_t,
    385 as i32 as flex_int16_t,
    393 as i32 as flex_int16_t,
    401 as i32 as flex_int16_t,
    409 as i32 as flex_int16_t,
    417 as i32 as flex_int16_t,
    425 as i32 as flex_int16_t,
    433 as i32 as flex_int16_t,
    441 as i32 as flex_int16_t,
    449 as i32 as flex_int16_t,
    452 as i32 as flex_int16_t,
    459 as i32 as flex_int16_t,
    464 as i32 as flex_int16_t,
    468 as i32 as flex_int16_t,
    475 as i32 as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 212] = [
    0 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    207 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    209 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    207 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    210 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 531] = [
    0 as i32 as flex_int16_t,
    12 as i32 as flex_int16_t,
    13 as i32 as flex_int16_t,
    14 as i32 as flex_int16_t,
    15 as i32 as flex_int16_t,
    16 as i32 as flex_int16_t,
    17 as i32 as flex_int16_t,
    18 as i32 as flex_int16_t,
    19 as i32 as flex_int16_t,
    20 as i32 as flex_int16_t,
    21 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    25 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    28 as i32 as flex_int16_t,
    29 as i32 as flex_int16_t,
    30 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    12 as i32 as flex_int16_t,
    32 as i32 as flex_int16_t,
    33 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    34 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    35 as i32 as flex_int16_t,
    36 as i32 as flex_int16_t,
    37 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    31 as i32 as flex_int16_t,
    12 as i32 as flex_int16_t,
    38 as i32 as flex_int16_t,
    12 as i32 as flex_int16_t,
    39 as i32 as flex_int16_t,
    44 as i32 as flex_int16_t,
    44 as i32 as flex_int16_t,
    47 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    48 as i32 as flex_int16_t,
    55 as i32 as flex_int16_t,
    55 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    47 as i32 as flex_int16_t,
    57 as i32 as flex_int16_t,
    48 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    52 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    53 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    52 as i32 as flex_int16_t,
    63 as i32 as flex_int16_t,
    53 as i32 as flex_int16_t,
    87 as i32 as flex_int16_t,
    88 as i32 as flex_int16_t,
    90 as i32 as flex_int16_t,
    91 as i32 as flex_int16_t,
    49 as i32 as flex_int16_t,
    184 as i32 as flex_int16_t,
    181 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    68 as i32 as flex_int16_t,
    49 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    78 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    79 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    95 as i32 as flex_int16_t,
    180 as i32 as flex_int16_t,
    41 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    42 as i32 as flex_int16_t,
    73 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    96 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    100 as i32 as flex_int16_t,
    74 as i32 as flex_int16_t,
    75 as i32 as flex_int16_t,
    76 as i32 as flex_int16_t,
    179 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    178 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    102 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    84 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    120 as i32 as flex_int16_t,
    122 as i32 as flex_int16_t,
    168 as i32 as flex_int16_t,
    57 as i32 as flex_int16_t,
    140 as i32 as flex_int16_t,
    101 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    102 as i32 as flex_int16_t,
    166 as i32 as flex_int16_t,
    114 as i32 as flex_int16_t,
    176 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    125 as i32 as flex_int16_t,
    175 as i32 as flex_int16_t,
    141 as i32 as flex_int16_t,
    84 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    126 as i32 as flex_int16_t,
    126 as i32 as flex_int16_t,
    174 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    127 as i32 as flex_int16_t,
    173 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    172 as i32 as flex_int16_t,
    168 as i32 as flex_int16_t,
    127 as i32 as flex_int16_t,
    163 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    162 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    161 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    148 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    145 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    160 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    120 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    159 as i32 as flex_int16_t,
    122 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    158 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    145 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    164 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    157 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    114 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    150 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    149 as i32 as flex_int16_t,
    132 as i32 as flex_int16_t,
    151 as i32 as flex_int16_t,
    151 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    150 as i32 as flex_int16_t,
    150 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    145 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    166 as i32 as flex_int16_t,
    150 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    118 as i32 as flex_int16_t,
    167 as i32 as flex_int16_t,
    167 as i32 as flex_int16_t,
    110 as i32 as flex_int16_t,
    165 as i32 as flex_int16_t,
    186 as i32 as flex_int16_t,
    189 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    143 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    186 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    142 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    139 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    138 as i32 as flex_int16_t,
    137 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    186 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    136 as i32 as flex_int16_t,
    135 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    132 as i32 as flex_int16_t,
    128 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    123 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    118 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    117 as i32 as flex_int16_t,
    110 as i32 as flex_int16_t,
    99 as i32 as flex_int16_t,
    98 as i32 as flex_int16_t,
    97 as i32 as flex_int16_t,
    94 as i32 as flex_int16_t,
    93 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    46 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    50 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    89 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    85 as i32 as flex_int16_t,
    92 as i32 as flex_int16_t,
    92 as i32 as flex_int16_t,
    92 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    62 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    59 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    134 as i32 as flex_int16_t,
    134 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    152 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    152 as i32 as flex_int16_t,
    152 as i32 as flex_int16_t,
    169 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    169 as i32 as flex_int16_t,
    169 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    11 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 531] = [
    0 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    1 as i32 as flex_int16_t,
    2 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    4 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    2 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    9 as i32 as flex_int16_t,
    10 as i32 as flex_int16_t,
    3 as i32 as flex_int16_t,
    4 as i32 as flex_int16_t,
    6 as i32 as flex_int16_t,
    13 as i32 as flex_int16_t,
    6 as i32 as flex_int16_t,
    7 as i32 as flex_int16_t,
    7 as i32 as flex_int16_t,
    13 as i32 as flex_int16_t,
    7 as i32 as flex_int16_t,
    8 as i32 as flex_int16_t,
    8 as i32 as flex_int16_t,
    19 as i32 as flex_int16_t,
    8 as i32 as flex_int16_t,
    28 as i32 as flex_int16_t,
    28 as i32 as flex_int16_t,
    30 as i32 as flex_int16_t,
    30 as i32 as flex_int16_t,
    5 as i32 as flex_int16_t,
    181 as i32 as flex_int16_t,
    175 as i32 as flex_int16_t,
    9 as i32 as flex_int16_t,
    10 as i32 as flex_int16_t,
    19 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    6 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    25 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    25 as i32 as flex_int16_t,
    22 as i32 as flex_int16_t,
    34 as i32 as flex_int16_t,
    174 as i32 as flex_int16_t,
    2 as i32 as flex_int16_t,
    25 as i32 as flex_int16_t,
    2 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    34 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    38 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    23 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    173 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    24 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    172 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    39 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    27 as i32 as flex_int16_t,
    39 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    169 as i32 as flex_int16_t,
    57 as i32 as flex_int16_t,
    97 as i32 as flex_int16_t,
    38 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    57 as i32 as flex_int16_t,
    102 as i32 as flex_int16_t,
    167 as i32 as flex_int16_t,
    45 as i32 as flex_int16_t,
    163 as i32 as flex_int16_t,
    102 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    162 as i32 as flex_int16_t,
    97 as i32 as flex_int16_t,
    26 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    161 as i32 as flex_int16_t,
    54 as i32 as flex_int16_t,
    56 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    160 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    159 as i32 as flex_int16_t,
    152 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    143 as i32 as flex_int16_t,
    40 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    142 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    82 as i32 as flex_int16_t,
    141 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    83 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    86 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    140 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    139 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    138 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    137 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    126 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    131 as i32 as flex_int16_t,
    126 as i32 as flex_int16_t,
    126 as i32 as flex_int16_t,
    144 as i32 as flex_int16_t,
    127 as i32 as flex_int16_t,
    125 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    147 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    149 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    130 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    165 as i32 as flex_int16_t,
    149 as i32 as flex_int16_t,
    151 as i32 as flex_int16_t,
    124 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    118 as i32 as flex_int16_t,
    151 as i32 as flex_int16_t,
    151 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    149 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    146 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    99 as i32 as flex_int16_t,
    165 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    165 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    98 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    96 as i32 as flex_int16_t,
    185 as i32 as flex_int16_t,
    188 as i32 as flex_int16_t,
    95 as i32 as flex_int16_t,
    94 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    183 as i32 as flex_int16_t,
    91 as i32 as flex_int16_t,
    87 as i32 as flex_int16_t,
    177 as i32 as flex_int16_t,
    79 as i32 as flex_int16_t,
    76 as i32 as flex_int16_t,
    182 as i32 as flex_int16_t,
    65 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    51 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    49 as i32 as flex_int16_t,
    43 as i32 as flex_int16_t,
    37 as i32 as flex_int16_t,
    36 as i32 as flex_int16_t,
    35 as i32 as flex_int16_t,
    33 as i32 as flex_int16_t,
    32 as i32 as flex_int16_t,
    187 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    191 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    192 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    193 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    194 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    195 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    29 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    196 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    21 as i32 as flex_int16_t,
    197 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    198 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    199 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    20 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    200 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    201 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    18 as i32 as flex_int16_t,
    17 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    202 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    203 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    204 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    15 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    205 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    206 as i32 as flex_int16_t,
    207 as i32 as flex_int16_t,
    207 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    208 as i32 as flex_int16_t,
    209 as i32 as flex_int16_t,
    11 as i32 as flex_int16_t,
    209 as i32 as flex_int16_t,
    209 as i32 as flex_int16_t,
    210 as i32 as flex_int16_t,
    0 as i32 as flex_int16_t,
    210 as i32 as flex_int16_t,
    210 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    211 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
    190 as i32 as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut yy_flex_debug: i32 = 1 as i32;
static mut yy_rule_linenum: [flex_int16_t; 81] = [
    0 as i32 as flex_int16_t,
    58 as i32 as flex_int16_t,
    59 as i32 as flex_int16_t,
    60 as i32 as flex_int16_t,
    61 as i32 as flex_int16_t,
    62 as i32 as flex_int16_t,
    63 as i32 as flex_int16_t,
    64 as i32 as flex_int16_t,
    66 as i32 as flex_int16_t,
    67 as i32 as flex_int16_t,
    69 as i32 as flex_int16_t,
    70 as i32 as flex_int16_t,
    71 as i32 as flex_int16_t,
    72 as i32 as flex_int16_t,
    74 as i32 as flex_int16_t,
    75 as i32 as flex_int16_t,
    76 as i32 as flex_int16_t,
    77 as i32 as flex_int16_t,
    78 as i32 as flex_int16_t,
    79 as i32 as flex_int16_t,
    80 as i32 as flex_int16_t,
    81 as i32 as flex_int16_t,
    87 as i32 as flex_int16_t,
    88 as i32 as flex_int16_t,
    89 as i32 as flex_int16_t,
    90 as i32 as flex_int16_t,
    91 as i32 as flex_int16_t,
    92 as i32 as flex_int16_t,
    93 as i32 as flex_int16_t,
    94 as i32 as flex_int16_t,
    95 as i32 as flex_int16_t,
    96 as i32 as flex_int16_t,
    97 as i32 as flex_int16_t,
    98 as i32 as flex_int16_t,
    99 as i32 as flex_int16_t,
    100 as i32 as flex_int16_t,
    101 as i32 as flex_int16_t,
    102 as i32 as flex_int16_t,
    103 as i32 as flex_int16_t,
    104 as i32 as flex_int16_t,
    105 as i32 as flex_int16_t,
    106 as i32 as flex_int16_t,
    107 as i32 as flex_int16_t,
    108 as i32 as flex_int16_t,
    109 as i32 as flex_int16_t,
    110 as i32 as flex_int16_t,
    111 as i32 as flex_int16_t,
    112 as i32 as flex_int16_t,
    113 as i32 as flex_int16_t,
    114 as i32 as flex_int16_t,
    115 as i32 as flex_int16_t,
    116 as i32 as flex_int16_t,
    117 as i32 as flex_int16_t,
    118 as i32 as flex_int16_t,
    119 as i32 as flex_int16_t,
    120 as i32 as flex_int16_t,
    121 as i32 as flex_int16_t,
    122 as i32 as flex_int16_t,
    123 as i32 as flex_int16_t,
    127 as i32 as flex_int16_t,
    128 as i32 as flex_int16_t,
    129 as i32 as flex_int16_t,
    133 as i32 as flex_int16_t,
    137 as i32 as flex_int16_t,
    138 as i32 as flex_int16_t,
    139 as i32 as flex_int16_t,
    152 as i32 as flex_int16_t,
    153 as i32 as flex_int16_t,
    154 as i32 as flex_int16_t,
    155 as i32 as flex_int16_t,
    156 as i32 as flex_int16_t,
    157 as i32 as flex_int16_t,
    158 as i32 as flex_int16_t,
    159 as i32 as flex_int16_t,
    160 as i32 as flex_int16_t,
    161 as i32 as flex_int16_t,
    166 as i32 as flex_int16_t,
    167 as i32 as flex_int16_t,
    169 as i32 as flex_int16_t,
    170 as i32 as flex_int16_t,
    171 as i32 as flex_int16_t,
];
#[no_mangle]
pub static mut yytext: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn yylex() -> i32 {
    let mut yy_amount_of_matched_text: i32 = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut i8 = 0 as *mut i8;
    let mut yy_bp: *mut i8 = 0 as *mut i8;
    let mut yy_act: i32 = 0;
    if yy_init == 0 {
        yy_init = 1 as i32;
        if yy_start == 0 {
            yy_start = 1 as i32;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            yyensure_buffer_stack();
            let ref mut fresh3 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh3 = yy_create_buffer(yyin, 16384 as i32);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        yy_current_state
            += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as u8 as u32 as usize] as YY_CHAR;
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32)
                    as usize] as i32 != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as i32;
                    if yy_current_state >= 191 as i32 {
                        yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
                    .wrapping_add(yy_c as u32) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as i32 != 484 as i32) {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as i32;
                if yy_act == 0 as i32 {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as i32;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as i64 as size_t as i32;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as i8;
                yy_c_buf_p = yy_cp;
                loop {
                    if yy_flex_debug != 0 {
                        if yy_act == 0 as i32 {
                            fprintf(
                                stderr,
                                b"--scanner backing up\n\0" as *const u8 as *const i8,
                            );
                        } else if yy_act < 81 as i32 {
                            fprintf(
                                stderr,
                                b"--accepting rule at line %ld (\"%s\")\n\0" as *const u8
                                    as *const i8,
                                yy_rule_linenum[yy_act as usize] as i64,
                                yytext,
                            );
                        } else if yy_act == 81 as i32 {
                            fprintf(
                                stderr,
                                b"--accepting default rule (\"%s\")\n\0" as *const u8
                                    as *const i8,
                                yytext,
                            );
                        } else if yy_act == 82 as i32 {
                            fprintf(
                                stderr,
                                b"--(end of buffer or a NUL)\n\0" as *const u8 as *const i8,
                            );
                        } else {
                            fprintf(
                                stderr,
                                b"--EOF (start condition %d)\n\0" as *const u8 as *const i8,
                                (yy_start - 1 as i32) / 2 as i32,
                            );
                        }
                    }
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        2 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 1 as i32;
                            break '_yy_match;
                        }
                        3 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        4 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        5 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        6 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        7 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 0 as i32;
                            break '_yy_match;
                        }
                        8 | 9 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            update_loc();
                            break '_yy_match;
                        }
                        10 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 4 as i32;
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        11 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        12 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        13 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 0 as i32;
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        14 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        15 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 261 as i32;
                        }
                        16 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 262 as i32;
                        }
                        17 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 263 as i32;
                        }
                        18 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"struct\0" as *const u8 as *const i8
                                as *mut i8;
                            return 264 as i32;
                        }
                        19 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"union\0" as *const u8 as *const i8
                                as *mut i8;
                            return 264 as i32;
                        }
                        20 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"enum\0" as *const u8 as *const i8
                                as *mut i8;
                            return 264 as i32;
                        }
                        21 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"*\0" as *const u8 as *const i8 as *mut i8;
                            return 265 as i32;
                        }
                        22 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"->\0" as *const u8 as *const i8 as *mut i8;
                            return 269 as i32;
                        }
                        23 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b".\0" as *const u8 as *const i8 as *mut i8;
                            return 269 as i32;
                        }
                        24 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"*=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        25 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"/=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        26 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"/\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        27 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"%=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        28 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"%\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        29 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"+=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        30 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"+\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        31 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"-=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        32 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"-\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        33 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"<<=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        34 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b">>=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        35 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"&=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        36 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"|=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        37 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"^=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        38 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"^\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        39 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"||\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        40 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"|\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        41 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"&&\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        42 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"&\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        43 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"==\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        44 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"=\0" as *const u8 as *const i8 as *mut i8;
                            return '=' as i32;
                        }
                        45 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"!=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        46 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"!\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        47 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b">=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        48 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b">\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        49 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"<=\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        50 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"<\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        51 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"<<\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        52 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b">>\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        53 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"++\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        54 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yylval.str_0 = b"--\0" as *const u8 as *const i8 as *mut i8;
                            return 266 as i32;
                        }
                        55 | 56 | 57 | 58 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 271 as i32;
                        }
                        59 | 60 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return ident();
                        }
                        61 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            let mut __o: *mut obstack = &mut string_stk;
                            let mut __len: i32 = yyleng + 1 as i32;
                            if ((*__o).next_free).offset(__len as isize)
                                > (*__o).chunk_limit
                            {
                                _obstack_newchunk(__o, __len);
                            }
                            memcpy(
                                (*__o).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len as u64,
                            );
                            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                            yylval.str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut i8 {
                                    (*__o1).set_maybe_empty_object(1 as i32 as u32);
                                }
                                (*__o1).next_free = (if (::core::mem::size_of::<i64>()
                                    as u64) < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut i8
                                })
                                    .offset(
                                        (((*__o1).next_free)
                                            .offset_from(
                                                (if (::core::mem::size_of::<i64>() as u64)
                                                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut i8
                                                }),
                                            ) as i64 + (*__o1).alignment_mask as i64
                                            & !(*__o1).alignment_mask as i64) as isize,
                                    );
                                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8)
                                    as i64
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut i8) as i64
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut i8;
                            return 257 as i32;
                        }
                        62 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            let mut yyless_macro_arg: i32 = yyleng - 1 as i32;
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_bp
                                .offset(yyless_macro_arg as isize)
                                .offset(-(0 as i32 as isize));
                            yy_c_buf_p = yy_cp;
                            yytext = yy_bp;
                            yyleng = yy_cp.offset_from(yy_bp) as i64 as size_t as i32;
                            yy_hold_char = *yy_cp;
                            *yy_cp = '\0' as i32 as i8;
                            yy_c_buf_p = yy_cp;
                            let mut __o_0: *mut obstack = &mut string_stk;
                            let mut __len_0: i32 = yyleng + 1 as i32;
                            if ((*__o_0).next_free).offset(__len_0 as isize)
                                > (*__o_0).chunk_limit
                            {
                                _obstack_newchunk(__o_0, __len_0);
                            }
                            memcpy(
                                (*__o_0).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len_0 as u64,
                            );
                            (*__o_0).next_free = ((*__o_0).next_free)
                                .offset(__len_0 as isize);
                            yylval.str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut i8 {
                                    (*__o1).set_maybe_empty_object(1 as i32 as u32);
                                }
                                (*__o1).next_free = (if (::core::mem::size_of::<i64>()
                                    as u64) < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut i8
                                })
                                    .offset(
                                        (((*__o1).next_free)
                                            .offset_from(
                                                (if (::core::mem::size_of::<i64>() as u64)
                                                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut i8
                                                }),
                                            ) as i64 + (*__o1).alignment_mask as i64
                                            & !(*__o1).alignment_mask as i64) as isize,
                                    );
                                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8)
                                    as i64
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut i8) as i64
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut i8;
                            return 257 as i32;
                        }
                        63 | 64 | 65 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            let mut __o_1: *mut obstack = &mut string_stk;
                            let mut __len_1: i32 = yyleng + 1 as i32;
                            if ((*__o_1).next_free).offset(__len_1 as isize)
                                > (*__o_1).chunk_limit
                            {
                                _obstack_newchunk(__o_1, __len_1);
                            }
                            memcpy(
                                (*__o_1).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len_1 as u64,
                            );
                            (*__o_1).next_free = ((*__o_1).next_free)
                                .offset(__len_1 as isize);
                            yylval.str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut i8 {
                                    (*__o1).set_maybe_empty_object(1 as i32 as u32);
                                }
                                (*__o1).next_free = (if (::core::mem::size_of::<i64>()
                                    as u64) < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut i8
                                })
                                    .offset(
                                        (((*__o1).next_free)
                                            .offset_from(
                                                (if (::core::mem::size_of::<i64>() as u64)
                                                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut i8
                                                }),
                                            ) as i64 + (*__o1).alignment_mask as i64
                                            & !(*__o1).alignment_mask as i64) as isize,
                                    );
                                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8)
                                    as i64
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut i8) as i64
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut i8;
                            return 257 as i32;
                        }
                        66 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 2 as i32;
                            break '_yy_match;
                        }
                        67 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        68 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            error_at_line(
                                0 as i32,
                                0 as i32,
                                filename,
                                line_num as u32,
                                b"%s\0" as *const u8 as *const i8,
                                dcgettext(
                                    0 as *const i8,
                                    b"unterminated string?\0" as *const u8 as *const i8,
                                    5 as i32,
                                ),
                            );
                            break '_yy_match;
                        }
                        69 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        70 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        71 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 3 as i32;
                            break '_yy_match;
                        }
                        72 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        73 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        74 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 2 as i32;
                            break '_yy_match;
                        }
                        75 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            yy_start = 1 as i32 + 2 as i32 * 0 as i32;
                            let mut yyless_macro_arg_0: i32 = 0 as i32;
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_bp
                                .offset(yyless_macro_arg_0 as isize)
                                .offset(-(0 as i32 as isize));
                            yy_c_buf_p = yy_cp;
                            yytext = yy_bp;
                            yyleng = yy_cp.offset_from(yy_bp) as i64 as size_t as i32;
                            yy_hold_char = *yy_cp;
                            *yy_cp = '\0' as i32 as i8;
                            yy_c_buf_p = yy_cp;
                            return 271 as i32;
                        }
                        76 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        77 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            break '_yy_match;
                        }
                        78 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 258 as i32;
                        }
                        79 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return 259 as i32;
                        }
                        80 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            return *yytext.offset(0 as i32 as isize) as i32;
                        }
                        81 => {
                            if yyleng > 0 as i32 {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext.offset((yyleng - 1 as i32) as isize)
                                    as i32 == '\n' as i32) as i32;
                            }
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as size_t,
                                1 as i32 as size_t,
                                yyout,
                            );
                            break '_yy_match;
                        }
                        83 | 84 | 85 | 86 | 87 => return 0 as i32,
                        82 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext) as i64
                                as i32 - 1 as i32;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as i32
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh4 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh4 = yyin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as i32;
                            }
                            if yy_c_buf_p
                                <= &mut *((**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut i8
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as i32 as isize);
                                if yy_next_state != 0 {
                                    current_block = 12001150584817703308;
                                    break;
                                } else {
                                    current_block = 11924925386998228636;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as i32;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as i32 as isize);
                                            yy_act = 82 as i32 + (yy_start - 1 as i32) / 2 as i32
                                                + 1 as i32;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                yyrestart(yyin);
                                            }
                                            break '_yy_match;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as i32 as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut i8;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as i32 as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const i8,
                            );
                            break '_yy_match;
                        }
                    }
                }
                match current_block {
                    11924925386998228636 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_get_next_buffer() -> i32 {
    let mut dest: *mut i8 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
        .yy_ch_buf;
    let mut source_0: *mut i8 = yytext;
    let mut number_to_move: i32 = 0;
    let mut i: i32 = 0;
    let mut ret_val: i32 = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as i32) as isize) as *mut i8
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const i8,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as i32
    {
        if yy_c_buf_p.offset_from(yytext) as i64 - 0 as i32 as i64 == 1 as i32 as i64 {
            return 1 as i32
        } else {
            return 2 as i32
        }
    }
    number_to_move = yy_c_buf_p.offset_from(yytext) as i64 as i32 - 1 as i32;
    i = 0 as i32;
    while i < number_to_move {
        let fresh5 = source_0;
        source_0 = source_0.offset(1);
        let fresh6 = dest;
        dest = dest.offset(1);
        *fresh6 = *fresh5;
        i += 1;
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as i32
    {
        yy_n_chars = 0 as i32;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: i32 = ((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size)
            .wrapping_sub(number_to_move as u64)
            .wrapping_sub(1 as i32 as u64) as i32;
        while num_to_read <= 0 as i32 {
            let mut b: YY_BUFFER_STATE = if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            };
            let mut yy_c_buf_p_offset: i32 = yy_c_buf_p.offset_from((*b).yy_ch_buf)
                as i64 as i32;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: i32 = ((*b).yy_buf_size).wrapping_mul(2 as i32 as u64)
                    as i32;
                if new_size <= 0 as i32 {
                    (*b).yy_buf_size = ((*b).yy_buf_size as u64)
                        .wrapping_add(((*b).yy_buf_size).wrapping_div(8 as i32 as u64))
                        as yy_size_t as yy_size_t;
                } else {
                    (*b).yy_buf_size = ((*b).yy_buf_size as u64)
                        .wrapping_mul(2 as i32 as u64) as yy_size_t as yy_size_t;
                }
                (*b).yy_ch_buf = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size).wrapping_add(2 as i32 as u64),
                ) as *mut i8;
            } else {
                (*b).yy_ch_buf = 0 as *mut i8;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const i8,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut i8;
            num_to_read = ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size)
                .wrapping_sub(number_to_move as u64)
                .wrapping_sub(1 as i32 as u64) as i32;
        }
        if num_to_read > 8192 as i32 {
            num_to_read = 8192 as i32;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            let mut c: i32 = '*' as i32;
            let mut n: i32 = 0;
            n = 0 as i32;
            while (n as u64) < num_to_read as size_t
                && {
                    c = _IO_getc(yyin);
                    c != -(1 as i32)
                } && c != '\n' as i32
            {
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut i8)
                    .offset(n as isize) = c as i8;
                n += 1;
                n;
            }
            if c == '\n' as i32 {
                let fresh7 = n;
                n = n + 1;
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut i8)
                    .offset(fresh7 as isize) = c as i8;
            }
            if c == -(1 as i32) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const i8,
                );
            }
            yy_n_chars = n;
        } else {
            *__errno_location() = 0 as i32;
            loop {
                yy_n_chars = fread(
                    &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                        .yy_ch_buf)
                        .offset(number_to_move as isize) as *mut i8 as *mut libc::c_void,
                    1 as i32 as size_t,
                    num_to_read as size_t,
                    yyin,
                ) as i32;
                if !(yy_n_chars == 0 as i32 && ferror(yyin) != 0) {
                    break;
                }
                if *__errno_location() != 4 as i32 {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8 as *const i8,
                    );
                    break;
                } else {
                    *__errno_location() = 0 as i32;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as i32 {
        if number_to_move == 0 as i32 {
            ret_val = 1 as i32;
            yyrestart(yyin);
        } else {
            ret_val = 2 as i32;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status = 2
                as i32;
        }
    } else {
        ret_val = 0 as i32;
    }
    if (yy_n_chars + number_to_move) as yy_size_t
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: yy_size_t = (yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as i32)) as yy_size_t;
        let ref mut fresh8 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh8 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0,
        ) as *mut i8;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const i8,
            );
        }
    }
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as i32 as i8;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset((yy_n_chars + 1 as i32) as isize) = 0 as i32 as i8;
    yytext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as i32 as isize) as *mut i8;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut i8 = 0 as *mut i8;
    yy_current_state = yy_start;
    yy_current_state
        += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
    yy_cp = yytext.offset(0 as i32 as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as i32 != 0 {
            yy_ec[*yy_cp as u8 as u32 as usize]
        } else {
            1 as i32
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32) as usize]
            as i32 != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as i32;
            if yy_current_state >= 191 as i32 {
                yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
            .wrapping_add(yy_c as u32) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: i32 = 0;
    let mut yy_cp: *mut i8 = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as i32 as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as i32 + yy_c as i32) as usize]
        as i32 != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as i32;
        if yy_current_state >= 191 as i32 {
            yy_c = yy_meta[yy_c as u32 as usize] as YY_CHAR;
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as u32)
        .wrapping_add(yy_c as u32) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 190 as i32) as i32;
    return if yy_is_jam != 0 { 0 as i32 } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: i32, mut yy_bp: *mut i8) {
    let mut yy_cp: *mut i8 = 0 as *mut i8;
    yy_cp = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp
        < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset(2 as i32 as isize)
    {
        let mut number_to_move: i32 = yy_n_chars + 2 as i32;
        let mut dest: *mut i8 = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(
                ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size)
                    .wrapping_add(2 as i32 as u64) as isize,
            ) as *mut i8;
        let mut source_0: *mut i8 = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(number_to_move as isize) as *mut i8;
        while source_0
            > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
        {
            source_0 = source_0.offset(-1);
            dest = dest.offset(-1);
            *dest = *source_0;
        }
        yy_cp = yy_cp.offset(dest.offset_from(source_0) as i64 as i32 as isize);
        yy_bp = yy_bp.offset(dest.offset_from(source_0) as i64 as i32 as isize);
        yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
            as i32;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
        if yy_cp
            < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(2 as i32 as isize)
        {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const i8,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as i8;
    yytext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
unsafe extern "C" fn input() -> i32 {
    let mut c: i32 = 0;
    *yy_c_buf_p = yy_hold_char;
    if *yy_c_buf_p as i32 == 0 as i32 {
        if yy_c_buf_p
            < &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(yy_n_chars as isize) as *mut i8
        {
            *yy_c_buf_p = '\0' as i32 as i8;
        } else {
            let mut offset: i32 = yy_c_buf_p.offset_from(yytext) as i64 as i32;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 1030730166680676460;
                }
                1 => {
                    current_block_10 = 1030730166680676460;
                }
                0 => {
                    yy_c_buf_p = yytext.offset(offset as isize);
                    current_block_10 = 7746791466490516765;
                }
                _ => {
                    current_block_10 = 7746791466490516765;
                }
            }
            match current_block_10 {
                7746791466490516765 => {}
                _ => {
                    if yywrap() != 0 {
                        return -(1 as i32);
                    }
                    if yy_did_buffer_switch_on_eof == 0 {
                        yyrestart(yyin);
                    }
                    return input();
                }
            }
        }
    }
    c = *(yy_c_buf_p as *mut u8) as i32;
    *yy_c_buf_p = '\0' as i32 as i8;
    yy_c_buf_p = yy_c_buf_p.offset(1);
    yy_hold_char = *yy_c_buf_p;
    (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol = (c
        == '\n' as i32) as i32;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yyensure_buffer_stack();
        let ref mut fresh9 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh9 = yy_create_buffer(yyin, 16384 as i32);
    }
    yy_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    yy_load_buffer_state();
}
#[no_mangle]
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    yyensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }) == new_buffer
    {
        return;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh10 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh10 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh11 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh11 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as i32;
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
#[no_mangle]
pub unsafe extern "C" fn yy_create_buffer(
    mut file: *mut FILE,
    mut size: i32,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yyalloc(::core::mem::size_of::<yy_buffer_state>() as u64) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_buf_size = size as yy_size_t;
    (*b).yy_ch_buf = yyalloc(((*b).yy_buf_size).wrapping_add(2 as i32 as u64))
        as *mut i8;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_is_our_buffer = 1 as i32;
    yy_init_buffer(b, file);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh12 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
}
unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: i32 = *__errno_location();
    yy_flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as i32;
    if b
        != (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        (*b).yy_bs_lineno = 1 as i32;
        (*b).yy_bs_column = 0 as i32;
    }
    (*b).yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as i32) as i32
    } else {
        0 as i32
    };
    *__errno_location() = oerrno;
}
#[no_mangle]
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as i32;
    *((*b).yy_ch_buf).offset(0 as i32 as isize) = 0 as i32 as i8;
    *((*b).yy_ch_buf).offset(1 as i32 as isize) = 0 as i32 as i8;
    (*b).yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as i32 as isize) as *mut i8;
    (*b).yy_at_bol = 1 as i32;
    (*b).yy_buffer_status = 0 as i32;
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        yy_load_buffer_state();
    }
}
#[no_mangle]
pub unsafe extern "C" fn yypush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    yyensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh13 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh13 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
        yy_buffer_stack_top;
    }
    let ref mut fresh14 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh14 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yypop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh15 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh15 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as i32 as u64 {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
        yy_buffer_stack_top;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as i32;
    }
}
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: i32 = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as i32;
        yy_buffer_stack = yyalloc(
            (num_to_alloc as u64)
                .wrapping_mul(::core::mem::size_of::<*mut yy_buffer_state>() as u64),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const i8,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as i32,
            (num_to_alloc as u64)
                .wrapping_mul(::core::mem::size_of::<*mut yy_buffer_state>() as u64),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
        yy_buffer_stack_top = 0 as i32 as size_t;
        return;
    }
    if yy_buffer_stack_top >= yy_buffer_stack_max.wrapping_sub(1 as i32 as u64) {
        let mut grow_size: i32 = 8 as i32;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size as u64) as i32;
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            (num_to_alloc as u64)
                .wrapping_mul(::core::mem::size_of::<*mut yy_buffer_state>() as u64),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const i8,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as i32,
            (grow_size as u64)
                .wrapping_mul(::core::mem::size_of::<*mut yy_buffer_state>() as u64),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut i8,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as i32 as u64
        || *base.offset(size.wrapping_sub(2 as i32 as u64) as isize) as i32 != 0 as i32
        || *base.offset(size.wrapping_sub(1 as i32 as u64) as isize) as i32 != 0 as i32
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yyalloc(::core::mem::size_of::<yy_buffer_state>() as u64) as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8 as *const i8,
        );
    }
    (*b).yy_buf_size = size.wrapping_sub(2 as i32 as u64);
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as i32;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size as i32;
    (*b).yy_is_interactive = 0 as i32;
    (*b).yy_at_bol = 1 as i32;
    (*b).yy_fill_buffer = 0 as i32;
    (*b).yy_buffer_status = 0 as i32;
    yy_switch_to_buffer(b);
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_string(mut yystr: *const i8) -> YY_BUFFER_STATE {
    return yy_scan_bytes(yystr, strlen(yystr) as i32);
}
#[no_mangle]
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const i8,
    mut _yybytes_len: i32,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut n: yy_size_t = 0;
    let mut i: i32 = 0;
    n = (_yybytes_len + 2 as i32) as yy_size_t;
    buf = yyalloc(n) as *mut i8;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8 as *const i8,
        );
    }
    i = 0 as i32;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh16 = *buf.offset((_yybytes_len + 1 as i32) as isize);
    *fresh16 = 0 as i32 as i8;
    *buf.offset(_yybytes_len as isize) = *fresh16;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const i8);
    }
    (*b).yy_is_our_buffer = 1 as i32;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const i8) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, msg);
    exit(2 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn yyget_lineno() -> i32 {
    return yylineno;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_leng() -> i32 {
    return yyleng;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_text() -> *mut i8 {
    return yytext;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_lineno(mut line_number: i32) {
    yylineno = line_number;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_in(mut in_str: *mut FILE) {
    yyin = in_str;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_out(mut out_str: *mut FILE) {
    yyout = out_str;
}
#[no_mangle]
pub unsafe extern "C" fn yyget_debug() -> i32 {
    return yy_flex_debug;
}
#[no_mangle]
pub unsafe extern "C" fn yyset_debug(mut bdebug: i32) {
    yy_flex_debug = bdebug;
}
unsafe extern "C" fn yy_init_globals() -> i32 {
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as i32 as size_t;
    yy_buffer_stack_max = 0 as i32 as size_t;
    yy_c_buf_p = 0 as *mut i8;
    yy_init = 0 as i32;
    yy_start = 0 as i32;
    yyin = 0 as *mut FILE;
    yyout = 0 as *mut FILE;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yylex_destroy() -> i32 {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh17 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh17 = 0 as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr as *mut i8 as *mut libc::c_void, size);
}
#[no_mangle]
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut i8 as *mut libc::c_void);
}