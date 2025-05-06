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
    pub type table_entry;
    fn free(__ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn printf(_: *const i8, _: ...) -> i32;
    fn _obstack_newchunk(_: *mut obstack, _: i32);
    fn _obstack_begin(
        _: *mut obstack,
        _: i32,
        _: i32,
        _: Option<unsafe extern "C" fn(i64) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
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
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut verbose: i32;
    static mut use_indentation: i32;
    static mut strict_ansi: i32;
    static mut debug: i32;
    static mut omit_arguments_option: i32;
    static mut omit_symbol_names_option: i32;
    fn lookup(_: *const i8) -> *mut Symbol;
    fn install(_: *mut i8, _: i32) -> *mut Symbol;
    fn install_ident(name: *mut i8, storage: storage) -> *mut Symbol;
    fn ident_change_storage(sp: *mut Symbol, storage: storage);
    fn delete_autos(level_0: i32);
    fn delete_parms(level_0: i32);
    fn move_parms(level_0: i32);
    fn linked_list_create(fun: linked_list_free_data_fp) -> *mut linked_list;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    fn data_in_list(data: *mut libc::c_void, list: *mut linked_list) -> i32;
    fn get_token() -> i32;
    fn globals_only() -> i32;
    static mut yylval: YYSTYPE;
    static mut filename: *mut i8;
    static mut line_num: i32;
}
pub type size_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub line: i32,
    pub source: *mut i8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOKSTK {
    pub type_0: i32,
    pub token: *mut i8,
    pub line: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ident {
    pub name: *mut i8,
    pub type_end: i32,
    pub parmcnt: i32,
    pub line: i32,
    pub storage: storage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTYPE {
    pub str_0: *mut i8,
}
pub type Stackpos = [i32; 1];
#[no_mangle]
pub static mut level: i32 = 0;
#[no_mangle]
pub static mut caller: *mut Symbol = 0 as *const Symbol as *mut Symbol;
#[no_mangle]
pub static mut text_stk: obstack = obstack {
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
pub static mut parm_level: i32 = 0;
#[no_mangle]
pub static mut tok: TOKSTK = TOKSTK {
    type_0: 0,
    token: 0 as *const i8 as *mut i8,
    line: 0,
};
#[no_mangle]
pub static mut token_stack: *mut TOKSTK = 0 as *const TOKSTK as *mut TOKSTK;
#[no_mangle]
pub static mut tos: i32 = 0;
#[no_mangle]
pub static mut curs: i32 = 0;
#[no_mangle]
pub static mut token_stack_length: i32 = 64 as i32;
#[no_mangle]
pub static mut token_stack_increase: i32 = 32 as i32;
static mut need_space: i32 = 0;
unsafe extern "C" fn print_token(mut tokptr: *mut TOKSTK) {
    match (*tokptr).type_0 {
        260 | 270 | 257 | 265 | 264 | 272 | 273 | 266 => {
            fprintf(stderr, b"`%s'\0" as *const u8 as *const i8, (*tokptr).token);
        }
        258 | 123 => {
            fprintf(stderr, b"`{'\0" as *const u8 as *const i8);
        }
        259 | 125 => {
            fprintf(stderr, b"`}'\0" as *const u8 as *const i8);
        }
        261 => {
            fprintf(stderr, b"`extern'\0" as *const u8 as *const i8);
        }
        262 => {
            fprintf(stderr, b"`static'\0" as *const u8 as *const i8);
        }
        263 => {
            fprintf(stderr, b"`typedef'\0" as *const u8 as *const i8);
        }
        271 => {
            fprintf(stderr, b"\"%s\"\0" as *const u8 as *const i8, (*tokptr).token);
        }
        _ => {
            fprintf(stderr, b"`%c'\0" as *const u8 as *const i8, (*tokptr).type_0);
        }
    };
}
unsafe extern "C" fn file_error(mut msg: *mut i8, mut tokptr: *mut TOKSTK) {
    fprintf(stderr, b"%s:%d: %s\0" as *const u8 as *const i8, filename, tok.line, msg);
    if !tokptr.is_null() {
        fprintf(
            stderr,
            dcgettext(0 as *const i8, b" near \0" as *const u8 as *const i8, 5 as i32),
        );
        print_token(tokptr);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn mark(mut pos: *mut i32) {
    *pos.offset(0 as i32 as isize) = curs;
}
#[no_mangle]
pub unsafe extern "C" fn restore(mut pos: *mut i32) {
    curs = *pos.offset(0 as i32 as isize);
    if curs != 0 {
        tok = *token_stack.offset((curs - 1 as i32) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tokpush(mut type_0: i32, mut line: i32, mut token: *mut i8) {
    (*token_stack.offset(tos as isize)).type_0 = type_0;
    let ref mut fresh0 = (*token_stack.offset(tos as isize)).token;
    *fresh0 = token;
    (*token_stack.offset(tos as isize)).line = line;
    tos += 1;
    if tos == token_stack_length {
        token_stack_length += token_stack_increase;
        token_stack = xrealloc(
            token_stack as *mut libc::c_void,
            (token_stack_length as u64)
                .wrapping_mul(::core::mem::size_of::<TOKSTK>() as u64),
        ) as *mut TOKSTK;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_stack() {
    let mut delta: i32 = tos - curs;
    if delta != 0 {
        memmove(
            token_stack as *mut libc::c_void,
            token_stack.offset(curs as isize) as *const libc::c_void,
            (delta as u64).wrapping_mul(::core::mem::size_of::<TOKSTK>() as u64),
        );
    }
    tos = delta;
    curs = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn clearstack() {
    curs = 0 as i32;
    tos = curs;
}
#[no_mangle]
pub unsafe extern "C" fn nexttoken() -> i32 {
    let mut type_0: i32 = 0;
    if curs == tos {
        type_0 = get_token();
        tokpush(type_0, line_num, yylval.str_0);
    }
    tok = *token_stack.offset(curs as isize);
    curs += 1;
    curs;
    return tok.type_0;
}
#[no_mangle]
pub unsafe extern "C" fn putback() -> i32 {
    if curs == 0 as i32 {
        error(
            10 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"INTERNAL ERROR: cannot return token to stream\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    curs -= 1;
    curs;
    if curs > 0 as i32 {
        tok.type_0 = (*token_stack.offset((curs - 1 as i32) as isize)).type_0;
        tok.token = (*token_stack.offset((curs - 1 as i32) as isize)).token;
    } else {
        tok.type_0 = 0 as i32;
    }
    return tok.type_0;
}
#[no_mangle]
pub unsafe extern "C" fn init_parse() {
    _obstack_begin(
        &mut text_stk,
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
    token_stack = xmalloc(
        (token_stack_length as u64).wrapping_mul(::core::mem::size_of::<TOKSTK>() as u64),
    ) as *mut TOKSTK;
    clearstack();
}
#[no_mangle]
pub unsafe extern "C" fn save_token(mut tokptr: *mut TOKSTK) {
    let mut len: i32 = 0;
    match (*tokptr).type_0 {
        260 | 270 | 264 | 272 | 257 | 273 => {
            if need_space != 0 {
                let mut __o: *mut obstack = &mut text_stk;
                if ((*__o).next_free).offset(1 as i32 as isize) > (*__o).chunk_limit {
                    _obstack_newchunk(__o, 1 as i32);
                }
                let fresh1 = (*__o).next_free;
                (*__o).next_free = ((*__o).next_free).offset(1);
                *fresh1 = ' ' as i32 as i8;
            }
            len = strlen((*tokptr).token) as i32;
            let mut __o_0: *mut obstack = &mut text_stk;
            let mut __len: i32 = len;
            if ((*__o_0).next_free).offset(__len as isize) > (*__o_0).chunk_limit {
                _obstack_newchunk(__o_0, __len);
            }
            memcpy(
                (*__o_0).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len as u64,
            );
            (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
            need_space = 1 as i32;
        }
        265 => {
            if need_space != 0 {
                let mut __o_1: *mut obstack = &mut text_stk;
                if ((*__o_1).next_free).offset(1 as i32 as isize) > (*__o_1).chunk_limit
                {
                    _obstack_newchunk(__o_1, 1 as i32);
                }
                let fresh2 = (*__o_1).next_free;
                (*__o_1).next_free = ((*__o_1).next_free).offset(1);
                *fresh2 = ' ' as i32 as i8;
            }
            if *((*tokptr).token).offset(0 as i32 as isize) as i32 == '*' as i32 {
                need_space = 0 as i32;
            } else {
                need_space = 1 as i32;
            }
            len = strlen((*tokptr).token) as i32;
            let mut __o_2: *mut obstack = &mut text_stk;
            let mut __len_0: i32 = len;
            if ((*__o_2).next_free).offset(__len_0 as isize) > (*__o_2).chunk_limit {
                _obstack_newchunk(__o_2, __len_0);
            }
            memcpy(
                (*__o_2).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len_0 as u64,
            );
            (*__o_2).next_free = ((*__o_2).next_free).offset(__len_0 as isize);
        }
        261 | 262 => {}
        44 => {
            let mut __o_3: *mut obstack = &mut text_stk;
            if ((*__o_3).next_free).offset(1 as i32 as isize) > (*__o_3).chunk_limit {
                _obstack_newchunk(__o_3, 1 as i32);
            }
            let fresh3 = (*__o_3).next_free;
            (*__o_3).next_free = ((*__o_3).next_free).offset(1);
            *fresh3 = ',' as i32 as i8;
            need_space = 1 as i32;
        }
        40 => {
            if need_space != 0 {
                let mut __o_4: *mut obstack = &mut text_stk;
                if ((*__o_4).next_free).offset(1 as i32 as isize) > (*__o_4).chunk_limit
                {
                    _obstack_newchunk(__o_4, 1 as i32);
                }
                let fresh4 = (*__o_4).next_free;
                (*__o_4).next_free = ((*__o_4).next_free).offset(1);
                *fresh4 = ' ' as i32 as i8;
            }
            let mut __o_5: *mut obstack = &mut text_stk;
            if ((*__o_5).next_free).offset(1 as i32 as isize) > (*__o_5).chunk_limit {
                _obstack_newchunk(__o_5, 1 as i32);
            }
            let fresh5 = (*__o_5).next_free;
            (*__o_5).next_free = ((*__o_5).next_free).offset(1);
            *fresh5 = (*tokptr).type_0 as i8;
            need_space = 0 as i32;
        }
        41 => {
            let mut __o_6: *mut obstack = &mut text_stk;
            if ((*__o_6).next_free).offset(1 as i32 as isize) > (*__o_6).chunk_limit {
                _obstack_newchunk(__o_6, 1 as i32);
            }
            let fresh6 = (*__o_6).next_free;
            (*__o_6).next_free = ((*__o_6).next_free).offset(1);
            *fresh6 = (*tokptr).type_0 as i8;
            need_space = 1 as i32;
        }
        91 | 93 => {
            let mut __o_7: *mut obstack = &mut text_stk;
            if ((*__o_7).next_free).offset(1 as i32 as isize) > (*__o_7).chunk_limit {
                _obstack_newchunk(__o_7, 1 as i32);
            }
            let fresh7 = (*__o_7).next_free;
            (*__o_7).next_free = ((*__o_7).next_free).offset(1);
            *fresh7 = (*tokptr).type_0 as i8;
            need_space = 0 as i32;
        }
        266 => {
            let mut __o_8: *mut obstack = &mut text_stk;
            if ((*__o_8).next_free).offset(1 as i32 as isize) > (*__o_8).chunk_limit {
                _obstack_newchunk(__o_8, 1 as i32);
            }
            let fresh8 = (*__o_8).next_free;
            (*__o_8).next_free = ((*__o_8).next_free).offset(1);
            *fresh8 = ' ' as i32 as i8;
            let mut __o_9: *mut obstack = &mut text_stk;
            let mut __len_1: i32 = strlen((*tokptr).token) as i32;
            if ((*__o_9).next_free).offset(__len_1 as isize) > (*__o_9).chunk_limit {
                _obstack_newchunk(__o_9, __len_1);
            }
            memcpy(
                (*__o_9).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len_1 as u64,
            );
            (*__o_9).next_free = ((*__o_9).next_free).offset(__len_1 as isize);
            need_space = 1 as i32;
        }
        _ => {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"unrecognized definition\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    tokptr,
                );
            }
        }
    };
}
static mut start_pos: Stackpos = [0; 1];
static mut save_end: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn save_stack() {
    mark(start_pos.as_mut_ptr());
    save_end = curs - 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn undo_save_stack() {
    save_end = -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn finish_save_stack(mut name: *mut i8) -> *mut i8 {
    let mut i: i32 = 0;
    let mut level_0: i32 = 0 as i32;
    let mut found_ident: i32 = (omit_symbol_names_option == 0) as i32;
    need_space = 0 as i32;
    let mut current_block_13: u64;
    i = 0 as i32;
    while i < save_end {
        match (*token_stack.offset(i as isize)).type_0 {
            40 => {
                if omit_arguments_option != 0 {
                    if level_0 == 0 as i32 {
                        save_token(token_stack.offset(i as isize));
                    }
                    level_0 += 1;
                    level_0;
                }
                current_block_13 = 13056961889198038528;
            }
            41 => {
                if omit_arguments_option != 0 {
                    level_0 -= 1;
                    level_0;
                }
                current_block_13 = 13056961889198038528;
            }
            260 => {
                if found_ident == 0
                    && strcmp(name, (*token_stack.offset(i as isize)).token) == 0 as i32
                {
                    need_space = 1 as i32;
                    found_ident = 1 as i32;
                    current_block_13 = 4988723283678924448;
                } else {
                    current_block_13 = 13056961889198038528;
                }
            }
            _ => {
                current_block_13 = 13056961889198038528;
            }
        }
        match current_block_13 {
            13056961889198038528 => {
                if level_0 == 0 as i32 {
                    save_token(token_stack.offset(i as isize));
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    let mut __o: *mut obstack = &mut text_stk;
    if ((*__o).next_free).offset(1 as i32 as isize) > (*__o).chunk_limit {
        _obstack_newchunk(__o, 1 as i32);
    }
    let fresh9 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh9 = 0 as i32 as i8;
    return ({
        let mut __o1: *mut obstack = &mut text_stk as *mut obstack;
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
}
#[no_mangle]
pub unsafe extern "C" fn skip_to(mut c: i32) {
    while nexttoken() != 0 {
        if tok.type_0 == c {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn skip_balanced(
    mut open_tok: i32,
    mut close_tok: i32,
    mut level_0: i32,
) -> i32 {
    if level_0 == 0 as i32 {
        if nexttoken() != open_tok {
            return 1 as i32;
        }
        level_0 += 1;
        level_0;
    }
    while nexttoken() != 0 {
        if tok.type_0 == 258 as i32 && open_tok == '{' as i32 {
            tok.type_0 = '{' as i32;
        } else if tok.type_0 == 259 as i32 && close_tok == '}' as i32 {
            tok.type_0 = '}' as i32;
        }
        if tok.type_0 == open_tok {
            level_0 += 1;
            level_0;
        } else if tok.type_0 == close_tok {
            level_0 -= 1;
            if level_0 == 0 as i32 {
                nexttoken();
                return 0 as i32;
            }
        }
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> i32 {
    let mut identifier: Ident = Ident {
        name: 0 as *mut i8,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: storage::ExternStorage,
    };
    level = 0 as i32;
    caller = 0 as *mut Symbol;
    clearstack();
    let mut current_block_11: u64;
    while nexttoken() != 0 {
        identifier.storage = storage::ExternStorage;
        match tok.type_0 {
            0 => return 0 as i32,
            273 => {
                continue;
            }
            263 => {
                parse_typedef();
                current_block_11 = 5399440093318478209;
            }
            261 => {
                identifier.storage = storage::ExplicitExternStorage;
                parse_declaration(&mut identifier, 0 as i32);
                current_block_11 = 5399440093318478209;
            }
            262 => {
                identifier.storage = storage::StaticStorage;
                nexttoken();
                current_block_11 = 740180670140435916;
            }
            _ => {
                current_block_11 = 740180670140435916;
            }
        }
        match current_block_11 {
            740180670140435916 => {
                parse_declaration(&mut identifier, 0 as i32);
            }
            _ => {}
        }
        cleanup_stack();
    }
    return 0 as i32;
}
unsafe extern "C" fn is_function() -> i32 {
    let mut sp: Stackpos = [0; 1];
    let mut res: i32 = 0 as i32;
    mark(sp.as_mut_ptr());
    loop {
        match tok.type_0 {
            273 | 270 | 260 | 265 | 262 | 261 => {
                nexttoken();
            }
            272 => {
                if skip_balanced('(' as i32, ')' as i32, 0 as i32) == -(1 as i32) {
                    file_error(
                        dcgettext(
                            0 as *const i8,
                            b"unexpected end of file in declaration\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
            }
            40 => {
                res = (nexttoken() != 265 as i32) as i32;
                break;
            }
            _ => {
                break;
            }
        }
    }
    restore(sp.as_mut_ptr());
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn parse_declaration(mut ident: *mut Ident, mut parm: i32) {
    if is_function() != 0 {
        parse_function_declaration(ident, parm);
    } else {
        parse_variable_declaration(ident, parm);
    }
    delete_parms(parm_level);
}
#[no_mangle]
pub unsafe extern "C" fn expression() {
    let mut name: *mut i8 = 0 as *mut i8;
    let mut line: i32 = 0;
    let mut parens_lev: i32 = 0;
    parens_lev = 0 as i32;
    loop {
        match tok.type_0 {
            59 => return,
            123 | 258 | 125 | 259 => {
                putback();
                return;
            }
            44 => {
                if parens_lev == 0 as i32 {
                    return;
                }
            }
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const i8,
                            b"unexpected end of file in expression\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                return;
            }
            260 => {
                name = tok.token;
                line = tok.line;
                nexttoken();
                if tok.type_0 == '(' as i32 {
                    call(name, line);
                    parens_lev += 1;
                    parens_lev;
                } else {
                    reference(name, line);
                    if tok.type_0 == 269 as i32 {
                        while tok.type_0 == 269 as i32 {
                            nexttoken();
                        }
                    } else {
                        putback();
                    }
                }
            }
            40 => {
                if nexttoken() == 270 as i32 {
                    skip_to(')' as i32);
                } else {
                    putback();
                    parens_lev += 1;
                    parens_lev;
                }
            }
            41 => {
                parens_lev -= 1;
                parens_lev;
            }
            _ => {}
        }
        nexttoken();
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_function_declaration(
    mut ident: *mut Ident,
    mut parm: i32,
) {
    let mut error_recovery: i32 = 0 as i32;
    (*ident).type_end = -(1 as i32);
    parse_knr_dcl(ident);
    loop {
        match tok.type_0 {
            41 => {
                if parm != 0 {
                    break;
                }
            }
            59 | 44 => {
                break;
            }
            258 | 123 => {
                if !((*ident).name).is_null() {
                    caller = lookup((*ident).name);
                    func_body();
                }
                break;
            }
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const i8,
                            b"unexpected end of file in declaration\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                break;
            }
            _ => {}
        }
        if error_recovery != 0 {
            nexttoken();
        } else {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"expected `;'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    &mut tok,
                );
            }
            error_recovery = 1 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn fake_struct(mut ident: *mut Ident) -> i32 {
    let mut sp: Stackpos = [0; 1];
    mark(sp.as_mut_ptr());
    (*ident).type_end = -(1 as i32);
    if tok.type_0 == 264 as i32 {
        if nexttoken() == 260 as i32 {
            (*ident).type_end = tos;
        }
        putback();
        skip_struct();
        if tok.type_0 == 260 as i32 || tok.type_0 == 265 as i32 {
            let mut hold: TOKSTK = tok;
            restore(sp.as_mut_ptr());
            if (*ident).type_end == -(1 as i32) {
                tos = curs;
                (*token_stack.offset(curs as isize)).type_0 = 260 as i32;
                let ref mut fresh10 = (*token_stack.offset(curs as isize)).token;
                *fresh10 = b"{ ... }\0" as *const u8 as *const i8 as *mut i8;
                tos += 1;
                tos;
            } else {
                tos = curs + 1 as i32;
            }
            tokpush(hold.type_0, hold.line, hold.token);
        } else if tok.type_0 == '(' as i32 {
            return 0 as i32
        } else if tok.type_0 != ';' as i32 {
            file_error(
                dcgettext(
                    0 as *const i8,
                    b"missing `;' after struct declaration\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                &mut tok,
            );
        }
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn parse_variable_declaration(
    mut ident: *mut Ident,
    mut parm: i32,
) {
    let mut current_block: u64;
    let mut sp: Stackpos = [0; 1];
    mark(sp.as_mut_ptr());
    (*ident).type_end = -(1 as i32);
    if tok.type_0 == 264 as i32 {
        if nexttoken() == 260 as i32 {
            (*ident).type_end = tos;
        }
        putback();
        skip_struct();
        while tok.type_0 == 265 as i32 || tok.type_0 == 273 as i32 {
            nexttoken();
        }
        if tok.type_0 == 260 as i32 {
            let mut hold: TOKSTK = tok;
            restore(sp.as_mut_ptr());
            if (*ident).type_end == -(1 as i32) {
                tos = curs;
                (*token_stack.offset(curs as isize)).type_0 = 260 as i32;
                let ref mut fresh11 = (*token_stack.offset(curs as isize)).token;
                *fresh11 = b"{ ... }\0" as *const u8 as *const i8 as *mut i8;
                tos += 1;
                tos;
            } else {
                tos = curs + 1 as i32;
            }
            tokpush(hold.type_0, hold.line, hold.token);
        } else {
            if tok.type_0 == ';' as i32 {
                return;
            }
            restore(sp.as_mut_ptr());
        }
    }
    '_again: loop {
        parse_dcl(ident, 0 as i32);
        loop {
            match tok.type_0 {
                41 => {
                    if parm != 0 {
                        current_block = 3123434771885419771;
                        break '_again;
                    } else {
                        current_block = 14180127615403059768;
                        break '_again;
                    }
                }
                59 => {
                    current_block = 3123434771885419771;
                    break '_again;
                }
                44 => {
                    if parm != 0 {
                        current_block = 3123434771885419771;
                        break '_again;
                    }
                    tos = (*ident).type_end;
                    restore(sp.as_mut_ptr());
                    break;
                }
                61 => {
                    nexttoken();
                    if tok.type_0 == '{' as i32 || tok.type_0 == 258 as i32 {
                        initializer_list();
                    } else {
                        expression();
                    }
                }
                258 | 123 => {
                    func_body();
                    current_block = 3123434771885419771;
                    break '_again;
                }
                0 => {
                    if verbose != 0 {
                        file_error(
                            dcgettext(
                                0 as *const i8,
                                b"unexpected end of file in declaration\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            0 as *mut TOKSTK,
                        );
                    }
                    current_block = 3123434771885419771;
                    break '_again;
                }
                _ => {
                    current_block = 14180127615403059768;
                    break '_again;
                }
            }
        }
    }
    match current_block {
        14180127615403059768 => {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"expected `;'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    &mut tok,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn initializer_list() {
    let mut lev: i32 = 0 as i32;
    loop {
        match tok.type_0 {
            123 | 258 => {
                lev += 1;
                lev;
            }
            125 | 259 => {
                lev -= 1;
                if lev <= 0 as i32 {
                    nexttoken();
                    return;
                }
            }
            0 => {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"unexpected end of file in initializer list\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    0 as *mut TOKSTK,
                );
                return;
            }
            44 => {}
            _ => {
                expression();
            }
        }
        nexttoken();
    };
}
#[no_mangle]
pub unsafe extern "C" fn parse_knr_dcl(mut ident: *mut Ident) {
    (*ident).type_end = -(1 as i32);
    parse_dcl(ident, (strict_ansi == 0) as i32);
}
#[no_mangle]
pub unsafe extern "C" fn skip_struct() {
    if nexttoken() == 260 as i32 {
        nexttoken();
    } else if tok.type_0 == ';' as i32 {
        return
    }
    if tok.type_0 == '{' as i32 || tok.type_0 == 258 as i32 {
        if skip_balanced('{' as i32, '}' as i32, 1 as i32) == -(1 as i32) {
            file_error(
                dcgettext(
                    0 as *const i8,
                    b"unexpected end of file in struct\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                0 as *mut TOKSTK,
            );
            return;
        }
    }
    while tok.type_0 == 272 as i32 {
        if skip_balanced('(' as i32, ')' as i32, 0 as i32) == -(1 as i32) {
            file_error(
                dcgettext(
                    0 as *const i8,
                    b"unexpected end of file in struct\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                0 as *mut TOKSTK,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_typedef() {
    let mut ident: Ident = Ident {
        name: 0 as *mut i8,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: storage::ExternStorage,
    };
    ident.name = 0 as *mut i8;
    ident.type_end = -(1 as i32);
    ident.parmcnt = -(1 as i32);
    ident.line = -(1 as i32);
    ident.storage = storage::AnyStorage;
    nexttoken();
    if fake_struct(&mut ident) == 0 {
        putback();
    }
    dcl(&mut ident);
    if !(ident.name).is_null() {
        declare_type(&mut ident);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_dcl(mut ident: *mut Ident, mut maybe_knr: i32) {
    (*ident).parmcnt = -(1 as i32);
    (*ident).name = 0 as *mut i8;
    putback();
    dcl(ident);
    save_stack();
    if !((*ident).name).is_null() {
        declare(ident, maybe_knr);
    } else {
        undo_save_stack();
    };
}
#[no_mangle]
pub unsafe extern "C" fn dcl(mut idptr: *mut Ident) -> i32 {
    while nexttoken() != 0 as i32 && tok.type_0 != '(' as i32 {
        if tok.type_0 == 265 as i32 {
            if !idptr.is_null() && (*idptr).type_end == -(1 as i32) {
                (*idptr).type_end = curs - 1 as i32;
            }
        } else if tok.type_0 == 272 as i32 {
            if skip_balanced('(' as i32, ')' as i32, 0 as i32) == -(1 as i32) {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"unexpected end of file in function declaration\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    0 as *mut TOKSTK,
                );
                return 1 as i32;
            }
        } else if tok.type_0 == 260 as i32 {
            let mut type_0: i32 = 0;
            while tok.type_0 == 260 as i32 {
                nexttoken();
            }
            type_0 = tok.type_0;
            putback();
            if !(type_0 == 270 as i32 || type_0 == 265 as i32 || type_0 == 273 as i32) {
                break;
            }
        } else if tok.type_0 == ')' as i32 || tok.type_0 == ';' as i32 {
            return 1 as i32
        }
    }
    if !idptr.is_null() && (*idptr).type_end == -(1 as i32) {
        (*idptr).type_end = curs - 1 as i32;
    }
    return dirdcl(idptr);
}
#[no_mangle]
pub unsafe extern "C" fn dirdcl(mut idptr: *mut Ident) -> i32 {
    let mut wrapper: i32 = 0 as i32;
    let mut parm_ptr: *mut i32 = 0 as *mut i32;
    if tok.type_0 == '(' as i32 {
        dcl(idptr);
        if tok.type_0 != ')' as i32 && verbose != 0 {
            file_error(
                dcgettext(
                    0 as *const i8,
                    b"expected `)'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                &mut tok,
            );
            return 1 as i32;
        }
    } else if tok.type_0 == 260 as i32 {
        if !idptr.is_null() {
            (*idptr).name = tok.token;
            (*idptr).line = tok.line;
            parm_ptr = &mut (*idptr).parmcnt;
        }
    }
    if nexttoken() == 272 as i32 {
        wrapper = 1 as i32;
        nexttoken();
    } else {
        putback();
    }
    while nexttoken() == '[' as i32 || tok.type_0 == '(' as i32 {
        if tok.type_0 == '[' as i32 {
            skip_to(']' as i32);
        } else {
            maybe_parm_list(parm_ptr);
            if tok.type_0 != ')' as i32 && verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const i8,
                        b"expected `)'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    &mut tok,
                );
                return 1 as i32;
            }
        }
    }
    if wrapper != 0 {
        nexttoken();
    }
    while tok.type_0 == 272 as i32 {
        if skip_balanced('(' as i32, ')' as i32, 0 as i32) == -(1 as i32) {
            file_error(
                dcgettext(
                    0 as *const i8,
                    b"unexpected end of file in function declaration\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                0 as *mut TOKSTK,
            );
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn parmdcl(mut idptr: *mut Ident) -> i32 {
    let mut type_0: i32 = 0;
    while nexttoken() != 0 as i32 && tok.type_0 != '(' as i32 {
        if tok.type_0 == 265 as i32 {
            if !idptr.is_null() && (*idptr).type_end == -(1 as i32) {
                (*idptr).type_end = curs - 1 as i32;
            }
        } else if tok.type_0 == 260 as i32 {
            while tok.type_0 == 260 as i32 {
                nexttoken();
            }
            type_0 = tok.type_0;
            putback();
            if type_0 != 265 as i32 {
                break;
            }
        } else if tok.type_0 == ')' as i32 || tok.type_0 == ',' as i32 {
            return 0 as i32
        }
    }
    if !idptr.is_null() && (*idptr).type_end == -(1 as i32) {
        (*idptr).type_end = curs - 1 as i32;
    }
    return dirdcl(idptr);
}
#[no_mangle]
pub unsafe extern "C" fn maybe_parm_list(mut parm_cnt_return: *mut i32) {
    let mut parmcnt: i32 = 0 as i32;
    let mut ident: Ident = Ident {
        name: 0 as *mut i8,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: storage::ExternStorage,
    };
    let mut level_0: i32 = 0;
    parm_level += 1;
    parm_level;
    while nexttoken() != 0 {
        match tok.type_0 {
            41 => {
                if !parm_cnt_return.is_null() {
                    *parm_cnt_return = parmcnt;
                }
                parm_level -= 1;
                parm_level;
                return;
            }
            44 => {}
            273 | 260 | 265 | 264 | 267 | 270 => {
                parmcnt += 1;
                parmcnt;
                ident.storage = storage::AutoStorage;
                parse_declaration(&mut ident, 1 as i32);
                putback();
            }
            _ => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const i8,
                            b"unexpected token in parameter list\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        &mut tok,
                    );
                }
                level_0 = 0 as i32;
                loop {
                    if tok.type_0 == '(' as i32 {
                        level_0 += 1;
                        level_0;
                    } else if tok.type_0 == ')' as i32 {
                        let fresh12 = level_0;
                        level_0 = level_0 - 1;
                        if fresh12 == 0 as i32 {
                            break;
                        }
                    }
                    if !(nexttoken() != 0) {
                        break;
                    }
                }
                putback();
            }
        }
    }
    if verbose != 0 {
        file_error(
            dcgettext(
                0 as *const i8,
                b"unexpected end of file in parameter list\0" as *const u8 as *const i8,
                5 as i32,
            ),
            0 as *mut TOKSTK,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn func_body() {
    let mut ident: Ident = Ident {
        name: 0 as *mut i8,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: storage::ExternStorage,
    };
    level += 1;
    level;
    move_parms(level);
    while level != 0 {
        cleanup_stack();
        nexttoken();
        let mut current_block_23: u64;
        match tok.type_0 {
            262 => {
                ident.storage = storage::StaticStorage;
                nexttoken();
                parse_variable_declaration(&mut ident, 0 as i32);
                current_block_23 = 6669252993407410313;
            }
            270 | 264 => {
                ident.storage = storage::AutoStorage;
                parse_variable_declaration(&mut ident, 0 as i32);
                current_block_23 = 6669252993407410313;
            }
            261 => {
                ident.storage = storage::ExplicitExternStorage;
                parse_declaration(&mut ident, 0 as i32);
                current_block_23 = 6669252993407410313;
            }
            258 | 123 => {
                level += 1;
                level;
                current_block_23 = 6669252993407410313;
            }
            259 => {
                if use_indentation != 0 {
                    if verbose != 0 && level != 1 as i32 {
                        file_error(
                            dcgettext(
                                0 as *const i8,
                                b"forced function body close\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            0 as *mut TOKSTK,
                        );
                    }
                    while level != 0 {
                        delete_autos(level);
                        level -= 1;
                        level;
                    }
                    current_block_23 = 6669252993407410313;
                } else {
                    current_block_23 = 16641983121783467691;
                }
            }
            125 => {
                current_block_23 = 16641983121783467691;
            }
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const i8,
                            b"unexpected end of file in function body\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                caller = 0 as *mut Symbol;
                return;
            }
            _ => {
                expression();
                current_block_23 = 6669252993407410313;
            }
        }
        match current_block_23 {
            16641983121783467691 => {
                delete_autos(level);
                level -= 1;
                level;
            }
            _ => {}
        }
    }
    caller = 0 as *mut Symbol;
}
#[no_mangle]
pub unsafe extern "C" fn get_knr_args(mut ident: *mut Ident) -> i32 {
    let mut parmcnt: i32 = 0;
    let mut stop: i32 = 0;
    let mut sp: Stackpos = [0; 1];
    let mut new_sp: Stackpos = [0; 1];
    let mut id: Ident = Ident {
        name: 0 as *mut i8,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: storage::ExternStorage,
    };
    match tok.type_0 {
        260 | 270 | 264 => {
            mark(sp.as_mut_ptr());
            parmcnt = 0 as i32;
            stop = 0 as i32;
            while stop == 0 && parmcnt < (*ident).parmcnt {
                id.type_end = -(1 as i32);
                let mut current_block_19: u64;
                match tok.type_0 {
                    123 | 258 => {
                        putback();
                        stop = 1 as i32;
                        current_block_19 = 14401909646449704462;
                    }
                    270 | 260 | 264 => {
                        putback();
                        mark(new_sp.as_mut_ptr());
                        if dcl(&mut id) == 0 as i32 {
                            parmcnt += 1;
                            parmcnt;
                            if tok.type_0 == ',' as i32 {
                                loop {
                                    tos = id.type_end;
                                    restore(new_sp.as_mut_ptr());
                                    dcl(&mut id);
                                    if !(tok.type_0 == ',' as i32) {
                                        break;
                                    }
                                }
                            } else if tok.type_0 != ';' as i32 {
                                putback();
                            }
                            current_block_19 = 14401909646449704462;
                        } else {
                            current_block_19 = 17615156732777954325;
                        }
                    }
                    _ => {
                        current_block_19 = 17615156732777954325;
                    }
                }
                match current_block_19 {
                    14401909646449704462 => {}
                    _ => {
                        restore(sp.as_mut_ptr());
                        return 1 as i32;
                    }
                }
                nexttoken();
            }
        }
        _ => {}
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn declare(mut ident: *mut Ident, mut maybe_knr: i32) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    if (*ident).storage as u32 == storage::AutoStorage as i32 as u32 {
        undo_save_stack();
        sp = install_ident((*ident).name, (*ident).storage);
        if parm_level != 0 {
            (*sp).level = parm_level;
            (*sp).flag = symbol_flag::symbol_parm;
        } else {
            (*sp).level = level;
        }
        (*sp).arity = -(1 as i32);
        return;
    }
    if (*ident).parmcnt >= 0 as i32
        && (maybe_knr == 0 || get_knr_args(ident) == 0 as i32)
        && !(tok.type_0 == '{' as i32 || tok.type_0 == 258 as i32
            || tok.type_0 == 270 as i32 || tok.type_0 == 272 as i32)
        || (*ident).parmcnt < 0 as i32
            && (*ident).storage as u32 == storage::ExplicitExternStorage as i32 as u32
    {
        undo_save_stack();
        return;
    }
    sp = get_symbol((*ident).name);
    if !((*sp).source).is_null() {
        if (*ident).storage as u32 == storage::StaticStorage as i32 as u32
            && ((*sp).storage as u32 != storage::StaticStorage as i32 as u32
                || level > 0 as i32)
        {
            sp = install_ident((*ident).name, (*ident).storage);
        } else {
            if (*sp).arity >= 0 as i32 {
                error_at_line(
                    0 as i32,
                    0 as i32,
                    filename,
                    (*ident).line as u32,
                    dcgettext(
                        0 as *const i8,
                        b"%s/%d redefined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*ident).name,
                    (*sp).arity,
                );
            } else {
                error_at_line(
                    0 as i32,
                    0 as i32,
                    filename,
                    (*ident).line as u32,
                    dcgettext(
                        0 as *const i8,
                        b"%s redefined\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*ident).name,
                );
            }
            error_at_line(
                0 as i32,
                0 as i32,
                (*sp).source,
                (*sp).def_line as u32,
                dcgettext(
                    0 as *const i8,
                    b"this is the place of previous definition\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    (*sp).type_0 = symtype::SymIdentifier;
    (*sp).arity = (*ident).parmcnt;
    ident_change_storage(
        sp,
        storage::from_libc_c_uint(
            (if (*ident).storage as u32 == storage::ExplicitExternStorage as i32 as u32 {
                storage::ExternStorage as i32 as u32
            } else {
                (*ident).storage as u32
            }) as u32,
        ),
    );
    (*sp).decl = finish_save_stack((*ident).name);
    (*sp).source = filename;
    (*sp).def_line = (*ident).line;
    (*sp).level = level;
    if debug != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"%s:%d: %s/%d defined to %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
            line_num,
            (*ident).name,
            (*ident).parmcnt,
            (*sp).decl,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn declare_type(mut ident: *mut Ident) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    undo_save_stack();
    sp = lookup((*ident).name);
    while !sp.is_null() {
        if (*sp).type_0 as u32 == symtype::SymToken as i32 as u32
            && (*sp).token_type == 270 as i32
        {
            break;
        }
        sp = (*sp).next;
    }
    if sp.is_null() {
        sp = install((*ident).name, 0x4 as i32);
    }
    (*sp).type_0 = symtype::SymToken;
    (*sp).token_type = 270 as i32;
    (*sp).source = filename;
    (*sp).def_line = (*ident).line;
    (*sp).ref_line = 0 as *mut linked_list;
    if debug != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"%s:%d: type %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename,
            line_num,
            (*ident).name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_symbol(mut name: *mut i8) -> *mut Symbol {
    let mut sp: *mut Symbol = lookup(name);
    if !sp.is_null() {
        while !sp.is_null() {
            if (*sp).type_0 as u32 == symtype::SymIdentifier as i32 as u32
                && strcmp((*sp).name, name) == 0 as i32
            {
                break;
            }
            sp = (*sp).next;
        }
        if !sp.is_null() {
            return sp;
        }
    }
    return install_ident(name, storage::ExternStorage);
}
#[no_mangle]
pub unsafe extern "C" fn add_reference(mut name: *mut i8, mut line: i32) -> *mut Symbol {
    let mut sp: *mut Symbol = get_symbol(name);
    let mut refptr: *mut Ref = 0 as *mut Ref;
    if (*sp).storage as u32 == storage::AutoStorage as i32 as u32
        || (*sp).storage as u32 == storage::StaticStorage as i32 as u32
            && globals_only() != 0
    {
        return 0 as *mut Symbol;
    }
    refptr = xmalloc(::core::mem::size_of::<Ref>() as u64) as *mut Ref;
    (*refptr).source = filename;
    (*refptr).line = line;
    if ((*sp).ref_line).is_null() {
        (*sp).ref_line = linked_list_create(
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    linked_list_append(&mut (*sp).ref_line, refptr as *mut libc::c_void);
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn call(mut name: *mut i8, mut line: i32) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    sp = add_reference(name, line);
    if sp.is_null() {
        return;
    }
    if (*sp).arity < 0 as i32 {
        (*sp).arity = 0 as i32;
    }
    if !caller.is_null() {
        if data_in_list(caller as *mut libc::c_void, (*sp).caller) == 0 {
            linked_list_append(&mut (*sp).caller, caller as *mut libc::c_void);
        }
        if data_in_list(sp as *mut libc::c_void, (*caller).callee) == 0 {
            linked_list_append(&mut (*caller).callee, sp as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn reference(mut name: *mut i8, mut line: i32) {
    let mut sp: *mut Symbol = add_reference(name, line);
    if sp.is_null() {
        return;
    }
    if !caller.is_null() {
        if data_in_list(caller as *mut libc::c_void, (*sp).caller) == 0 {
            linked_list_append(&mut (*sp).caller, caller as *mut libc::c_void);
        }
        if data_in_list(sp as *mut libc::c_void, (*caller).callee) == 0 {
            linked_list_append(&mut (*caller).callee, sp as *mut libc::c_void);
        }
    }
}