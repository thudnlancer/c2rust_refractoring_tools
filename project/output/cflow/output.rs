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
    pub type cflow_depmap;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut outname: *mut i8;
    static mut print_option: i32;
    static mut level_indent: [*mut i8; 0];
    static mut level_end: [*mut i8; 0];
    static mut level_begin: *mut i8;
    static mut print_levels: i32;
    static mut print_line_numbers: i32;
    static mut reverse_tree: i32;
    static mut start_name: *mut i8;
    static mut max_depth: i32;
    fn lookup(_: *const i8) -> *mut Symbol;
    fn collect_symbols(
        _: *mut *mut *mut Symbol,
        sel: Option<unsafe extern "C" fn() -> i32>,
        rescnt: size_t,
    ) -> size_t;
    fn collect_functions(return_sym: *mut *mut *mut Symbol) -> size_t;
    fn depmap_alloc(count: size_t) -> cflow_depmap_t;
    fn depmap_set(dmap: cflow_depmap_t, row: size_t, col: size_t);
    fn depmap_tc(dmap: cflow_depmap_t);
    fn depmap_isset(dmap: cflow_depmap_t, row: size_t, col: size_t) -> i32;
    fn include_symbol(sym: *mut Symbol) -> i32;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
pub struct output_driver {
    pub name: *mut i8,
    pub handler: Option<
        unsafe extern "C" fn(
            cflow_output_command,
            *mut FILE,
            i32,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub handler_data: *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_symbol {
    pub direct: i32,
    pub level: i32,
    pub last: i32,
    pub sym: *mut Symbol,
}
pub type cflow_depmap_t = *mut cflow_depmap;
#[no_mangle]
pub static mut level_mark: *mut u8 = 0 as *const u8 as *mut u8;
#[no_mangle]
pub static mut level_mark_size: i32 = 0 as i32;
#[no_mangle]
pub static mut level_mark_incr: i32 = 128 as i32;
#[no_mangle]
pub static mut out_line: i32 = 1 as i32;
#[no_mangle]
pub static mut outfile: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn set_level_mark(mut lev: i32, mut mark: i32) {
    if lev >= level_mark_size {
        level_mark_size += level_mark_incr;
        level_mark = xrealloc(level_mark as *mut libc::c_void, level_mark_size as size_t)
            as *mut u8;
    }
    *level_mark.offset(lev as isize) = mark as u8;
}
#[no_mangle]
pub unsafe extern "C" fn print_level(mut lev: i32, mut last: i32) {
    let mut i: i32 = 0;
    if print_line_numbers != 0 {
        fprintf(outfile, b"%5d \0" as *const u8 as *const i8, out_line);
    }
    if print_levels != 0 {
        fprintf(outfile, b"{%4d} \0" as *const u8 as *const i8, lev);
    }
    fprintf(outfile, b"%s\0" as *const u8 as *const i8, level_begin);
    i = 0 as i32;
    while i < lev {
        fprintf(
            outfile,
            b"%s\0" as *const u8 as *const i8,
            *level_indent.as_mut_ptr().offset(*level_mark.offset(i as isize) as isize),
        );
        i += 1;
        i;
    }
    fprintf(
        outfile,
        b"%s\0" as *const u8 as *const i8,
        *level_end.as_mut_ptr().offset(last as isize),
    );
}
static mut driver_index: i32 = 0;
static mut driver_max: i32 = 0 as i32;
#[no_mangle]
pub static mut output_driver: [output_driver; 8] = [output_driver {
    name: 0 as *const i8 as *mut i8,
    handler: None,
    handler_data: 0 as *const libc::c_void as *mut libc::c_void,
}; 8];
#[no_mangle]
pub unsafe extern "C" fn register_output(
    mut name: *const i8,
    mut handler: Option<
        unsafe extern "C" fn(
            cflow_output_command,
            *mut FILE,
            i32,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> i32,
    >,
    mut handler_data: *mut libc::c_void,
) -> i32 {
    if driver_max == 8 as i32 - 1 as i32 {
        abort();
    }
    output_driver[driver_max as usize].name = strdup(name);
    output_driver[driver_max as usize].handler = handler;
    output_driver[driver_max as usize].handler_data = handler_data;
    let fresh0 = driver_max;
    driver_max = driver_max + 1;
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn select_output_driver(mut name: *const i8) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < driver_max {
        if strcmp(output_driver[i as usize].name, name) == 0 as i32 {
            driver_index = i;
            return 0 as i32;
        }
        i += 1;
        i;
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn output_init() {
    (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_init,
        0 as *mut FILE,
        0 as i32,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn newline() {
    (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_newline,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
    out_line += 1;
    out_line;
}
unsafe extern "C" fn begin() {
    (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_begin,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn end() {
    (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_end,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn separator() {
    (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_separator,
        outfile,
        out_line,
        0 as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn print_symbol(
    mut direct: i32,
    mut level: i32,
    mut last: i32,
    mut sym: *mut Symbol,
) -> i32 {
    let mut output_symbol: output_symbol = output_symbol {
        direct: 0,
        level: 0,
        last: 0,
        sym: 0 as *mut Symbol,
    };
    output_symbol.direct = direct;
    output_symbol.level = level;
    output_symbol.last = last;
    output_symbol.sym = sym;
    return (output_driver[driver_index as usize].handler)
        .expect(
            "non-null function pointer",
        )(
        cflow_output_command::cflow_output_symbol,
        outfile,
        out_line,
        &mut output_symbol as *mut output_symbol as *mut libc::c_void,
        output_driver[driver_index as usize].handler_data,
    );
}
unsafe extern "C" fn compare(
    mut ap: *const libc::c_void,
    mut bp: *const libc::c_void,
) -> i32 {
    let mut a: *const *mut Symbol = ap as *const *mut Symbol;
    let mut b: *const *mut Symbol = bp as *const *mut Symbol;
    return strcmp((**a).name, (**b).name);
}
unsafe extern "C" fn is_var(mut symp: *mut Symbol) -> i32 {
    if include_symbol(symp) != 0 {
        if (*symp).type_0 as u32 == symtype::SymIdentifier as i32 as u32 {
            return ((*symp).storage as u32 == storage::ExternStorage as i32 as u32
                || (*symp).storage as u32 == storage::StaticStorage as i32 as u32) as i32
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn symbol_is_function(mut symp: *mut Symbol) -> i32 {
    return ((*symp).type_0 as u32 == symtype::SymIdentifier as i32 as u32
        && (*symp).arity >= 0 as i32) as i32;
}
unsafe extern "C" fn clear_active(mut sym: *mut Symbol) {
    (*sym).active = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn print_refs(mut name: *mut i8, mut reflist: *mut linked_list) {
    let mut refptr: *mut Ref = 0 as *mut Ref;
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    p = if !reflist.is_null() { (*reflist).head } else { 0 as *mut linked_list_entry };
    while !p.is_null() {
        refptr = (*p).data as *mut Ref;
        fprintf(
            outfile,
            b"%s   %s:%d\n\0" as *const u8 as *const i8,
            name,
            (*refptr).source,
            (*refptr).line,
        );
        p = (*p).next;
    }
}
unsafe extern "C" fn print_function(mut symp: *mut Symbol) {
    if !((*symp).source).is_null() {
        fprintf(
            outfile,
            b"%s * %s:%d %s\n\0" as *const u8 as *const i8,
            (*symp).name,
            (*symp).source,
            (*symp).def_line,
            (*symp).decl,
        );
    }
    print_refs((*symp).name, (*symp).ref_line);
}
unsafe extern "C" fn print_type(mut symp: *mut Symbol) {
    if !((*symp).source).is_null() {
        fprintf(
            outfile,
            b"%s t %s:%d\n\0" as *const u8 as *const i8,
            (*symp).name,
            (*symp).source,
            (*symp).def_line,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xref_output() {
    let mut symbols: *mut *mut Symbol = 0 as *mut *mut Symbol;
    let mut symp: *mut Symbol = 0 as *mut Symbol;
    let mut i: size_t = 0;
    let mut num: size_t = 0;
    num = collect_symbols(
        &mut symbols,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Symbol) -> i32>,
            Option<unsafe extern "C" fn() -> i32>,
        >(Some(is_var as unsafe extern "C" fn(*mut Symbol) -> i32)),
        0 as i32 as size_t,
    );
    qsort(
        symbols as *mut libc::c_void,
        num,
        ::core::mem::size_of::<*mut Symbol>() as u64,
        Some(
            compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    i = 0 as i32 as size_t;
    while i < num {
        symp = *symbols.offset(i as isize);
        match (*symp).type_0 as u32 {
            2 => {
                print_function(symp);
            }
            1 => {
                print_type(symp);
            }
            0 | _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    free(symbols as *mut libc::c_void);
}
unsafe extern "C" fn set_active(mut sym: *mut Symbol) {
    (*sym).active = out_line;
}
unsafe extern "C" fn is_printable(mut p: *mut linked_list_entry) -> i32 {
    return (!p.is_null() && include_symbol((*p).data as *mut Symbol) != 0) as i32;
}
unsafe extern "C" fn is_last(mut p: *mut linked_list_entry) -> i32 {
    loop {
        p = (*p).next;
        if p.is_null() {
            break;
        }
        if is_printable(p) != 0 {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn direct_tree(mut lev: i32, mut last: i32, mut sym: *mut Symbol) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    let mut rc: i32 = 0;
    if (*sym).type_0 as u32 == symtype::SymUndefined as i32 as u32
        || max_depth != 0 && lev >= max_depth || include_symbol(sym) == 0
    {
        return;
    }
    rc = print_symbol(1 as i32, lev, last, sym);
    newline();
    if rc != 0 || (*sym).active != 0 {
        return;
    }
    set_active(sym);
    p = if !((*sym).callee).is_null() {
        (*(*sym).callee).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        set_level_mark(lev + 1 as i32, (is_last(p) == 0) as i32);
        direct_tree(lev + 1 as i32, is_last(p), (*p).data as *mut Symbol);
        p = (*p).next;
    }
    clear_active(sym);
}
unsafe extern "C" fn inverted_tree(mut lev: i32, mut last: i32, mut sym: *mut Symbol) {
    let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
    let mut rc: i32 = 0;
    if (*sym).type_0 as u32 == symtype::SymUndefined as i32 as u32
        || max_depth != 0 && lev >= max_depth || include_symbol(sym) == 0
    {
        return;
    }
    rc = print_symbol(0 as i32, lev, last, sym);
    newline();
    if rc != 0 || (*sym).active != 0 {
        return;
    }
    set_active(sym);
    p = if !((*sym).caller).is_null() {
        (*(*sym).caller).head
    } else {
        0 as *mut linked_list_entry
    };
    while !p.is_null() {
        set_level_mark(lev + 1 as i32, (is_last(p) == 0) as i32);
        inverted_tree(lev + 1 as i32, is_last(p), (*p).data as *mut Symbol);
        p = (*p).next;
    }
    clear_active(sym);
}
unsafe extern "C" fn tree_output() {
    let mut symbols: *mut *mut Symbol = 0 as *mut *mut Symbol;
    let mut main_sym: *mut Symbol = 0 as *mut Symbol;
    let mut i: size_t = 0;
    let mut num: size_t = 0;
    let mut depmap: cflow_depmap_t = 0 as *mut cflow_depmap;
    num = collect_functions(&mut symbols);
    i = 0 as i32 as size_t;
    while i < num {
        (**symbols.offset(i as isize)).ord = i;
        i = i.wrapping_add(1);
        i;
    }
    depmap = depmap_alloc(num);
    i = 0 as i32 as size_t;
    while i < num {
        if !((**symbols.offset(i as isize)).callee).is_null() {
            let mut p: *mut linked_list_entry = 0 as *mut linked_list_entry;
            p = if !((**symbols.offset(i as isize)).callee).is_null() {
                (*(**symbols.offset(i as isize)).callee).head
            } else {
                0 as *mut linked_list_entry
            };
            while !p.is_null() {
                let mut s: *mut Symbol = (*p).data as *mut Symbol;
                if symbol_is_function(s) != 0 {
                    depmap_set(depmap, i, (*((*p).data as *mut Symbol)).ord);
                }
                p = (*p).next;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    depmap_tc(depmap);
    i = 0 as i32 as size_t;
    while i < num {
        if depmap_isset(depmap, i, i) != 0 {
            (**symbols.offset(i as isize)).recursive = 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    free(depmap as *mut libc::c_void);
    free(symbols as *mut libc::c_void);
    num = collect_symbols(
        &mut symbols,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Symbol) -> i32>,
            Option<unsafe extern "C" fn() -> i32>,
        >(Some(is_var as unsafe extern "C" fn(*mut Symbol) -> i32)),
        0 as i32 as size_t,
    );
    qsort(
        symbols as *mut libc::c_void,
        num,
        ::core::mem::size_of::<*mut Symbol>() as u64,
        Some(
            compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    begin();
    if reverse_tree != 0 {
        i = 0 as i32 as size_t;
        while i < num {
            inverted_tree(0 as i32, 0 as i32, *symbols.offset(i as isize));
            separator();
            i = i.wrapping_add(1);
            i;
        }
    } else {
        main_sym = lookup(start_name);
        if !main_sym.is_null() {
            direct_tree(0 as i32, 0 as i32, main_sym);
            separator();
        } else {
            i = 0 as i32 as size_t;
            while i < num {
                if !((**symbols.offset(i as isize)).callee).is_null() {
                    direct_tree(0 as i32, 0 as i32, *symbols.offset(i as isize));
                    separator();
                }
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    end();
    free(symbols as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn output() {
    if strcmp(outname, b"-\0" as *const u8 as *const i8) == 0 as i32 {
        outfile = stdout;
    } else {
        outfile = fopen(outname, b"w\0" as *const u8 as *const i8);
        if outfile.is_null() {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"cannot open file `%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                outname,
            );
        }
    }
    set_level_mark(0 as i32, 0 as i32);
    if print_option & 0x1 as i32 != 0 {
        xref_output();
    }
    if print_option & 0x2 as i32 != 0 {
        tree_output();
    }
    fclose(outfile);
}