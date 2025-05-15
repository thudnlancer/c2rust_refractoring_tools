use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_double, c_void};
use std::ptr::{null_mut, null};
use std::mem::{size_of, zeroed};
use std::cmp::Ordering;
use std::collections::HashMap;

type size_t = c_ulong;
type awk_bool = c_uint;
type awk_valtype_t = c_uint;
type NODETYPE = c_uint;
type flagvals = c_uint;
type reflagvals = c_uint;
type assoc_kind_t = c_uint;
type block_id = c_uint;

const AWK_NUMBER_TYPE_DOUBLE: awk_valtype_t = 0;
const AWK_NUMBER_TYPE_MPFR: awk_valtype_t = 1;
const AWK_NUMBER_TYPE_MPZ: awk_valtype_t = 2;
const AWK_UNDEFINED: awk_valtype_t = 0;
const AWK_NUMBER: awk_valtype_t = 1;
const AWK_STRING: awk_valtype_t = 2;
const AWK_REGEX: awk_valtype_t = 3;
const AWK_STRNUM: awk_valtype_t = 4;
const AWK_ARRAY: awk_valtype_t = 5;
const AWK_SCALAR: awk_valtype_t = 6;
const AWK_VALUE_COOKIE: awk_valtype_t = 7;
const AWK_BOOL: awk_valtype_t = 8;

const MALLOC: flagvals = 1;
const STRING: flagvals = 2;
const STRCUR: flagvals = 4;
const NUMCUR: flagvals = 8;
const NUMBER: flagvals = 16;
const USER_INPUT: flagvals = 32;
const BOOLVAL: flagvals = 64;
const INTLSTR: flagvals = 128;
const NUMINT: flagvals = 256;
const INTIND: flagvals = 512;
const WSTRCUR: flagvals = 1024;
const MPFN: flagvals = 2048;
const MPZN: flagvals = 4096;
const NO_EXT_SET: flagvals = 8192;
const NULL_FIELD: flagvals = 16384;
const ARRAYMAXED: flagvals = 32768;
const HALFHAT: flagvals = 65536;
const XARRAY: flagvals = 131072;
const NUMCONSTSTR: flagvals = 262144;
const REGEX: flagvals = 524288;

const ANONE: assoc_kind_t = 0;
const AINDEX: assoc_kind_t = 1;
const AVALUE: assoc_kind_t = 2;
const AINUM: assoc_kind_t = 4;
const AISTR: assoc_kind_t = 8;
const AVNUM: assoc_kind_t = 16;
const AVSTR: assoc_kind_t = 32;
const AASC: assoc_kind_t = 64;
const ADESC: assoc_kind_t = 128;
const ADELETE: assoc_kind_t = 256;

const BLOCK_NODE: block_id = 0;
const BLOCK_BUCKET: block_id = 1;
const BLOCK_MAX: block_id = 2;

const Node_illegal: NODETYPE = 0;
const Node_val: NODETYPE = 1;
const Node_regex: NODETYPE = 2;
const Node_dynregex: NODETYPE = 3;
const Node_var: NODETYPE = 4;
const Node_var_array: NODETYPE = 5;
const Node_var_new: NODETYPE = 6;
const Node_elem_new: NODETYPE = 7;
const Node_param_list: NODETYPE = 8;
const Node_func: NODETYPE = 9;
const Node_ext_func: NODETYPE = 10;
const Node_builtin_func: NODETYPE = 11;
const Node_array_ref: NODETYPE = 12;
const Node_array_tree: NODETYPE = 13;
const Node_array_leaf: NODETYPE = 14;
const Node_dump_array: NODETYPE = 15;
const Node_arrayfor: NODETYPE = 16;
const Node_frame: NODETYPE = 17;
const Node_instruction: NODETYPE = 18;
const Node_final: NODETYPE = 19;

const CONSTANT: reflagvals = 1;
const FS_DFLT: reflagvals = 2;

struct NODE {
    sub: NodeSub,
    type_: NODETYPE,
    flags: flagvals,
    valref: c_long,
}

union NodeSub {
    val: NodeVal,
    nodep: NodePtr,
}

struct NodeVal {
    fltnum: c_double,
    sp: *mut c_char,
    slen: size_t,
    idx: c_int,
    wsp: *mut c_int,
    wslen: size_t,
    typre: *mut NODE,
    comtype: c_uint,
}

struct NodePtr {
    l: NodeL,
    r: NodeR,
    x: NodeX,
    name: *mut c_char,
    reserved: size_t,
    rn: *mut NODE,
    cnt: c_ulong,
    reflags: reflagvals,
}

