use libc::{c_char, c_double, c_int, c_uint, c_void, size_t};
use std::ptr;
use std::mem;
use std::ffi::CString;
use std::os::raw::c_uchar;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NODE {
    sub: SubNode,
    type_: NODETYPE,
    flags: flagvals,
    valref: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SubNode {
    nodep: NodePtr,
    val: NodeVal,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
struct NodePtr {
    l: LeftPtr,
    r: RightPtr,
    x: ExtraPtr,
    name: *mut c_char,
    reserved: size_t,
    rn: *mut NODE,
    cnt: u64,
    reflags: reflagvals,
}

#[repr(C)]
#[derive(Copy, Clone)]
union LeftPtr {
    lptr: *mut NODE,
    li: *mut exp_instruction,
    ll: i64,
    lp: *const array_funcs_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
union RightPtr {
    rptr: *mut NODE,
    preg: [*mut Regexp; 2],
    av: *mut *mut NODE,
    bv: *mut *mut BUCKET,
    uptr: Option<unsafe extern "C" fn()>,
    iptr: *mut exp_instruction,
}

#[repr(C)]
#[derive(Copy, Clone)]
union ExtraPtr {
    extra: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xl: i64,
    cmnt: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct exp_instruction {
    nexti: *mut exp_instruction,
    d: DataPtr,
    x: XPtr,
    comment: *mut exp_instruction,
    source_line: i16,
    pool_size: i16,
    opcode: OPCODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
union DataPtr {
    dn: *mut NODE,
    di: *mut exp_instruction,
    fptr: Option<unsafe extern "C" fn(c_int) -> *mut NODE>,
    efptr: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    dl: i64,
    ldl: u64,
    name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
union XPtr {
    xl: i64,
    xn: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xi: *mut exp_instruction,
    bpt: *mut break_point,
    exf: *mut awk_ext_func_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Regexp {
    pat: re_pattern_buffer,
    regs: re_registers,
    dfareg: *mut dfa,
    has_meta: bool,
    maybe_long: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct re_pattern_buffer {
    buffer: *mut re_dfa_t,
    allocated: u64,
    used: u64,
    syntax: u64,
    fastmap: *mut c_char,
    translate: *mut c_uchar,
    re_nsub: size_t,
    flags: [u8; 1],
    _padding: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct re_registers {
    num_regs: u32,
    start: *mut i32,
    end: *mut i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct awk_value {
    val_type: awk_valtype_t,
    u: awk_value_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
union awk_value_union {
    s: awk_string_t,
    n: awk_number_t,
    a: awk_array_t,
    scl: awk_scalar_t,
    vc: awk_value_cookie_t,
    b: awk_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct awk_string_t {
    str_: *mut c_char,
    len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct awk_number_t {
    d: c_double,
    type_: AWK_NUMBER_TYPE,
    ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct awk_ext_func {
    name: *const c_char,
    function: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: awk_bool_t,
    data: *mut c_void,
}

type awk_array_t = *mut c_void;
type awk_scalar_t = *mut c_void;
type awk_value_cookie_t = *mut c_void;
type wchar_t = c_int;
type wint_t = c_uint;
type awk_bool_t = c_uint;

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum NODETYPE {
    Illegal = 0,
    Val = 1,
    Regex = 2,
    Dynregex = 3,
    Var = 4,
    VarArray = 5,
    VarNew = 6,
    ElemNew = 7,
    ParamList = 8,
    Func = 9,
    ExtFunc = 10,
    BuiltinFunc = 11,
    ArrayRef = 12,
    ArrayTree = 13,
    ArrayLeaf = 14,
    DumpArray = 15,
    Arrayfor = 16,
    Frame = 17,
    Instruction = 18,
    Final = 19,
}

bitflags! {
    #[repr(C)]
    struct flagvals: c_uint {
        const MALLOC = 1;
        const STRING = 2;
        const STRCUR = 4;
        const NUMCUR = 8;
        const NUMBER = 16;
        const USER_INPUT = 32;
        const BOOLVAL = 64;
        const INTLSTR = 128;
        const NUMINT = 256;
        const INTIND = 512;
        const WSTRCUR = 1024;
        const MPFN = 2048;
        const MPZN = 4096;
        const NO_EXT_SET = 8192;
        const NULL_FIELD = 16384;
        const ARRAYMAXED = 32768;
        const HALFHAT = 65536;
        const XARRAY = 131072;
        const NUMCONSTSTR = 262144;
        const REGEX = 524288;
    }
}

bitflags! {
    #[repr(C)]
    struct reflagvals: c_uint {
        const CONSTANT = 1;
        const FS_DFLT = 2;
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum AWK_NUMBER_TYPE {
    Double = 0,
    MPFR = 1,
    MPZ = 2,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum awk_valtype_t {
    Undefined = 0,
    Number = 1,
    String = 2,
    Regex = 3,
    Strnum = 4,
    Array = 5,
    Scalar = 6,
    ValueCookie = 7,
    Bool = 8,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum commenttype {
    EolComment = 1,
    BlockComment = 2,
    ForComment = 3,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u32)]
enum OPCODE {
    Illegal = 0,
    Times = 1,
    TimesI = 2,
    Quotient = 3,
    QuotientI = 4,
    Mod = 5,
    ModI = 6,
    Plus = 7,
    PlusI = 8,
    Minus = 9,
    MinusI = 10,
    Exp = 11,
    ExpI = 12,
    Concat = 13,
    LineRange = 14,
    CondPair = 15,
    Subscript = 16,
    SubArray = 17,
    Preincrement = 18,
    Predecrement = 19,
    Postincrement = 20,
    Postdecrement = 21,
    UnaryMinus = 22,
    UnaryPlus = 23,
    FieldSpec = 24,
    Not = 25,
    Assign = 26,
    StoreVar = 27,
    StoreSub = 28,
    StoreField = 29,
    StoreFieldExp = 30,
    AssignTimes = 31,
    AssignQuotient = 32,
    AssignMod = 33,
    AssignPlus = 34,
    AssignMinus = 35,
    AssignExp = 36,
    AssignConcat = 37,
    And = 38,
    AndFinal = 39,
    Or = 40,
    OrFinal = 41,
    Equal = 42,
    Notequal = 43,
    Less = 44,
    Greater = 45,
    Leq = 46,
    Geq = 47,
    Match = 48,
    MatchRec = 49,
    Nomatch = 50,
    Rule = 51,
    Case = 52,
    Default = 53,
    Break = 54,
    Continue = 55,
    Print = 56,
    PrintRec = 57,
    Printf = 58,
    Next = 59,
    Exit = 60,
    Return = 61,
    ReturnFromEval = 62,
    Delete = 63,
    DeleteLoop = 64,
    GetlineRedir = 65,
    Getline = 66,
    Nextfile = 67,
    Namespace = 68,
    Builtin = 69,
    SubBuiltin = 70,
    ExtBuiltin = 71,
    InArray = 72,
    FuncCall = 73,
    IndirectFuncCall = 74,
    Push = 75,
    PushArg = 76,
    PushArgUntyped = 77,
    PushI = 78,
    PushRe = 79,
    PushArray = 80,
    PushParam = 81,
    PushLhs = 82,
    SubscriptLhs = 83,
    FieldSpecLhs = 84,
    NoOp = 85,
    Pop = 86,
    Jmp = 87,
    JmpTrue = 88,
    JmpFalse = 89,
    GetRecord = 90,
    Newfile = 91,
    ArrayforInit = 92,
    ArrayforIncr = 93,
    ArrayforFinal = 94,
    VarUpdate = 95,
    VarAssign = 96,
    FieldAssign = 97,
    SubscriptAssign = 98,
    AfterBeginfile = 99,
    AfterEndfile = 100,
    Func = 101,
    Comment = 102,
    ExecCount = 103,
    Breakpoint = 104,
    Lint = 105,
    LintPlus = 106,
    Atexit = 107,
    Stop = 108,
    Token = 109,
    Symbol = 110,
    List = 111,
    Do = 112,
    For = 113,
    Arrayfor = 114,
    While = 115,
    If = 116,
    Else = 117,
    Function = 118,
    KFunction = 119,
    CondExp = 120,
    Parens = 121,
    Final = 122,
}

#[repr(C)]
struct block_header {
    freep: *mut block_item,
    size: size_t,
    name: *const c_char,
    highwater: i64,
}

#[repr(C)]
struct block_item {
    freep: *mut block_item,
}

#[repr(C)]
struct array_funcs_t {
    name: *const c_char,
    init: afunc_t,
    type_of: afunc_t,
    lookup: afunc_t,
    exists: afunc_t,
    clear: afunc_t,
    remove: afunc_t,
    list: afunc_t,
    copy: afunc_t,
    dump: afunc_t,
    store: afunc_t,
}

type afunc_t = Option<unsafe extern "C" fn(*mut NODE, *mut NODE) -> *mut *mut NODE>;

#[repr(C)]
struct BUCKET {
    next: *mut BUCKET,
    str_: *mut c_char,
    len: size_t,
    code: size_t,
    name: *mut NODE,
    val: *mut NODE,
}

static mut Nnull_string: *mut NODE = ptr::null_mut();
static mut Null_field: *mut NODE = ptr::null_mut();
static mut fmt_list: *mut *mut NODE = ptr::null_mut();
static mut btowc_cache: [wint_t; 256] = [0; 256];
static mut nextfree: [block_header; 2] = [
    block_header {
        freep: ptr::null_mut(),
        size: mem::size_of::<NODE>() as size_t,
        name: b"node\0" as *const u8 as *const c_char,
        highwater: 0,
    },
    block_header {
        freep: ptr::null_mut(),
        size: mem::size_of::<BUCKET>() as size_t,
        name: b"bucket\0" as *const u8 as *const c_char,
        highwater: 0,
    },
];

pub unsafe extern "C" fn r_make_number(x: c_double) -> *mut NODE {
    let r = make_number_node(0);
    (*r).sub.val.fltnum = x;
    r
}

pub unsafe extern "C" fn make_number_node(flags: c_uint) -> *mut NODE {
    let mut r = nextfree[0].freep as *mut NODE;
    if !r.is_null() {
        nextfree[0].freep = (*(r as *mut block_item)).freep;
    } else {
        r = more_blocks(0) as *mut NODE;
    }
    ptr::write_bytes(r as *mut u8, 0, mem::size_of::<NODE>());
    (*r).type_ = NODETYPE::Val;
    (*r).valref = 1;
    (*r).flags = flagvals::MALLOC | flagvals::NUMBER | flagvals::NUMCUR;
    r
}

pub unsafe extern "C" fn more_blocks(id: c_int) -> *mut c_void {
    let size = nextfree[id as usize].size;
    let freep = emalloc_real(
        100 * size,
        b"more_blocks\0".as_ptr() as *const c_char,
        b"freep\0".as_ptr() as *const c_char,
        b"node.c\0".as_ptr() as *const c_char,
        1075,
    ) as *mut block_item;
    
    let mut p = freep as *mut c_char;
    let endp = p.add(100 * size);
    let mut np = freep;
    
    loop {
        p = p.add(size);
        let next = p as *mut block_item;
        if p >= endp {
            (*np).freep = ptr::null_mut();
            break;
        } else {
            (*np).freep = next;
            np = next;
        }
    }
    
    nextfree[id as usize].freep = (*freep).freep;
    nextfree[id as usize].highwater += 100;
    freep as *mut c_void
}

pub unsafe extern "C" fn emalloc_real(
    count: size_t,
    where_: *const c_char,
    var: *const c_char,
    file: *const c_char,
    line: c_int,
) -> *mut c_void {
    if count == 0 {
        set_loc(b"./awk.h\0".as_ptr() as *const c_char, 2052);
        r_fatal(
            b"%s:%d: emalloc called with zero bytes\0".as_ptr() as *const c_char,
            file,
            line,
        );
    }
    
    let ret = pma_malloc(count);
    if ret.is_null() {
        set_loc(b"./awk.h\0".as_ptr() as *const c_char, 2056);
        r_fatal(
            dcgettext(
                ptr::null(),
                b"%s:%d:%s: %s: cannot allocate %ld bytes of memory: %s\0".as_ptr() as *const c_char,
                5,
            ),
            file,
            line,
            where_,
            var,
            count as i64,
            strerror(*__errno_location()),
        );
    }
    ret
}

pub unsafe extern "C" fn r_fatal(mesg: *const c_char, ...) {
    // Implementation would use va_list
}

pub unsafe extern "C" fn set_loc(file: *const c_char, line: c_int) {
    // Implementation would set location
}

pub unsafe extern "C" fn dcgettext(
    domainname: *const c_char,
    msgid: *const c_char,
    category: c_int,
) -> *mut c_char {
    // Implementation would call gettext
    ptr::null_mut()
}

pub unsafe extern "C" fn __errno_location() -> *mut c_int {
    // Implementation would return errno location
    ptr::null_mut()
}

pub unsafe extern "C" fn strerror(errnum: c_int) -> *mut c_char {
    // Implementation would return error string
    ptr::null_mut()
}

pub unsafe extern "C" fn pma_malloc(size: size_t) -> *mut c_void {
    // Implementation would allocate memory
    ptr::null_m