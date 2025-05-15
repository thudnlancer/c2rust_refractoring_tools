use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_long, c_uchar, c_ulong, c_ushort};
use std::ptr::{null, null_mut};

// Constants and types from original C code
const _ISalnum: c_uint = 8;
const _ISpunct: c_uint = 4;
const _IScntrl: c_uint = 2;
const _ISblank: c_uint = 1;
const _ISgraph: c_uint = 32768;
const _ISprint: c_uint = 16384;
const _ISspace: c_uint = 8192;
const _ISxdigit: c_uint = 4096;
const _ISdigit: c_uint = 2048;
const _ISalpha: c_uint = 1024;
const _ISlower: c_uint = 512;
const _ISupper: c_uint = 256;

type size_t = c_ulong;
type wchar_t = c_int;
type __off_t = c_long;
type __off64_t = c_long;

#[repr(C)]
struct _IO_FILE {
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
    _old_offset: __off_t,
    _cur_column: c_ushort,
    _vtable_offset: c_char,
    _shortbuf: [c_char; 1],
    _lock: *mut std::ffi::c_void,
    _offset: __off64_t,
    __pad1: *mut std::ffi::c_void,
    __pad2: *mut std::ffi::c_void,
    __pad3: *mut std::ffi::c_void,
    __pad4: *mut std::ffi::c_void,
    __pad5: size_t,
    _mode: c_int,
    _unused2: [c_char; 20],
}

struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: c_int,
}

type FILE = _IO_FILE;

