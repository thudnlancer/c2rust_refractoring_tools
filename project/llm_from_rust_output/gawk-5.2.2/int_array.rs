use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ushort};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NODE {
    sub: SubNode,
    type_: NODETYPE,
    flags: flagvals,
    valref: c_long,
}

#[derive(Debug, Clone, Copy)]
union SubNode {
    nodep: NodePtr,
    val: NodeVal,
}

#[derive(Debug, Clone, Copy)]
struct NodePtr {
    l: LeftPtr,
    r: RightPtr,
    x: ExtraPtr,
    name: *mut c_char,
    reserved: size_t,
    rn: *mut NODE,
    cnt: c_ulong,
    reflags: reflagvals,
}

#[derive(Debug, Clone, Copy)]
union LeftPtr {
    lptr: *mut NODE,
    li: *mut exp_instruction,
    ll: c_long,
    lp: *const array_funcs_t,
}

#[derive(Debug, Clone, Copy)]
union RightPtr {
    rptr: *mut NODE,
    preg: [*mut Regexp; 2],
    av: *mut *mut NODE,
    bv: *mut *mut BUCKET,
    uptr: Option<unsafe extern "C" fn() -> ()>,
    iptr: *mut exp_instruction,
}

#[derive(Debug, Clone, Copy)]
union ExtraPtr {
    extra: *mut NODE,
    aptr: Option<unsafe extern "C" fn() -> ()>,
    xl: c_long,
    cmnt: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
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

type size_t = c_ulong;
type wchar_t = c_int;
type commenttype = c_uint;
type reflagvals = c_uint;
type flagvals = c_uint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    _padding: [u8; 7],
}

type __re_long_size_t = c_ulong;
type reg_syntax_t = c_ulong;

#[repr(C)]
struct re_registers {
    num_regs: __re_size_t,
    start: *mut regoff_t,
    end: *mut regoff_t,
}

type __re_size_t = c_uint;
type regoff_t = c_int;

#[repr(C)]
struct BUCKET {
    hs: StrBucket,
    hi: IntBucket,
}

#[derive(Debug, Clone, Copy)]
union StrBucket {
    next: *mut BUCKET,
    str_: *mut c_char,
    len: size_t,
    code: size_t,
    name: *mut NODE,
    val: *mut NODE,
}

#[derive(Debug, Clone, Copy)]
union IntBucket {
    next: *mut BUCKET,
    li: [c_long; 2],
    val: [*mut NODE; 2],
    cnt: size_t,
}

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

#[repr(C)]
struct exp_instruction {
    nexti: *mut exp_instruction,
    d: DataPtr,
    x: ExtraPtr,
    comment: *mut exp_instruction,
    source_line: c_short,
    pool_size: c_short,
    opcode: OPCODE,
}

#[derive(Debug, Clone, Copy)]
union DataPtr {
    dn: *mut NODE,
    di: *mut exp_instruction,
    fptr: Option<unsafe extern "C" fn(c_int) -> *mut NODE>,
    efptr: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    dl: c_long,
    ldl: exec_count_t,
    name: *mut c_char,
}

type exec_count_t = c_ulonglong;

#[repr(C)]
struct awk_value_t {
    val_type: awk_valtype_t,
    u: awk_value_union,
}

#[derive(Debug, Clone, Copy)]
union awk_value_union {
    s: awk_string_t,
    n: awk_number_t,
    a: awk_array_t,
    scl: awk_scalar_t,
    vc: awk_value_cookie_t,
    b: awk_bool_t,
}

type awk_array_t = *mut c_void;
type awk_scalar_t = *mut c_void;
type awk_value_cookie_t = *mut c_void;

#[repr(C)]
struct awk_ext_func {
    name: *const c_char,
    function: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: awk_bool_t,
    data: *mut c_void,
}

type awk_bool_t = awk_bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum awk_bool {
    False = 0,
    True = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[repr(C)]
struct awk_string_t {
    str_: *mut c_char,
    len: size_t,
}

#[repr(C)]
struct awk_number_t {
    d: c_double,
    type_: AWK_NUMBER_TYPE,
    ptr: *mut c_void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum AWK_NUMBER_TYPE {
    Double = 0,
    Mpfr = 1,
    Mpz = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Switch = 116,
    If = 117,
    Else = 118,
    Function = 119,
    CondExp = 120,
    Parens = 121,
    Final = 122,
}

// Safe Rust wrappers and implementations would go here
// Note: This is a direct translation of the C types and structures.
// Actual safe Rust implementations would need to:
// 1. Replace raw pointers with references or smart pointers
// 2. Implement proper memory management
// 3. Add error handling
// 4. Provide safe interfaces to the functionality
// 5. Handle null checks and pointer safety
// 6. Implement proper ownership semantics

// The rest of the implementation would need to be carefully rewritten
// to follow Rust's safety guarantees while maintaining the same functionality.