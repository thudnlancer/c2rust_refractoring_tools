use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_uint, c_void, c_long, c_ulong};
use std::collections::LinkedList;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct _IO_FILE {
    _flags: c_int,
    _IO_read_ptr: *mut c_char,
    _IO_read_end: *mut c_char,
    _IO_read_base: *mut c_char,
    _IO_write_base: *mut c_char,
    _IO_write_ptr: *mut c_char,
    _IO_write_end: *mut c_char,
    _IO_buf_base: *mut c_char,
    _IO_buf_end: *mut c_char,
    _IO_save_base: *mut c_char,
    _IO_backup_base: *mut c_char,
    _IO_save_end: *mut c_char,
    _markers: *mut _IO_marker,
    _chain: *mut _IO_FILE,
    _fileno: c_int,
    _flags2: c_int,
    _old_offset: c_long,
    _cur_column: c_ushort,
    _vtable_offset: c_char,
    _shortbuf: [c_char; 1],
    _lock: *mut c_void,
    _offset: c_long,
    __pad1: *mut c_void,
    __pad2: *mut c_void,
    __pad3: *mut c_void,
    __pad4: *mut c_void,
    __pad5: c_ulong,
    _mode: c_int,
    _unused2: [c_char; 20],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: c_int,
}

pub type FILE = _IO_FILE;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct obstack {
    chunk_size: c_long,
    chunk: *mut _obstack_chunk,
    object_base: *mut c_char,
    next_free: *mut c_char,
    chunk_limit: *mut c_char,
    temp: C2RustUnnamed,
    alignment_mask: c_int,
    chunkfun: Option<unsafe extern "C" fn(*mut c_void, c_long) -> *mut _obstack_chunk>,
    freefun: Option<unsafe extern "C" fn(*mut c_void, *mut _obstack_chunk)>,
    extra_arg: *mut c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    c2rust_padding: [u8; 7],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct _obstack_chunk {
    limit: *mut c_char,
    prev: *mut _obstack_chunk,
    contents: [c_char; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union C2RustUnnamed {
    tempint: c_long,
    tempptr: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct linked_list_entry {
    next: *mut linked_list_entry,
    prev: *mut linked_list_entry,
    list: *mut linked_list,
    data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct linked_list {
    free_data: Option<unsafe extern "C" fn(*mut c_void)>,
    head: *mut linked_list_entry,
    tail: *mut linked_list_entry,
}

pub type linked_list_free_data_fp = Option<unsafe extern "C" fn(*mut c_void)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum symtype {
    SymUndefined = 0,
    SymToken = 1,
    SymIdentifier = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum storage {
    ExternStorage = 0,
    ExplicitExternStorage = 1,
    StaticStorage = 2,
    AutoStorage = 3,
    AnyStorage = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ref {
    line: c_int,
    source: *mut c_char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum symbol_flag {
    symbol_none = 0,
    symbol_temp = 1,
    symbol_parm = 2,
    symbol_alias = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct symbol {
    owner: *mut table_entry,
    next: *mut symbol,
    entry: *mut linked_list_entry,
    type_: symtype,
    name: *mut c_char,
    flag: symbol_flag,
    alias: *mut symbol,
    active: c_int,
    expand_line: c_int,
    token_type: c_int,
    source: *mut c_char,
    def_line: c_int,
    ref_line: *mut linked_list,
    level: c_int,
    decl: *mut c_char,
    storage: storage,
    arity: c_int,
    recursive: c_int,
    ord: c_ulong,
    caller: *mut linked_list,
    callee: *mut linked_list,
}

pub type Symbol = symbol;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TOKSTK {
    type_: c_int,
    token: *mut c_char,
    line: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ident {
    name: *mut c_char,
    type_end: c_int,
    parmcnt: c_int,
    line: c_int,
    storage: storage,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct YYSTYPE {
    str_: *mut c_char,
}

pub type Stackpos = [c_int; 1];

static mut level: c_int = 0;
static mut caller: *mut Symbol = ptr::null_mut();
static mut text_stk: obstack = obstack {
    chunk_size: 0,
    chunk: ptr::null_mut(),
    object_base: ptr::null_mut(),
    next_free: ptr::null_mut(),
    chunk_limit: ptr::null_mut(),
    temp: C2RustUnnamed { tempint: 0 },
    alignment_mask: 0,
    chunkfun: None,
    freefun: None,
    extra_arg: ptr::null_mut(),
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};

static mut parm_level: c_int = 0;
static mut tok: TOKSTK = TOKSTK {
    type_: 0,
    token: ptr::null_mut(),
    line: 0,
};

static mut token_stack: *mut TOKSTK = ptr::null_mut();
static mut tos: c_int = 0;
static mut curs: c_int = 0;
static mut token_stack_length: c_int = 64;
static mut token_stack_increase: c_int = 32;
static mut need_space: c_int = 0;

unsafe fn print_token(tokptr: *mut TOKSTK) {
    match (*tokptr).type_ {
        260 | 270 | 257 | 265 | 264 | 272 | 273 | 266 => {
            eprint!("`{}'", CStr::from_ptr((*tokptr).token).to_string_lossy());
        }
        258 | 123 => eprint!("`{{'"),
        259 | 125 => eprint!("`}}'"),
        261 => eprint!("`extern'"),
        262 => eprint!("`static'"),
        263 => eprint!("`typedef'"),
        271 => {
            eprint!("\"{}\"", CStr::from_ptr((*tokptr).token).to_string_lossy());
        }
        _ => eprint!("`{}'", (*tokptr).type_ as u8 as char),
    }
}

unsafe fn file_error(msg: *mut c_char, tokptr: *mut TOKSTK) {
    eprint!(
        "{}:{}: {}",
        CStr::from_ptr(filename).to_string_lossy(),
        tok.line,
        CStr::from_ptr(msg).to_string_lossy()
    );
    
    if !tokptr.is_null() {
        eprint!(" near ");
        print_token(tokptr);
    }
    eprintln!();
}

pub unsafe extern "C" fn mark(pos: *mut c_int) {
    *pos.offset(0) = curs;
}

pub unsafe extern "C" fn restore(pos: *mut c_int) {
    curs = *pos.offset(0);
    if curs != 0 {
        tok = *token_stack.offset((curs - 1) as isize);
    }
}

pub unsafe extern "C" fn tokpush(type_: c_int, line: c_int, token: *mut c_char) {
    (*token_stack.offset(tos as isize)).type_ = type_;
    (*token_stack.offset(tos as isize)).token = token;
    (*token_stack.offset(tos as isize)).line = line;
    tos += 1;
    
    if tos == token_stack_length {
        token_stack_length += token_stack_increase;
        token_stack = libc::realloc(
            token_stack as *mut c_void,
            (token_stack_length as usize) * mem::size_of::<TOKSTK>()
        ) as *mut TOKSTK;
    }
}

pub unsafe extern "C" fn cleanup_stack() {
    let delta = tos - curs;
    if delta != 0 {
        libc::memmove(
            token_stack as *mut c_void,
            token_stack.offset(curs as isize) as *const c_void,
            (delta as usize) * mem::size_of::<TOKSTK>()
        );
    }
    tos = delta;
    curs = 0;
}

pub unsafe extern "C" fn clearstack() {
    curs = 0;
    tos = curs;
}

pub unsafe extern "C" fn nexttoken() -> c_int {
    let type_ = if curs == tos {
        let type_ = get_token();
        tokpush(type_, line_num, yylval.str_);
        type_
    } else {
        0
    };
    
    tok = *token_stack.offset(curs as isize);
    curs += 1;
    tok.type_
}

pub unsafe extern "C" fn putback() -> c_int {
    if curs == 0 {
        eprintln!("INTERNAL ERROR: cannot return token to stream");
    }
    
    curs -= 1;
    
    if curs > 0 {
        tok.type_ = (*token_stack.offset((curs - 1) as isize)).type_;
        tok.token = (*token_stack.offset((curs - 1) as isize)).token;
    } else {
        tok.type_ = 0;
    }
    
    tok.type_
}

pub unsafe extern "C" fn init_parse() {
    _obstack_begin(
        &mut text_stk,
        0,
        0,
        Some(libc::malloc as unsafe extern "C" fn(c_long) -> *mut c_void),
        Some(libc::free as unsafe extern "C" fn(*mut c_void)),
    );
    
    token_stack = libc::malloc(
        (token_stack_length as usize) * mem::size_of::<TOKSTK>()
    ) as *mut TOKSTK;
    
    clearstack();
}

pub unsafe extern "C" fn save_token(tokptr: *mut TOKSTK) {
    let len = match (*tokptr).type_ {
        260 | 270 | 264 | 272 | 257 | 273 => {
            if need_space != 0 {
                let __o = &mut text_stk;
                if ((*__o).next_free).offset(1) > (*__o).chunk_limit {
                    _obstack_newchunk(__o, 1);
                }
                let fresh1 = (*__o).next_free;
                (*__o).next_free = ((*__o).next_free).offset(1);
                *fresh1 = b' ' as c_char;
            }
            
            let len = libc::strlen((*tokptr).token) as c_int;
            let __o = &mut text_stk;
            if ((*__o).next_free).offset(len as isize) > (*__o).chunk_limit {
                _obstack_newchunk(__o, len);
            }
            
            libc::memcpy(
                (*__o).next_free as *mut c_void,
                (*tokptr).token as *const c_void,
                len as usize
            );
            
            (*__o).next_free = ((*__o).next_free).offset(len as isize);
            need_space = 1;
            len
        }
        // ... rest of the cases
        _ => 0
    };
}

static mut start_pos: Stackpos = [0; 1];
static mut save_end: c_int = 0;

pub unsafe extern "C" fn save_stack() {
    mark(start_pos.as_mut_ptr());
    save_end = curs - 1;
}

pub unsafe extern "C" fn undo_save_stack() {
    save_end = -1;
}

pub unsafe extern "C" fn finish_save_stack(name: *mut c_char) -> *mut c_char {
    let mut i = 0;
    let mut level = 0;
    let mut found_ident = if omit_symbol_names_option == 0 { 1 } else { 0 };
    need_space = 0;
    
    while i < save_end {
        match (*token_stack.offset(i as isize)).type_ {
            40 => {
                if omit_arguments_option != 0 {
                    if level == 0 {
                        save_token(token_stack.offset(i as isize));
                    }
                    level += 1;
                }
            }
            41 => {
                if omit_arguments_option != 0 {
                    level -= 1;
                }
            }
            260 => {
                if found_ident == 0 && libc::strcmp(name, (*token_stack.offset(i as isize)).token) == 0 {
                    need_space = 1;
                    found_ident = 1;
                }
            }
            _ => {
                if level == 0 {
                    save_token(token_stack.offset(i as isize));
                }
            }
        }
        i += 1;
    }
    
    let __o = &mut text_stk;
    if ((*__o).next_free).offset(1) > (*__o).chunk_limit {
        _obstack_newchunk(__o, 1);
    }
    let fresh9 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh9 = 0;
    
    let value = (*__o).object_base as *mut c_void;
    if (*__o).next_free == value as *mut c_char {
        (*__o).set_maybe_empty_object(1);
    }
    
    (*__o).next_free = (if mem::size_of::<c_long>() < mem::size_of::<*mut c_void>() {
        (*__o).object_base
    } else {
        ptr::null_mut()
    }).offset(
        ((*__o).next_free as isize - (if mem::size_of::<c_long>() < mem::size_of::<*mut c_void>() {
            (*__o).object_base as isize
        } else {
            0
        }) + (*__o).alignment_mask as isize) & !(*__o).alignment_mask as isize
    );
    
    if (*__o).next_free as isize - (*__o).chunk as isize > (*__o).chunk_limit as isize - (*__o).chunk as isize {
        (*__o).next_free = (*__o).chunk_limit;
    }
    
    (*__o).object_base = (*__o).next_free;
    value as *mut c_char
}

// ... rest of the functions would follow the same pattern of conversion