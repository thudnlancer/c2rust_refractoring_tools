use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
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
pub type __uint8_t = u8;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type va_list = __builtin_va_list;
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
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
pub enum C2RustUnnamed_0 {
    TINT = 1,
    TCELL,
    TSYMBOL,
    TPRIMITIVE,
    TFUNCTION,
    TMACRO,
    TENV,
    TMOVED,
    TTRUE,
    TNIL,
    TDOT,
    TCPAREN,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::TINT => 1,
            C2RustUnnamed_0::TCELL => 2,
            C2RustUnnamed_0::TSYMBOL => 3,
            C2RustUnnamed_0::TPRIMITIVE => 4,
            C2RustUnnamed_0::TFUNCTION => 5,
            C2RustUnnamed_0::TMACRO => 6,
            C2RustUnnamed_0::TENV => 7,
            C2RustUnnamed_0::TMOVED => 8,
            C2RustUnnamed_0::TTRUE => 9,
            C2RustUnnamed_0::TNIL => 10,
            C2RustUnnamed_0::TDOT => 11,
            C2RustUnnamed_0::TCPAREN => 12,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            1 => C2RustUnnamed_0::TINT,
            2 => C2RustUnnamed_0::TCELL,
            3 => C2RustUnnamed_0::TSYMBOL,
            4 => C2RustUnnamed_0::TPRIMITIVE,
            5 => C2RustUnnamed_0::TFUNCTION,
            6 => C2RustUnnamed_0::TMACRO,
            7 => C2RustUnnamed_0::TENV,
            8 => C2RustUnnamed_0::TMOVED,
            9 => C2RustUnnamed_0::TTRUE,
            10 => C2RustUnnamed_0::TNIL,
            11 => C2RustUnnamed_0::TDOT,
            12 => C2RustUnnamed_0::TCPAREN,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Obj {
    pub type_0: i32,
    pub size: i32,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub value: i32,
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub name: [i8; 1],
    pub fn_0: Option<Primitive>,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
    pub c2rust_unnamed_1: C2RustUnnamed_2,
    pub moved: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub vars: *mut Obj,
    pub up: *mut Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub params: *mut Obj,
    pub body: *mut Obj,
    pub env: *mut Obj,
}
pub type Primitive = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut *mut Obj,
    *mut *mut Obj,
) -> *mut Obj;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub car: *mut Obj,
    pub cdr: *mut Obj,
}
#[inline]
unsafe extern "C" fn getchar() -> i32 {
    return _IO_getc(stdin);
}
unsafe extern "C" fn error(mut fmt: *mut i8, mut args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    exit(1 as i32);
}
static mut True: *mut Obj = &{
    let mut init = Obj {
        type_0: C2RustUnnamed_0::TTRUE as i32,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0 },
    };
    init
} as *const Obj as *mut Obj;
static mut Nil: *mut Obj = &{
    let mut init = Obj {
        type_0: C2RustUnnamed_0::TNIL as i32,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0 },
    };
    init
} as *const Obj as *mut Obj;
static mut Dot: *mut Obj = &{
    let mut init = Obj {
        type_0: C2RustUnnamed_0::TDOT as i32,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0 },
    };
    init
} as *const Obj as *mut Obj;
static mut Cparen: *mut Obj = &{
    let mut init = Obj {
        type_0: C2RustUnnamed_0::TCPAREN as i32,
        size: 0,
        c2rust_unnamed: C2RustUnnamed_1 { value: 0 },
    };
    init
} as *const Obj as *mut Obj;
static mut Symbols: *mut Obj = 0 as *const Obj as *mut Obj;
static mut memory: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
static mut from_space: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
static mut mem_nused: size_t = 0 as i32 as size_t;
static mut gc_running: bool = 0 as i32 != 0;
static mut debug_gc: bool = 0 as i32 != 0;
static mut always_gc: bool = 0 as i32 != 0;
#[inline]
unsafe extern "C" fn roundup(mut var: size_t, mut size: size_t) -> size_t {
    return var.wrapping_add(size).wrapping_sub(1 as i32 as u64)
        & !size.wrapping_sub(1 as i32 as u64);
}
unsafe extern "C" fn alloc(
    mut root: *mut libc::c_void,
    mut type_0: i32,
    mut size: size_t,
) -> *mut Obj {
    size = roundup(size, ::core::mem::size_of::<*mut libc::c_void>() as u64);
    size = (size as u64).wrapping_add(8 as u64) as size_t as size_t;
    size = roundup(size, ::core::mem::size_of::<*mut libc::c_void>() as u64);
    if always_gc as i32 != 0 && !gc_running {
        gc(root);
    }
    if !always_gc && (65536 as i32 as u64) < mem_nused.wrapping_add(size) {
        gc(root);
    }
    if (65536 as i32 as u64) < mem_nused.wrapping_add(size) {
        error(b"Memory exhausted\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut obj: *mut Obj = memory.offset(mem_nused as isize) as *mut Obj;
    (*obj).type_0 = type_0;
    (*obj).size = size as i32;
    mem_nused = (mem_nused as u64).wrapping_add(size) as size_t as size_t;
    return obj;
}
static mut scan1: *mut Obj = 0 as *const Obj as *mut Obj;
static mut scan2: *mut Obj = 0 as *const Obj as *mut Obj;
#[inline]
unsafe extern "C" fn forward(mut obj: *mut Obj) -> *mut Obj {
    let mut offset: ptrdiff_t = (obj as *mut uint8_t)
        .offset_from(from_space as *mut uint8_t) as i64;
    if offset < 0 as i32 as i64 || 65536 as i32 as i64 <= offset {
        return obj;
    }
    if (*obj).type_0 == C2RustUnnamed_0::TMOVED as i32 {
        return (*obj).c2rust_unnamed.moved as *mut Obj;
    }
    let mut newloc: *mut Obj = scan2;
    memcpy(newloc as *mut libc::c_void, obj as *const libc::c_void, (*obj).size as u64);
    scan2 = (scan2 as *mut uint8_t).offset((*obj).size as isize) as *mut Obj;
    (*obj).type_0 = C2RustUnnamed_0::TMOVED as i32;
    (*obj).c2rust_unnamed.moved = newloc as *mut libc::c_void;
    return newloc;
}
unsafe extern "C" fn alloc_semispace() -> *mut libc::c_void {
    return mmap(
        0 as *mut libc::c_void,
        65536 as i32 as size_t,
        0x1 as i32 | 0x2 as i32,
        0x2 as i32 | 0x20 as i32,
        -(1 as i32),
        0 as i32 as __off_t,
    );
}
unsafe extern "C" fn forward_root_objects(mut root: *mut libc::c_void) {
    Symbols = forward(Symbols);
    let mut frame: *mut *mut libc::c_void = root as *mut *mut libc::c_void;
    while !frame.is_null() {
        let mut i: i32 = 1 as i32;
        while *frame.offset(i as isize) != -(1 as i32) as *mut libc::c_void {
            if !(*frame.offset(i as isize)).is_null() {
                let ref mut fresh0 = *frame.offset(i as isize);
                *fresh0 = forward(*frame.offset(i as isize) as *mut Obj)
                    as *mut libc::c_void;
            }
            i += 1;
            i;
        }
        frame = *(frame as *mut *mut *mut libc::c_void);
    }
}
unsafe extern "C" fn gc(mut root: *mut libc::c_void) {
    if !gc_running {} else {
        __assert_fail(
            b"!gc_running\0" as *const u8 as *const i8,
            b"minilisp.c\0" as *const u8 as *const i8,
            275 as i32 as u32,
            (*::core::mem::transmute::<&[u8; 16], &[i8; 16]>(b"void gc(void *)\0"))
                .as_ptr(),
        );
    }
    'c_3062: {
        if !gc_running {} else {
            __assert_fail(
                b"!gc_running\0" as *const u8 as *const i8,
                b"minilisp.c\0" as *const u8 as *const i8,
                275 as i32 as u32,
                (*::core::mem::transmute::<&[u8; 16], &[i8; 16]>(b"void gc(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    gc_running = 1 as i32 != 0;
    from_space = memory;
    memory = alloc_semispace();
    scan2 = memory as *mut Obj;
    scan1 = scan2;
    forward_root_objects(root);
    while scan1 < scan2 {
        match (*scan1).type_0 {
            1 | 3 | 4 => {}
            2 => {
                (*scan1).c2rust_unnamed.c2rust_unnamed.car = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed.car,
                );
                (*scan1).c2rust_unnamed.c2rust_unnamed.cdr = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed.cdr,
                );
            }
            5 | 6 => {
                (*scan1).c2rust_unnamed.c2rust_unnamed_0.params = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed_0.params,
                );
                (*scan1).c2rust_unnamed.c2rust_unnamed_0.body = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed_0.body,
                );
                (*scan1).c2rust_unnamed.c2rust_unnamed_0.env = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed_0.env,
                );
            }
            7 => {
                (*scan1).c2rust_unnamed.c2rust_unnamed_1.vars = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed_1.vars,
                );
                (*scan1).c2rust_unnamed.c2rust_unnamed_1.up = forward(
                    (*scan1).c2rust_unnamed.c2rust_unnamed_1.up,
                );
            }
            _ => {
                error(
                    b"Bug: copy: unknown type %d\0" as *const u8 as *const i8 as *mut i8,
                    (*scan1).type_0,
                );
            }
        }
        scan1 = (scan1 as *mut uint8_t).offset((*scan1).size as isize) as *mut Obj;
    }
    munmap(from_space, 65536 as i32 as size_t);
    let mut old_nused: size_t = mem_nused;
    mem_nused = (scan1 as *mut uint8_t).offset_from(memory as *mut uint8_t) as i64
        as size_t;
    if debug_gc {
        fprintf(
            stderr,
            b"GC: %zu bytes out of %zu bytes copied.\n\0" as *const u8 as *const i8,
            mem_nused,
            old_nused,
        );
    }
    gc_running = 0 as i32 != 0;
}
unsafe extern "C" fn make_int(mut root: *mut libc::c_void, mut value: i32) -> *mut Obj {
    let mut r: *mut Obj = alloc(
        root,
        C2RustUnnamed_0::TINT as i32,
        ::core::mem::size_of::<i32>() as u64,
    );
    (*r).c2rust_unnamed.value = value;
    return r;
}
unsafe extern "C" fn cons(
    mut root: *mut libc::c_void,
    mut car: *mut *mut Obj,
    mut cdr: *mut *mut Obj,
) -> *mut Obj {
    let mut cell: *mut Obj = alloc(
        root,
        C2RustUnnamed_0::TCELL as i32,
        (::core::mem::size_of::<*mut Obj>() as u64).wrapping_mul(2 as i32 as u64),
    );
    (*cell).c2rust_unnamed.c2rust_unnamed.car = *car;
    (*cell).c2rust_unnamed.c2rust_unnamed.cdr = *cdr;
    return cell;
}
unsafe extern "C" fn make_symbol(
    mut root: *mut libc::c_void,
    mut name: *mut i8,
) -> *mut Obj {
    let mut sym: *mut Obj = alloc(
        root,
        C2RustUnnamed_0::TSYMBOL as i32,
        (strlen(name)).wrapping_add(1 as i32 as u64),
    );
    strcpy(((*sym).c2rust_unnamed.name).as_mut_ptr(), name);
    return sym;
}
unsafe extern "C" fn make_primitive(
    mut root: *mut libc::c_void,
    mut fn_0: Option<Primitive>,
) -> *mut Obj {
    let mut r: *mut Obj = alloc(
        root,
        C2RustUnnamed_0::TPRIMITIVE as i32,
        ::core::mem::size_of::<Option<Primitive>>() as u64,
    );
    (*r).c2rust_unnamed.fn_0 = fn_0;
    return r;
}
unsafe extern "C" fn make_function(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut type_0: i32,
    mut params: *mut *mut Obj,
    mut body: *mut *mut Obj,
) -> *mut Obj {
    if type_0 == C2RustUnnamed_0::TFUNCTION as i32
        || type_0 == C2RustUnnamed_0::TMACRO as i32
    {} else {
        __assert_fail(
            b"type == C2RustUnnamed_0::TFUNCTION || type == C2RustUnnamed_0::TMACRO\0"
                as *const u8 as *const i8,
            b"minilisp.c\0" as *const u8 as *const i8,
            357 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[i8; 56],
            >(b"Obj *make_function(void *, Obj **, int, Obj **, Obj **)\0"))
                .as_ptr(),
        );
    }
    'c_3428: {
        if type_0 == C2RustUnnamed_0::TFUNCTION as i32
            || type_0 == C2RustUnnamed_0::TMACRO as i32
        {} else {
            __assert_fail(
                b"type == C2RustUnnamed_0::TFUNCTION || type == C2RustUnnamed_0::TMACRO\0"
                    as *const u8 as *const i8,
                b"minilisp.c\0" as *const u8 as *const i8,
                357 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[i8; 56],
                >(b"Obj *make_function(void *, Obj **, int, Obj **, Obj **)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut r: *mut Obj = alloc(
        root,
        type_0,
        (::core::mem::size_of::<*mut Obj>() as u64).wrapping_mul(3 as i32 as u64),
    );
    (*r).c2rust_unnamed.c2rust_unnamed_0.params = *params;
    (*r).c2rust_unnamed.c2rust_unnamed_0.body = *body;
    (*r).c2rust_unnamed.c2rust_unnamed_0.env = *env;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn make_env(
    mut root: *mut libc::c_void,
    mut vars: *mut *mut Obj,
    mut up: *mut *mut Obj,
) -> *mut Obj {
    let mut r: *mut Obj = alloc(
        root,
        C2RustUnnamed_0::TENV as i32,
        (::core::mem::size_of::<*mut Obj>() as u64).wrapping_mul(2 as i32 as u64),
    );
    (*r).c2rust_unnamed.c2rust_unnamed_1.vars = *vars;
    (*r).c2rust_unnamed.c2rust_unnamed_1.up = *up;
    return r;
}
unsafe extern "C" fn acons(
    mut root: *mut libc::c_void,
    mut x: *mut *mut Obj,
    mut y: *mut *mut Obj,
    mut a: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut cell: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *cell = cons(root, x, y);
    return cons(root, cell, a);
}
#[no_mangle]
pub static mut symbol_chars: [i8; 19] = unsafe {
    *::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"~!@#$%^&*-_=+:/?<>\0")
};
unsafe extern "C" fn peek() -> i32 {
    let mut c: i32 = getchar();
    ungetc(c, stdin);
    return c;
}
unsafe extern "C" fn reverse(mut p: *mut Obj) -> *mut Obj {
    let mut ret: *mut Obj = Nil;
    while p != Nil {
        let mut head: *mut Obj = p;
        p = (*p).c2rust_unnamed.c2rust_unnamed.cdr;
        (*head).c2rust_unnamed.c2rust_unnamed.cdr = ret;
        ret = head;
    }
    return ret;
}
unsafe extern "C" fn skip_line() {
    loop {
        let mut c: i32 = getchar();
        if c == -(1 as i32) || c == '\n' as i32 {
            return;
        }
        if c == '\r' as i32 {
            if peek() == '\n' as i32 {
                getchar();
            }
            return;
        }
    };
}
unsafe extern "C" fn read_list(mut root: *mut libc::c_void) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut obj: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut head: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut last: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *head = Nil;
    loop {
        *obj = read_expr(root);
        if (*obj).is_null() {
            error(b"Unclosed parenthesis\0" as *const u8 as *const i8 as *mut i8);
        }
        if *obj == Cparen {
            return reverse(*head);
        }
        if *obj == Dot {
            *last = read_expr(root);
            if read_expr(root) != Cparen {
                error(
                    b"Closed parenthesis expected after dot\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            let mut ret: *mut Obj = reverse(*head);
            (**head).c2rust_unnamed.c2rust_unnamed.cdr = *last;
            return ret;
        }
        *head = cons(root, obj, head);
    };
}
unsafe extern "C" fn intern(mut root: *mut libc::c_void, mut name: *mut i8) -> *mut Obj {
    let mut p: *mut Obj = Symbols;
    while p != Nil {
        if strcmp(
            name,
            ((*(*p).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.name).as_mut_ptr(),
        ) == 0 as i32
        {
            return (*p).c2rust_unnamed.c2rust_unnamed.car;
        }
        p = (*p).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *sym = make_symbol(root, name);
    Symbols = cons(root, sym, &mut Symbols);
    return *sym;
}
unsafe extern "C" fn read_quote(mut root: *mut libc::c_void) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut tmp: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *sym = intern(root, b"quote\0" as *const u8 as *const i8 as *mut i8);
    *tmp = read_expr(root);
    *tmp = cons(root, tmp, &mut Nil);
    *tmp = cons(root, sym, tmp);
    return *tmp;
}
unsafe extern "C" fn read_number(mut val: i32) -> i32 {
    while *(*__ctype_b_loc()).offset(peek() as isize) as i32
        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        val = val * 10 as i32 + (getchar() - '0' as i32);
    }
    return val;
}
unsafe extern "C" fn read_symbol(mut root: *mut libc::c_void, mut c: i8) -> *mut Obj {
    let mut buf: [i8; 201] = [0; 201];
    buf[0 as i32 as usize] = c;
    let mut len: i32 = 1 as i32;
    while *(*__ctype_b_loc()).offset(peek() as isize) as i32
        & C2RustUnnamed::_ISalnum as i32 as libc::c_ushort as i32 != 0
        || !(strchr(symbol_chars.as_ptr(), peek())).is_null()
    {
        if 200 as i32 <= len {
            error(b"Symbol name too long\0" as *const u8 as *const i8 as *mut i8);
        }
        let fresh1 = len;
        len = len + 1;
        buf[fresh1 as usize] = getchar() as i8;
    }
    buf[len as usize] = '\0' as i32 as i8;
    return intern(root, buf.as_mut_ptr());
}
unsafe extern "C" fn read_expr(mut root: *mut libc::c_void) -> *mut Obj {
    loop {
        let mut c: i32 = getchar();
        if c == ' ' as i32 || c == '\n' as i32 || c == '\r' as i32 || c == '\t' as i32 {
            continue;
        }
        if c == -(1 as i32) {
            return 0 as *mut Obj;
        }
        if c == ';' as i32 {
            skip_line();
        } else {
            if c == '(' as i32 {
                return read_list(root);
            }
            if c == ')' as i32 {
                return Cparen;
            }
            if c == '.' as i32 {
                return Dot;
            }
            if c == '\'' as i32 {
                return read_quote(root);
            }
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
            {
                return make_int(root, read_number(c - '0' as i32));
            }
            if c == '-' as i32
                && *(*__ctype_b_loc()).offset(peek() as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
            {
                return make_int(root, -read_number(0 as i32));
            }
            if *(*__ctype_b_loc()).offset(c as isize) as i32
                & C2RustUnnamed::_ISalpha as i32 as libc::c_ushort as i32 != 0
                || !(strchr(symbol_chars.as_ptr(), c)).is_null()
            {
                return read_symbol(root, c as i8);
            }
            error(
                b"Don't know how to handle %c\0" as *const u8 as *const i8 as *mut i8,
                c,
            );
        }
    };
}
unsafe extern "C" fn print(mut obj: *mut Obj) {
    match (*obj).type_0 {
        2 => {
            printf(b"(\0" as *const u8 as *const i8);
            loop {
                print((*obj).c2rust_unnamed.c2rust_unnamed.car);
                if (*obj).c2rust_unnamed.c2rust_unnamed.cdr == Nil {
                    break;
                }
                if (*(*obj).c2rust_unnamed.c2rust_unnamed.cdr).type_0
                    != C2RustUnnamed_0::TCELL as i32
                {
                    printf(b" . \0" as *const u8 as *const i8);
                    print((*obj).c2rust_unnamed.c2rust_unnamed.cdr);
                    break;
                } else {
                    printf(b" \0" as *const u8 as *const i8);
                    obj = (*obj).c2rust_unnamed.c2rust_unnamed.cdr;
                }
            }
            printf(b")\0" as *const u8 as *const i8);
            return;
        }
        1 => {
            printf(b"%d\0" as *const u8 as *const i8, (*obj).c2rust_unnamed.value);
            return;
        }
        3 => {
            printf(
                b"%s\0" as *const u8 as *const i8,
                ((*obj).c2rust_unnamed.name).as_mut_ptr(),
            );
            return;
        }
        4 => {
            printf(b"<primitive>\0" as *const u8 as *const i8);
            return;
        }
        5 => {
            printf(b"<function>\0" as *const u8 as *const i8);
            return;
        }
        6 => {
            printf(b"<macro>\0" as *const u8 as *const i8);
            return;
        }
        8 => {
            printf(b"<moved>\0" as *const u8 as *const i8);
            return;
        }
        9 => {
            printf(b"t\0" as *const u8 as *const i8);
            return;
        }
        10 => {
            printf(b"()\0" as *const u8 as *const i8);
            return;
        }
        _ => {
            error(
                b"Bug: print: Unknown tag type: %d\0" as *const u8 as *const i8
                    as *mut i8,
                (*obj).type_0,
            );
        }
    };
}
unsafe extern "C" fn length(mut list: *mut Obj) -> i32 {
    let mut len: i32 = 0 as i32;
    while (*list).type_0 == C2RustUnnamed_0::TCELL as i32 {
        len += 1;
        len;
        list = (*list).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    return if list == Nil { len } else { -(1 as i32) };
}
unsafe extern "C" fn add_variable(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut sym: *mut *mut Obj,
    mut val: *mut *mut Obj,
) {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut vars: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut tmp: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *vars = (**env).c2rust_unnamed.c2rust_unnamed_1.vars;
    *tmp = acons(root, sym, val, vars);
    (**env).c2rust_unnamed.c2rust_unnamed_1.vars = *tmp;
}
unsafe extern "C" fn push_env(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut vars: *mut *mut Obj,
    mut vals: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut map: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut val: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *map = Nil;
    while (**vars).type_0 == C2RustUnnamed_0::TCELL as i32 {
        if (**vals).type_0 != C2RustUnnamed_0::TCELL as i32 {
            error(
                b"Cannot apply function: number of argument does not match\0"
                    as *const u8 as *const i8 as *mut i8,
            );
        }
        *sym = (**vars).c2rust_unnamed.c2rust_unnamed.car;
        *val = (**vals).c2rust_unnamed.c2rust_unnamed.car;
        *map = acons(root, sym, val, map);
        *vars = (**vars).c2rust_unnamed.c2rust_unnamed.cdr;
        *vals = (**vals).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    if *vars != Nil {
        *map = acons(root, vars, vals, map);
    }
    return make_env(root, map, env);
}
unsafe extern "C" fn progn(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut lp: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut r: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *lp = *list;
    while *lp != Nil {
        *r = (**lp).c2rust_unnamed.c2rust_unnamed.car;
        *r = eval(root, env, r);
        *lp = (**lp).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    return *r;
}
unsafe extern "C" fn eval_list(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 6] = [0 as *mut libc::c_void; 6];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 4 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(4 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut head: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut lp: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut expr: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    let mut result: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(4 as i32 as isize)
        as *mut *mut Obj;
    *head = Nil;
    lp = list;
    while *lp != Nil {
        *expr = (**lp).c2rust_unnamed.c2rust_unnamed.car;
        *result = eval(root, env, expr);
        *head = cons(root, result, head);
        *lp = (**lp).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    return reverse(*head);
}
unsafe extern "C" fn is_list(mut obj: *mut Obj) -> bool {
    return obj == Nil || (*obj).type_0 == C2RustUnnamed_0::TCELL as i32;
}
unsafe extern "C" fn apply_func(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut fn_0: *mut *mut Obj,
    mut args: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut params: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut newenv: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut body: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *params = (**fn_0).c2rust_unnamed.c2rust_unnamed_0.params;
    *newenv = (**fn_0).c2rust_unnamed.c2rust_unnamed_0.env;
    *newenv = push_env(root, newenv, params, args);
    *body = (**fn_0).c2rust_unnamed.c2rust_unnamed_0.body;
    return progn(root, newenv, body);
}
unsafe extern "C" fn apply(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut fn_0: *mut *mut Obj,
    mut args: *mut *mut Obj,
) -> *mut Obj {
    if !is_list(*args) {
        error(b"argument must be a list\0" as *const u8 as *const i8 as *mut i8);
    }
    if (**fn_0).type_0 == C2RustUnnamed_0::TPRIMITIVE as i32 {
        return ((**fn_0).c2rust_unnamed.fn_0)
            .expect("non-null function pointer")(root, env, args);
    }
    if (**fn_0).type_0 == C2RustUnnamed_0::TFUNCTION as i32 {
        let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
        root_ADD_ROOT_[0 as i32 as usize] = root;
        let mut i: i32 = 1 as i32;
        while i <= 1 as i32 {
            root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
            i += 1;
            i;
        }
        root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32)
            as *mut libc::c_void;
        root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
        let mut eargs: *mut *mut Obj = root_ADD_ROOT_
            .as_mut_ptr()
            .offset(1 as i32 as isize) as *mut *mut Obj;
        *eargs = eval_list(root, env, args);
        return apply_func(root, env, fn_0, eargs);
    }
    error(b"not supported\0" as *const u8 as *const i8 as *mut i8);
}
unsafe extern "C" fn find(mut env: *mut *mut Obj, mut sym: *mut Obj) -> *mut Obj {
    let mut p: *mut Obj = *env;
    while p != Nil {
        let mut cell: *mut Obj = (*p).c2rust_unnamed.c2rust_unnamed_1.vars;
        while cell != Nil {
            let mut bind: *mut Obj = (*cell).c2rust_unnamed.c2rust_unnamed.car;
            if sym == (*bind).c2rust_unnamed.c2rust_unnamed.car {
                return bind;
            }
            cell = (*cell).c2rust_unnamed.c2rust_unnamed.cdr;
        }
        p = (*p).c2rust_unnamed.c2rust_unnamed_1.up;
    }
    return 0 as *mut Obj;
}
unsafe extern "C" fn macroexpand(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut obj: *mut *mut Obj,
) -> *mut Obj {
    if (**obj).type_0 != C2RustUnnamed_0::TCELL as i32
        || (*(**obj).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TSYMBOL as i32
    {
        return *obj;
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut bind: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut macro_0: *mut *mut Obj = root_ADD_ROOT_
        .as_mut_ptr()
        .offset(2 as i32 as isize) as *mut *mut Obj;
    let mut args: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *bind = find(env, (**obj).c2rust_unnamed.c2rust_unnamed.car);
    if (*bind).is_null()
        || (*(**bind).c2rust_unnamed.c2rust_unnamed.cdr).type_0
            != C2RustUnnamed_0::TMACRO as i32
    {
        return *obj;
    }
    *macro_0 = (**bind).c2rust_unnamed.c2rust_unnamed.cdr;
    *args = (**obj).c2rust_unnamed.c2rust_unnamed.cdr;
    return apply_func(root, env, macro_0, args);
}
unsafe extern "C" fn eval(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut obj: *mut *mut Obj,
) -> *mut Obj {
    match (**obj).type_0 {
        1 | 4 | 5 | 9 | 10 => return *obj,
        3 => {
            let mut bind: *mut Obj = find(env, *obj);
            if bind.is_null() {
                error(
                    b"Undefined symbol: %s\0" as *const u8 as *const i8 as *mut i8,
                    ((**obj).c2rust_unnamed.name).as_mut_ptr(),
                );
            }
            return (*bind).c2rust_unnamed.c2rust_unnamed.cdr;
        }
        2 => {
            let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
            root_ADD_ROOT_[0 as i32 as usize] = root;
            let mut i: i32 = 1 as i32;
            while i <= 3 as i32 {
                root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
                i += 1;
                i;
            }
            root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32)
                as *mut libc::c_void;
            root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
            let mut fn_0: *mut *mut Obj = root_ADD_ROOT_
                .as_mut_ptr()
                .offset(1 as i32 as isize) as *mut *mut Obj;
            let mut expanded: *mut *mut Obj = root_ADD_ROOT_
                .as_mut_ptr()
                .offset(2 as i32 as isize) as *mut *mut Obj;
            let mut args: *mut *mut Obj = root_ADD_ROOT_
                .as_mut_ptr()
                .offset(3 as i32 as isize) as *mut *mut Obj;
            *expanded = macroexpand(root, env, obj);
            if *expanded != *obj {
                return eval(root, env, expanded);
            }
            *fn_0 = (**obj).c2rust_unnamed.c2rust_unnamed.car;
            *fn_0 = eval(root, env, fn_0);
            *args = (**obj).c2rust_unnamed.c2rust_unnamed.cdr;
            if (**fn_0).type_0 != C2RustUnnamed_0::TPRIMITIVE as i32
                && (**fn_0).type_0 != C2RustUnnamed_0::TFUNCTION as i32
            {
                error(
                    b"The head of a list must be a function\0" as *const u8 as *const i8
                        as *mut i8,
                );
            }
            return apply(root, env, fn_0, args);
        }
        _ => {
            error(
                b"Bug: eval: Unknown tag type: %d\0" as *const u8 as *const i8
                    as *mut i8,
                (**obj).type_0,
            );
        }
    };
}
unsafe extern "C" fn prim_quote(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 1 as i32 {
        error(b"Malformed quote\0" as *const u8 as *const i8 as *mut i8);
    }
    return (**list).c2rust_unnamed.c2rust_unnamed.car;
}
unsafe extern "C" fn prim_cons(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 2 as i32 {
        error(b"Malformed cons\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut cell: *mut Obj = eval_list(root, env, list);
    (*cell).c2rust_unnamed.c2rust_unnamed.cdr = (*(*cell)
        .c2rust_unnamed
        .c2rust_unnamed
        .cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    return cell;
}
unsafe extern "C" fn prim_car(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut args: *mut Obj = eval_list(root, env, list);
    if (*(*args).c2rust_unnamed.c2rust_unnamed.car).type_0
        != C2RustUnnamed_0::TCELL as i32
        || (*args).c2rust_unnamed.c2rust_unnamed.cdr != Nil
    {
        error(b"Malformed car\0" as *const u8 as *const i8 as *mut i8);
    }
    return (*(*args).c2rust_unnamed.c2rust_unnamed.car)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
}
unsafe extern "C" fn prim_cdr(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut args: *mut Obj = eval_list(root, env, list);
    if (*(*args).c2rust_unnamed.c2rust_unnamed.car).type_0
        != C2RustUnnamed_0::TCELL as i32
        || (*args).c2rust_unnamed.c2rust_unnamed.cdr != Nil
    {
        error(b"Malformed cdr\0" as *const u8 as *const i8 as *mut i8);
    }
    return (*(*args).c2rust_unnamed.c2rust_unnamed.car)
        .c2rust_unnamed
        .c2rust_unnamed
        .cdr;
}
unsafe extern "C" fn prim_setq(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 2 as i32
        || (*(**list).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TSYMBOL as i32
    {
        error(b"Malformed setq\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut bind: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut value: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *bind = find(env, (**list).c2rust_unnamed.c2rust_unnamed.car);
    if (*bind).is_null() {
        error(
            b"Unbound variable %s\0" as *const u8 as *const i8 as *mut i8,
            ((*(**list).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.name)
                .as_mut_ptr(),
        );
    }
    *value = (*(**list).c2rust_unnamed.c2rust_unnamed.cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    *value = eval(root, env, value);
    (**bind).c2rust_unnamed.c2rust_unnamed.cdr = *value;
    return *value;
}
unsafe extern "C" fn prim_setcar(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut args: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *args = eval_list(root, env, list);
    if length(*args) != 2 as i32
        || (*(**args).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TCELL as i32
    {
        error(b"Malformed setcar\0" as *const u8 as *const i8 as *mut i8);
    }
    (*(**args).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.c2rust_unnamed.car = (*(**args)
        .c2rust_unnamed
        .c2rust_unnamed
        .cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    return (**args).c2rust_unnamed.c2rust_unnamed.car;
}
unsafe extern "C" fn prim_while(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) < 2 as i32 {
        error(b"Malformed while\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut cond: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut exprs: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *cond = (**list).c2rust_unnamed.c2rust_unnamed.car;
    while eval(root, env, cond) != Nil {
        *exprs = (**list).c2rust_unnamed.c2rust_unnamed.cdr;
        eval_list(root, env, exprs);
    }
    return Nil;
}
unsafe extern "C" fn prim_gensym(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    static mut count: i32 = 0 as i32;
    let mut buf: [i8; 10] = [0; 10];
    let fresh2 = count;
    count = count + 1;
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 10]>() as u64,
        b"G__%d\0" as *const u8 as *const i8,
        fresh2,
    );
    return make_symbol(root, buf.as_mut_ptr());
}
unsafe extern "C" fn prim_plus(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut sum: i32 = 0 as i32;
    let mut args: *mut Obj = eval_list(root, env, list);
    while args != Nil {
        if (*(*args).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TINT as i32
        {
            error(b"+ takes only numbers\0" as *const u8 as *const i8 as *mut i8);
        }
        sum += (*(*args).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.value;
        args = (*args).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    return make_int(root, sum);
}
unsafe extern "C" fn prim_minus(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut args: *mut Obj = eval_list(root, env, list);
    let mut p: *mut Obj = args;
    while p != Nil {
        if (*(*p).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TINT as i32
        {
            error(b"- takes only numbers\0" as *const u8 as *const i8 as *mut i8);
        }
        p = (*p).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    if (*args).c2rust_unnamed.c2rust_unnamed.cdr == Nil {
        return make_int(
            root,
            -(*(*args).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.value,
        );
    }
    let mut r: i32 = (*(*args).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.value;
    let mut p_0: *mut Obj = (*args).c2rust_unnamed.c2rust_unnamed.cdr;
    while p_0 != Nil {
        r -= (*(*p_0).c2rust_unnamed.c2rust_unnamed.car).c2rust_unnamed.value;
        p_0 = (*p_0).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    return make_int(root, r);
}
unsafe extern "C" fn prim_lt(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut args: *mut Obj = eval_list(root, env, list);
    if length(args) != 2 as i32 {
        error(b"malformed <\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut x: *mut Obj = (*args).c2rust_unnamed.c2rust_unnamed.car;
    let mut y: *mut Obj = (*(*args).c2rust_unnamed.c2rust_unnamed.cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    if (*x).type_0 != C2RustUnnamed_0::TINT as i32
        || (*y).type_0 != C2RustUnnamed_0::TINT as i32
    {
        error(b"< takes only numbers\0" as *const u8 as *const i8 as *mut i8);
    }
    return if (*x).c2rust_unnamed.value < (*y).c2rust_unnamed.value {
        True
    } else {
        Nil
    };
}
unsafe extern "C" fn handle_function(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
    mut type_0: i32,
) -> *mut Obj {
    if (**list).type_0 != C2RustUnnamed_0::TCELL as i32
        || !is_list((**list).c2rust_unnamed.c2rust_unnamed.car)
        || (*(**list).c2rust_unnamed.c2rust_unnamed.cdr).type_0
            != C2RustUnnamed_0::TCELL as i32
    {
        error(b"Malformed lambda\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut p: *mut Obj = (**list).c2rust_unnamed.c2rust_unnamed.car;
    while (*p).type_0 == C2RustUnnamed_0::TCELL as i32 {
        if (*(*p).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TSYMBOL as i32
        {
            error(b"Parameter must be a symbol\0" as *const u8 as *const i8 as *mut i8);
        }
        p = (*p).c2rust_unnamed.c2rust_unnamed.cdr;
    }
    if p != Nil && (*p).type_0 != C2RustUnnamed_0::TSYMBOL as i32 {
        error(b"Parameter must be a symbol\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut params: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut body: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *params = (**list).c2rust_unnamed.c2rust_unnamed.car;
    *body = (**list).c2rust_unnamed.c2rust_unnamed.cdr;
    return make_function(root, env, type_0, params, body);
}
unsafe extern "C" fn prim_lambda(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    return handle_function(root, env, list, C2RustUnnamed_0::TFUNCTION as i32);
}
unsafe extern "C" fn handle_defun(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
    mut type_0: i32,
) -> *mut Obj {
    if (*(**list).c2rust_unnamed.c2rust_unnamed.car).type_0
        != C2RustUnnamed_0::TSYMBOL as i32
        || (*(**list).c2rust_unnamed.c2rust_unnamed.cdr).type_0
            != C2RustUnnamed_0::TCELL as i32
    {
        error(b"Malformed defun\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut fn_0: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut rest: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *sym = (**list).c2rust_unnamed.c2rust_unnamed.car;
    *rest = (**list).c2rust_unnamed.c2rust_unnamed.cdr;
    *fn_0 = handle_function(root, env, rest, type_0);
    add_variable(root, env, sym, fn_0);
    return *fn_0;
}
unsafe extern "C" fn prim_defun(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    return handle_defun(root, env, list, C2RustUnnamed_0::TFUNCTION as i32);
}
unsafe extern "C" fn prim_define(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 2 as i32
        || (*(**list).c2rust_unnamed.c2rust_unnamed.car).type_0
            != C2RustUnnamed_0::TSYMBOL as i32
    {
        error(b"Malformed define\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut value: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *sym = (**list).c2rust_unnamed.c2rust_unnamed.car;
    *value = (*(**list).c2rust_unnamed.c2rust_unnamed.cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    *value = eval(root, env, value);
    add_variable(root, env, sym, value);
    return *value;
}
unsafe extern "C" fn prim_defmacro(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    return handle_defun(root, env, list, C2RustUnnamed_0::TMACRO as i32);
}
unsafe extern "C" fn prim_macroexpand(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 1 as i32 {
        error(b"Malformed macroexpand\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut body: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *body = (**list).c2rust_unnamed.c2rust_unnamed.car;
    return macroexpand(root, env, body);
}
unsafe extern "C" fn prim_println(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut tmp: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *tmp = (**list).c2rust_unnamed.c2rust_unnamed.car;
    print(eval(root, env, tmp));
    printf(b"\n\0" as *const u8 as *const i8);
    return Nil;
}
unsafe extern "C" fn prim_if(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) < 2 as i32 {
        error(b"Malformed if\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut root_ADD_ROOT_: [*mut libc::c_void; 5] = [0 as *mut libc::c_void; 5];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 3 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(3 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut cond: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut then: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    let mut els: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(3 as i32 as isize)
        as *mut *mut Obj;
    *cond = (**list).c2rust_unnamed.c2rust_unnamed.car;
    *cond = eval(root, env, cond);
    if *cond != Nil {
        *then = (*(**list).c2rust_unnamed.c2rust_unnamed.cdr)
            .c2rust_unnamed
            .c2rust_unnamed
            .car;
        return eval(root, env, then);
    }
    *els = (*(**list).c2rust_unnamed.c2rust_unnamed.cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .cdr;
    return if *els == Nil { Nil } else { progn(root, env, els) };
}
unsafe extern "C" fn prim_num_eq(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 2 as i32 {
        error(b"Malformed =\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut values: *mut Obj = eval_list(root, env, list);
    let mut x: *mut Obj = (*values).c2rust_unnamed.c2rust_unnamed.car;
    let mut y: *mut Obj = (*(*values).c2rust_unnamed.c2rust_unnamed.cdr)
        .c2rust_unnamed
        .c2rust_unnamed
        .car;
    if (*x).type_0 != C2RustUnnamed_0::TINT as i32
        || (*y).type_0 != C2RustUnnamed_0::TINT as i32
    {
        error(b"= only takes numbers\0" as *const u8 as *const i8 as *mut i8);
    }
    return if (*x).c2rust_unnamed.value == (*y).c2rust_unnamed.value {
        True
    } else {
        Nil
    };
}
unsafe extern "C" fn prim_eq(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut list: *mut *mut Obj,
) -> *mut Obj {
    if length(*list) != 2 as i32 {
        error(b"Malformed eq\0" as *const u8 as *const i8 as *mut i8);
    }
    let mut values: *mut Obj = eval_list(root, env, list);
    return if (*values).c2rust_unnamed.c2rust_unnamed.car
        == (*(*values).c2rust_unnamed.c2rust_unnamed.cdr)
            .c2rust_unnamed
            .c2rust_unnamed
            .car
    {
        True
    } else {
        Nil
    };
}
unsafe extern "C" fn add_primitive(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
    mut name: *mut i8,
    mut fn_0: Option<Primitive>,
) {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut prim: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *sym = intern(root, name);
    *prim = make_primitive(root, fn_0);
    add_variable(root, env, sym, prim);
}
unsafe extern "C" fn define_constants(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
) {
    let mut root_ADD_ROOT_: [*mut libc::c_void; 3] = [0 as *mut libc::c_void; 3];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 1 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(1 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut sym: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    *sym = intern(root, b"t\0" as *const u8 as *const i8 as *mut i8);
    add_variable(root, env, sym, &mut True);
}
unsafe extern "C" fn define_primitives(
    mut root: *mut libc::c_void,
    mut env: *mut *mut Obj,
) {
    add_primitive(
        root,
        env,
        b"quote\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_quote
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"cons\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_cons
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"car\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_car
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"cdr\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_cdr
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"setq\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_setq
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"setcar\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_setcar
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"while\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_while
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"gensym\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_gensym
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"+\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_plus
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"-\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_minus
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"<\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_lt
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"define\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_define
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"defun\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_defun
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"defmacro\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_defmacro
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"macroexpand\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_macroexpand
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"lambda\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_lambda
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"if\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_if
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"=\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_num_eq
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"eq\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_eq
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
    add_primitive(
        root,
        env,
        b"println\0" as *const u8 as *const i8 as *mut i8,
        Some(
            prim_println
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut Obj,
                    *mut *mut Obj,
                ) -> *mut Obj,
        ),
    );
}
unsafe extern "C" fn getEnvFlag(mut name: *mut i8) -> bool {
    let mut val: *mut i8 = getenv(name);
    return !val.is_null() && *val.offset(0 as i32 as isize) as i32 != 0;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    debug_gc = getEnvFlag(b"MINILISP_DEBUG_GC\0" as *const u8 as *const i8 as *mut i8);
    always_gc = getEnvFlag(b"MINILISP_ALWAYS_GC\0" as *const u8 as *const i8 as *mut i8);
    memory = alloc_semispace();
    Symbols = Nil;
    let mut root: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut root_ADD_ROOT_: [*mut libc::c_void; 4] = [0 as *mut libc::c_void; 4];
    root_ADD_ROOT_[0 as i32 as usize] = root;
    let mut i: i32 = 1 as i32;
    while i <= 2 as i32 {
        root_ADD_ROOT_[i as usize] = 0 as *mut libc::c_void;
        i += 1;
        i;
    }
    root_ADD_ROOT_[(2 as i32 + 1 as i32) as usize] = -(1 as i32) as *mut libc::c_void;
    root = root_ADD_ROOT_.as_mut_ptr() as *mut libc::c_void;
    let mut env: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(1 as i32 as isize)
        as *mut *mut Obj;
    let mut expr: *mut *mut Obj = root_ADD_ROOT_.as_mut_ptr().offset(2 as i32 as isize)
        as *mut *mut Obj;
    *env = make_env(root, &mut Nil, &mut Nil);
    define_constants(root, env);
    define_primitives(root, env);
    loop {
        *expr = read_expr(root);
        if (*expr).is_null() {
            return 0 as i32;
        }
        if *expr == Cparen {
            error(b"Stray close parenthesis\0" as *const u8 as *const i8 as *mut i8);
        }
        if *expr == Dot {
            error(b"Stray dot\0" as *const u8 as *const i8 as *mut i8);
        }
        print(eval(root, env, expr));
        printf(b"\n\0" as *const u8 as *const i8);
    };
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