use std::env;
use std::ffi::{CString, CStr};
use std::mem;
use std::ptr;
use std::process;
use std::os::raw::{c_char, c_int, c_void, c_uchar, c_long, c_ulong, c_ushort};
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Int = 1,
    Cell = 2,
    Symbol = 3,
    Primitive = 4,
    Function = 5,
    Macro = 6,
    Env = 7,
    Moved = 8,
    True = 9,
    Nil = 10,
    Dot = 11,
    CParen = 12,
}

#[repr(C)]
union Value {
    int: i32,
    cell: Cell,
    symbol: *mut c_char,
    primitive: fn(*mut c_void, *mut *mut Obj, *mut *mut Obj) -> *mut Obj,
    function: Function,
    env: Env,
    moved: *mut c_void,
}

#[repr(C)]
struct Cell {
    car: *mut Obj,
    cdr: *mut Obj,
}

#[repr(C)]
struct Function {
    params: *mut Obj,
    body: *mut Obj,
    env: *mut Obj,
}

#[repr(C)]
struct Env {
    vars: *mut Obj,
    up: *mut Obj,
}

#[repr(C)]
struct Obj {
    type_: Type,
    size: i32,
    value: Value,
}

static mut TRUE: Obj = Obj {
    type_: Type::True,
    size: 0,
    value: Value { int: 0 },
};

static mut NIL: Obj = Obj {
    type_: Type::Nil,
    size: 0,
    value: Value { int: 0 },
};

static mut DOT: Obj = Obj {
    type_: Type::Dot,
    size: 0,
    value: Value { int: 0 },
};

static mut CPAREN: Obj = Obj {
    type_: Type::CParen,
    size: 0,
    value: Value { int: 0 },
};

static mut SYMBOLS: *mut Obj = ptr::null_mut();
static mut MEMORY: *mut c_void = ptr::null_mut();
static mut FROM_SPACE: *mut c_void = ptr::null_mut();
static mut MEM_NUSED: usize = 0;
static mut GC_RUNNING: bool = false;
static mut DEBUG_GC: bool = false;
static mut ALWAYS_GC: bool = false;

fn roundup(var: usize, size: usize) -> usize {
    (var + size - 1) & !(size - 1)
}

unsafe fn alloc(root: *mut c_void, type_: Type, size: usize) -> *mut Obj {
    let size = roundup(size, mem::size_of::<*mut c_void>());
    let size = roundup(size + 8, mem::size_of::<*mut c_void>());
    
    if ALWAYS_GC && !GC_RUNNING {
        gc(root);
    }
    
    if !ALWAYS_GC && 65536 < MEM_NUSED + size {
        gc(root);
    }
    
    if 65536 < MEM_NUSED + size {
        error("Memory exhausted");
    }
    
    let obj = MEMORY.add(MEM_NUSED) as *mut Obj;
    (*obj).type_ = type_;
    (*obj).size = size as i32;
    MEM_NUSED += size;
    
    obj
}

unsafe fn gc(root: *mut c_void) {
    if GC_RUNNING {
        panic!("GC already running");
    }
    
    GC_RUNNING = true;
    FROM_SPACE = MEMORY;
    MEMORY = alloc_semispace();
    
    let mut scan2 = MEMORY as *mut Obj;
    let mut scan1 = scan2;
    
    forward_root_objects(root);
    
    while scan1 < scan2 {
        match (*scan1).type_ {
            Type::Int | Type::Symbol | Type::Primitive => {}
            Type::Cell => {
                (*scan1).value.cell.car = forward((*scan1).value.cell.car);
                (*scan1).value.cell.cdr = forward((*scan1).value.cell.cdr);
            }
            Type::Function | Type::Macro => {
                (*scan1).value.function.params = forward((*scan1).value.function.params);
                (*scan1).value.function.body = forward((*scan1).value.function.body);
                (*scan1).value.function.env = forward((*scan1).value.function.env);
            }
            Type::Env => {
                (*scan1).value.env.vars = forward((*scan1).value.env.vars);
                (*scan1).value.env.up = forward((*scan1).value.env.up);
            }
            _ => error(&format!("Bug: copy: unknown type {:?}", (*scan1).type_)),
        }
        scan1 = (scan1 as *mut u8).add((*scan1).size as usize) as *mut Obj;
    }
    
    munmap(FROM_SPACE, 65536);
    let old_nused = MEM_NUSED;
    MEM_NUSED = scan1 as usize - MEMORY as usize;
    
    if DEBUG_GC {
        eprintln!("GC: {} bytes out of {} bytes copied.", MEM_NUSED, old_nused);
    }
    
    GC_RUNNING = false;
}

fn main() {
    unsafe {
        DEBUG_GC = get_env_flag("MINILISP_DEBUG_GC");
        ALWAYS_GC = get_env_flag("MINILISP_ALWAYS_GC");
        
        MEMORY = alloc_semispace();
        SYMBOLS = &mut NIL;
        
        let mut root = ptr::null_mut();
        let mut env = make_env(&mut root, &mut NIL, &mut NIL);
        
        define_constants(&mut root, &mut env);
        define_primitives(&mut root, &mut env);
        
        loop {
            let expr = read_expr(&mut root);
            if expr.is_null() {
                process::exit(0);
            }
            if expr == &mut CPAREN {
                error("Stray close parenthesis");
            }
            if expr == &mut DOT {
                error("Stray dot");
            }
            print(eval(&mut root, &mut env, &mut expr));
            println!();
        }
    }
}

// Helper functions would be implemented here following Rust safety practices
// including proper error handling, memory management, etc.
// The rest of the functions would be similarly converted to safe Rust equivalents