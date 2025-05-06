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
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn print_level(lev: i32, last: i32);
    fn newline();
    static mut outfile: *mut FILE;
    static mut print_as_tree: i32;
    static mut brief_listing: i32;
    static mut emacs_option: i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_symbol {
    pub direct: i32,
    pub level: i32,
    pub last: i32,
    pub sym: *mut Symbol,
}
#[no_mangle]
pub unsafe extern "C" fn print_function_name(
    mut sym: *mut Symbol,
    mut has_subtree: i32,
) {
    fprintf(outfile, b"%s\0" as *const u8 as *const i8, (*sym).name);
    if (*sym).arity >= 0 as i32 {
        fprintf(outfile, b"()\0" as *const u8 as *const i8);
    }
    if !((*sym).decl).is_null() {
        fprintf(
            outfile,
            b" <%s at %s:%d>\0" as *const u8 as *const i8,
            (*sym).decl,
            (*sym).source,
            (*sym).def_line,
        );
    }
    if (*sym).active != 0 {
        fprintf(
            outfile,
            b" (recursive: see %d)\0" as *const u8 as *const i8,
            (*sym).active - 1 as i32,
        );
        return;
    }
    if (*sym).recursive != 0 {
        fprintf(outfile, b" (R)\0" as *const u8 as *const i8);
    }
    if print_as_tree == 0 && has_subtree != 0 {
        fprintf(outfile, b":\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn print_symbol(
    mut outfile_0: *mut FILE,
    mut line: i32,
    mut s: *mut output_symbol,
) -> i32 {
    let mut has_subtree: i32 = if (*s).direct != 0 {
        ((*(*s).sym).callee != 0 as *mut libc::c_void as *mut linked_list) as i32
    } else {
        ((*(*s).sym).caller != 0 as *mut libc::c_void as *mut linked_list) as i32
    };
    print_level((*s).level, (*s).last);
    print_function_name((*s).sym, has_subtree);
    if brief_listing != 0 {
        if (*(*s).sym).expand_line != 0 {
            fprintf(
                outfile_0,
                b" [see %d]\0" as *const u8 as *const i8,
                (*(*s).sym).expand_line,
            );
            return 1 as i32;
        } else if !((*(*s).sym).callee).is_null() {
            (*(*s).sym).expand_line = line;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gnu_output_handler(
    mut cmd: cflow_output_command,
    mut outfile_0: *mut FILE,
    mut line: i32,
    mut data: *mut libc::c_void,
    mut handler_data: *mut libc::c_void,
) -> i32 {
    match cmd as u32 {
        1 => {
            if emacs_option != 0 {
                fprintf(
                    outfile_0,
                    b";; This file is generated by %s. -*- cflow -*-\0" as *const u8
                        as *const i8,
                    b"GNU cflow 1.4\0" as *const u8 as *const i8,
                );
                newline();
            }
        }
        3 => {
            fprintf(outfile_0, b"\n\0" as *const u8 as *const i8);
        }
        6 => {
            fprintf(outfile_0, b"%s\0" as *const u8 as *const i8, data as *mut i8);
        }
        5 => return print_symbol(outfile_0, line, data as *mut output_symbol),
        0 | 2 | 4 | _ => {}
    }
    return 0 as i32;
}