union NodeL {
    lptr: *mut NODE,
    li: *mut exp_instruction,
    ll: c_long,
    lp: *const array_funcs_t,
}

union NodeR {
    rptr: *mut NODE,
    preg: [*mut Regexp; 2],
    av: *mut *mut NODE,
    bv: *mut *mut BUCKET,
    uptr: Option<unsafe extern "C" fn()>,
    iptr: *mut exp_instruction,
}

union NodeX {
    extra: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xl: c_long,
    cmnt: *mut c_void,
}

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

struct Regexp {
    pat: re_pattern_buffer,
    regs: re_registers,
    dfareg: *mut dfa,
    has_meta: bool,
    maybe_long: bool,
}

struct re_pattern_buffer {
    buffer: *mut re_dfa_t,
    allocated: c_ulong,
    used: c_ulong,
    syntax: c_ulong,
    fastmap: *mut c_char,
    translate: *mut c_uchar,
    re_nsub: size_t,
    flags: u8,
    _padding: [u8; 7],
}

struct re_registers {
    num_regs: c_uint,
    start: *mut c_int,
    end: *mut c_int,
}

struct BUCKET {
    data: BucketData,
}

union BucketData {
    hs: BucketHS,
    hi: BucketHI,
}

struct BucketHS {
    next: *mut BUCKET,
    str_: *mut c_char,
    len: size_t,
    code: size_t,
    name: *mut NODE,
    val: *mut NODE,
}

struct BucketHI {
    next: *mut BUCKET,
    li: [c_long; 2],
    val: [*mut NODE; 2],
    cnt: size_t,
}

struct exp_instruction {
    nexti: *mut exp_instruction,
    d: InstrD,
    x: InstrX,
    comment: *mut exp_instruction,
    source_line: c_short,
    pool_size: c_short,
    opcode: OPCODE,
}

union InstrD {
    dn: *mut NODE,
    di: *mut exp_instruction,
    fptr: Option<unsafe extern "C" fn(c_int) -> *mut NODE>,
    efptr: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func) -> *mut awk_value_t>,
    dl: c_long,
    ldl: c_ulonglong,
    name: *mut c_char,
}

union InstrX {
    xl: c_long,
    xn: *mut NODE,
    aptr: Option<unsafe extern "C" fn()>,
    xi: *mut exp_instruction,
    bpt: *mut break_point,
    exf: *mut awk_ext_func_t,
}

struct awk_value_t {
    val_type: awk_valtype_t,
    u: awk_value_union,
}

union awk_value_union {
    s: awk_string_t,
    n: awk_number_t,
    a: awk_array_t,
    scl: awk_scalar_t,
    vc: awk_value_cookie_t,
    b: awk_bool_t,
}

struct awk_string_t {
    str_: *mut c_char,
    len: size_t,
}

struct awk_number_t {
    d: c_double,
    type_: AWK_NUMBER_TYPE,
    ptr: *mut c_void,
}

struct awk_ext_func_t {
    name: *const c_char,
    function: Option<unsafe extern "C" fn(c_int, *mut awk_value_t, *mut awk_ext_func_t) -> *mut awk_value_t>,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: awk_bool_t,
    data: *mut c_void,
}

struct block_header {
    freep: *mut block_item,
    size: size_t,
    name: *const c_char,
    highwater: c_long,
}

struct block_item {
    freep: *mut block_item,
}

static mut NHAT: c_int = 10;
static mut THRESHOLD: c_long = 0;
static mut argv_shadow_array: *mut NODE = null_mut();

static mut cint_array_func: array_funcs_t = array_funcs_t {
    name: b"cint\0".as_ptr() as *const c_char,
    init: Some(cint_array_init),
    type_of: Some(is_uinteger),
    lookup: Some(cint_lookup),
    exists: Some(cint_exists),
    clear: Some(cint_clear),
    remove: Some(cint_remove),
    list: Some(cint_list),
    copy: Some(cint_copy),
    dump: Some(cint_dump),
    store: None,
};

static mut argv_array_func: array_funcs_t = array_funcs_t {
    name: b"argv\0".as_ptr() as *const c_char,
    init: Some(cint_array_init),
    type_of: Some(is_uinteger),
    lookup: Some(cint_lookup),
    exists: Some(cint_exists),
    clear: Some(cint_clear),
    remove: Some(cint_remove),
    list: Some(cint_list),
    copy: Some(cint_copy),
    dump: Some(cint_dump),
    store: Some(argv_store),
};