// Node types and flags
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NODETYPE {
    Node_illegal = 0,
    Node_val = 1,
    Node_regex = 2,
    Node_dynregex = 3,
    Node_var = 4,
    Node_var_array = 5,
    Node_var_new = 6,
    Node_elem_new = 7,
    Node_param_list = 8,
    Node_func = 9,
    Node_ext_func = 10,
    Node_builtin_func = 11,
    Node_array_ref = 12,
    Node_array_tree = 13,
    Node_array_leaf = 14,
    Node_dump_array = 15,
    Node_arrayfor = 16,
    Node_frame = 17,
    Node_instruction = 18,
    Node_final = 19,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum flagvals {
    MALLOC = 1,
    STRING = 2,
    STRCUR = 4,
    NUMCUR = 8,
    NUMBER = 16,
    USER_INPUT = 32,
    BOOLVAL = 64,
    INTLSTR = 128,
    NUMINT = 256,
    INTIND = 512,
    WSTRCUR = 1024,
    MPFN = 2048,
    MPZN = 4096,
    NO_EXT_SET = 8192,
    NULL_FIELD = 16384,
    ARRAYMAXED = 32768,
    HALFHAT = 65536,
    XARRAY = 131072,
    NUMCONSTSTR = 262144,
    REGEX = 524288,
}

// Main Node structure
#[repr(C)]
#[derive(Debug)]
struct NODE {
    sub: NodeSub,
    type_: NODETYPE,
    flags: flagvals,
    valref: c_long,
}

#[repr(C)]
#[derive(Debug)]
union NodeSub {
    nodep: NodePtr,
    val: NodeVal,
}

#[repr(C)]
#[derive(Debug)]
struct NodePtr {
    l: NodeLeft,
    r: NodeRight,
    x: NodeExtra,
    name: *mut c_char,
    reserved: size_t,
    rn: *mut NODE,
    cnt: c_ulong,
    reflags: reflagvals,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum reflagvals {
    CONSTANT = 1,
    FS_DFLT = 2,
}

#[repr(C)]
#[derive(Debug)]
union NodeLeft {
    lptr: *mut NODE,
    li: *mut INSTRUCTION,
    ll: c_long,
    lp: *const array_funcs_t,
}

#[repr(C)]
#[derive(Debug)]
union NodeRight {
    rptr: *mut NODE,
    preg: [*mut Regexp; 2],
    av: *mut *mut NODE,
    bv: *mut *mut BUCKET,
    uptr: Option<unsafe extern "C" fn()>,
    iptr: *mut INSTRUCTION,
}

#[repr(C)]
#[derive(Debug)]
union NodeExtra {
    extra: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xl: c_long,
    cmnt: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Debug)]
struct NodeVal {
    fltnum: c_double,
    sp: *mut c_char,
    slen: size_t,
    idx: c_int,
    wsp: *mut wchar_t,
    wslen: size_t,
    typre: *mut NODE,
    comtype: commenttype,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum commenttype {
    EOL_COMMENT = 1,
    BLOCK_COMMENT = 2,
    FOR_COMMENT = 3,
}

// Instruction related types
#[repr(C)]
#[derive(Debug)]
struct INSTRUCTION {
    nexti: *mut INSTRUCTION,
    d: InstrData,
    x: InstrExtra,
    comment: *mut INSTRUCTION,
    source_line: c_short,
    pool_size: c_short,
    opcode: OPCODE,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OPCODE {
    Op_illegal = 0,
    // ... other opcodes ...
    Op_final = 122,
}

#[repr(C)]
#[derive(Debug)]
union InstrData {
    dn: *mut NODE,
    di: *mut INSTRUCTION,
    fptr: Option<unsafe extern "C" fn(c_int) -> *mut NODE>,
    efptr: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    dl: c_long,
    ldl: exec_count_t,
    name: *mut c_char,
}

type exec_count_t = u64;

#[repr(C)]
#[derive(Debug)]
union InstrExtra {
    xl: c_long,
    xn: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xi: *mut INSTRUCTION,
    bpt: *mut break_point,
    exf: *mut awk_ext_func_t,
}

// Other supporting types
#[repr(C)]
struct Regexp {
    pat: re_pattern_buffer,
    regs: re_registers,
    dfareg: *mut dfa,
    has_meta: bool,
    maybe_long: bool,
}

#[repr(C)]
struct re_pattern_buffer {
    buffer: *mut re_dfa_t,
    allocated: __re_long_size_t,
    used: __re_long_size_t,
    syntax: reg_syntax_t,
    fastmap: *mut c_char,
    translate: *mut c_uchar,
    re_nsub: size_t,
    can_be_null: u8,
    regs_allocated: u8,
    fastmap_accurate: u8,
    no_sub: u8,
    not_bol: u8,
    not_eol: u8,
    newline_anchor: u8,
}

type __re_size_t = c_uint;
type __re_long_size_t = c_ulong;
type reg_syntax_t = c_ulong;
type regoff_t = c_int;

#[repr(C)]
struct re_registers {
    num_regs: __re_size_t,
    start: *mut regoff_t,
    end: *mut regoff_t,
}

type dfa = std::ffi::c_void;
type break_point = std::ffi::c_void;

// Global variables
static mut IGNORECASE: bool = false;
static mut CONVFMT: *const c_char = null();
static mut CONVFMTidx: c_int = 0;
static mut SUBSEP_node: *mut NODE = null_mut();
static mut Nnull_string: *mut NODE = null_mut();
static mut Null_field: *mut NODE = null_mut();
static mut SUBSEPlen: size_t = 0;
static mut SUBSEP: *mut c_char = null_mut();
static mut indent_char: [c_char; 5] = [b' ' as c_char, b' ' as c_char, b' ' as c_char, b' ' as c_char, 0];

// Function declarations
extern "C" {
    fn __ctype_b_loc() -> *mut *const c_ushort;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn pma_free(ptr: *mut std::ffi::c_void);
    fn pma_realloc(ptr: *mut std::ffi::c_void, size: size_t) -> *mut std::ffi::c_void;
    fn pma_malloc(size: size_t) -> *mut std::ffi::c_void;
    fn qsort(base: *mut std::ffi::c_void, nmemb: size_t, size: size_t, compar: __compar_fn_t);
    fn dcgettext(domainname: *const c_char, msgid: *const c_char, category: c_int) -> *mut c_char;
    fn memcpy(dest: *mut std::ffi::c_void, src: *const std::ffi::c_void, n: size_t) -> *mut std::ffi::c_void;
    fn memset(s: *mut std::ffi::c_void, c: c_int, n: size_t) -> *mut std::ffi::c_void;
    fn memcmp(s1: *const std::ffi::c_void, s2: *const std::ffi::c_void, n: size_t) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
}

type __compar_fn_t = Option<unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void) -> c_int>;

// Safe wrapper functions
fn deref(r: *mut NODE) {
    unsafe {
        (*r).valref -= 1;
        if (*r).valref > 0 {
            return;
        }
        r_unref(r);
    }
}

fn r_unref(_tmp: *mut NODE) {
    // Implementation would free the node
    unimplemented!()
}

fn force_string_fmt(s: *mut NODE, fmtstr: *const c_char, fmtidx: c_int) -> *mut NODE {
    unsafe {
        if (*s).type_ == NODETYPE::Node_elem_new {
            (*s).type_ = NODETYPE::Node_val;
            (*s).flags = flagvals::from_bits_truncate((*s).flags as u32 & !(flagvals::NUMBER as u32));
            return s;
        }
        if (*s).flags as u32 & (flagvals::STRCUR as u32) != 0
            && ((*s).sub.val.idx == -1 || (*s).sub.val.idx == fmtidx)
        {
            return s;
        }
        format_val(fmtstr, fmtidx, s)
    }
}

fn format_val(_fmtstr: *const c_char, _fmtidx: c_int, _s: *mut NODE) -> *mut NODE {
    unimplemented!()
}

// More safe wrapper functions would follow...

// Main array functions
fn make_array() -> *mut NODE {
    unsafe {
        let array = nextfree[BLOCK_NODE as usize].freep as *mut NODE;
        if !array.is_null() {
            nextfree[BLOCK_NODE as usize].freep = (*(array as *mut block_item)).freep;
        } else {
            array = more_blocks(BLOCK_NODE as c_int) as *mut NODE;
        };
        memset(
            array as *mut std::ffi::c_void,
            0,
            std::mem::size_of::<NODE>() as size_t,
        );
        (*array).type_ = NODETYPE::Node_var_array;
        (*array).sub.nodep.l.lp = &null_array_func;
        array
    }
}

fn null_array(symbol: *mut NODE) {
    unsafe {
        (*symbol).type_ = NODETYPE::Node_var_array;
        (*symbol).sub.nodep.l.lp = &null_array_func;
        (*symbol).sub.nodep.r.bv = null_mut();
        (*symbol).sub.nodep.reflags = reflagvals::CONSTANT;
        (*symbol).sub.nodep.cnt = 0;
        (*symbol).sub.nodep.reserved = 0;
        (*symbol).flags = flagvals::from_bits_truncate(0);
    }
}

// More array functions would follow...

// Helper structs
#[repr(C)]
struct array_funcs_t {
    name: *const c_char,
    init: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    type_of: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    lookup: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    exists: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    clear: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    remove: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    list: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    copy: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    dump: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
    store: Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>,
}

static null_array_func: array_funcs_t = array_funcs_t {
    name: b"null\0" as *const u8 as *const c_char,
    init: None,
    type_of: None,
    lookup: Some(null_lookup),
    exists: Some(null_afunc),
    clear: Some(null_afunc),
    remove: Some(null_afunc),
    list: Some(null_afunc),
    copy: Some(null_afunc),
    dump: Some(null_dump),
    store: None,
};

// More implementation would follow...

// Note: This is a partial translation focusing on the core data structures and safety wrappers.
// A complete translation would need to handle all the functions and global state from the original C code.