static power_two_table: [c_long; 31] = [
    1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
    131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
    67108864, 134217728, 268435456, 536870912, 1073741824,
];

unsafe fn cint_array_init(symbol: *mut NODE, subs: *mut NODE) -> *mut *mut NODE {
    if symbol.is_null() {
        let newval = getenv_long(b"NHAT\0".as_ptr() as *const c_char);
        if newval > 1 && newval < 32 {
            NHAT = newval as c_int;
        }
        let nelems = size_of::<[c_long; 31]>() / size_of::<c_long>();
        if NHAT as size_t > nelems - 2 {
            NHAT = (nelems - 2) as c_int;
        }
        THRESHOLD = power_two_table[(NHAT + 1) as usize];
    } else {
        null_array(symbol);
    }
    &mut success_node
}

unsafe fn is_uinteger(symbol: *mut NODE, subs: *mut NODE) -> *mut *mut NODE {
    if !is_integer(symbol, subs).is_null() && (*subs).sub.val.fltnum >= 0.0 {
        &mut success_node
    } else {
        null_mut()
    }
}

unsafe fn cint_lookup(symbol: *mut NODE, subs: *mut NODE) -> *mut *mut NODE {
    let mut k = -1;
    let mut h1 = -1;
    let mut tn: *mut NODE = null_mut();
    let mut xn: *mut NODE = null_mut();
    
    if ((*subs).flags & NUMINT != 0 || !is_integer(symbol, subs).is_null()) 
        && (*subs).sub.val.fltnum >= 0.0 
    {
        k = (*subs).sub.val.fltnum as c_long;
        h1 = cint_hash(k);
        let lhs = cint_find(symbol, k, h1);
        if !lhs.is_null() {
            return lhs;
        }
    }
    
    xn = (*symbol).sub.nodep.rn;
    if !xn.is_null() {
        let lhs = ((*(*xn).sub.nodep.l.lp).exists.unwrap())(xn, subs);
        if !lhs.is_null() {
            return lhs;
        }
    }
    
    if k >= 0 {
        let m = h1 - 1;
        let li = if m > NHAT { m } else { NHAT };
        let capacity = ((*symbol).sub.nodep.reserved + power_two_table[li as usize] as size_t) as c_long;
        let cint_size = if xn.is_null() {
            (*symbol).sub.nodep.reflags as c_uint
        } else {
            (*symbol).sub.nodep.reflags as c_uint - (*xn).sub.nodep.reflags as c_uint
        } as c_long;
        
        if capacity - cint_size > THRESHOLD {
            if (*symbol).sub.nodep.r.av.is_null() {
                (*symbol).sub.nodep.reserved = 0;
                (*symbol).sub.nodep.r.av = ezalloc_real(
                    32 * size_of::<*mut NODE>(),
                    b"cint_lookup\0".as_ptr() as *const c_char,
                    b"symbol->nodes\0".as_ptr() as *const c_char,
                    b"cint_array.c\0".as_ptr() as *const c_char,
                    249,
                ) as *mut *mut NODE;
            }
            (*symbol).sub.nodep.reflags += 1;
            tn = *((*symbol).sub.nodep.r.av).offset(h1 as isize);
            if tn.is_null() {
                tn = make_node(Node_array_tree);
                *((*symbol).sub.nodep.r.av).offset(h1 as isize) = tn;
            }
            if m < NHAT {
                return tree_lookup(symbol, tn, k, NHAT, 0);
            }
            return tree_lookup(symbol, tn, k, m, power_two_table[m as usize]);
        }
    }
    
    (*symbol).sub.nodep.reflags += 1;
    if xn.is_null() {
        (*symbol).sub.nodep.rn = make_array();
        xn = (*symbol).sub.nodep.rn;
        (*xn).sub.nodep.name = (*symbol).sub.nodep.name;
        if !is_integer(xn, subs).is_null() {
            (*xn).sub.nodep.l.lp = &int_array_func;
        } else {
            (*xn).sub.nodep.l.lp = &str_array_func;
        }
        (*xn).flags |= XARRAY;
    }
    ((*(*xn).sub.nodep.l.lp).lookup.unwrap())(xn, subs)
}

// 其他函数类似地进行安全封装...

#[no_mangle]
pub unsafe extern "C" fn init_argv_array(argv_node: *mut NODE, shadow_node: *mut NODE) {
    if do_flags & DO_SANDBOX == 0 {
        return;
    }
    (*argv_node).sub.nodep.l.lp = &argv_array_func;
    argv_shadow_array = shadow_node;
}