use std::f64;
use std::mem;
use std::ptr;
use std::cmp;
use std::os::raw::{c_char, c_int, c_long, c_double};
use std::ffi::{CString, CStr};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type AWKNUM = f64;
type NODETYPE = u32;
type OPCODE = u32;
type Func_ptr = fn();

#[derive(Debug, Clone)]
struct NODE {
    type_: NODETYPE,
    flags: u32,
    valref: i32,
    stptr: *mut c_char,
    stlen: usize,
    numbr: AWKNUM,
    vname: *mut c_char,
    var_value: *mut NODE,
    param_cnt: i32,
    for_list: *mut *mut NODE,
    for_list_size: usize,
    func_node: *mut NODE,
    code_ptr: *mut INSTRUCTION,
    reti: *mut INSTRUCTION,
    stack: *mut *mut NODE,
    prev_frame_size: i32,
    prev_array: *mut NODE,
    orig_array: *mut NODE,
}

#[derive(Debug)]
struct INSTRUCTION {
    opcode: OPCODE,
    func_body: *mut NODE,
    expr_count: i32,
    nexti: *mut INSTRUCTION,
}

#[derive(Debug)]
struct STACK_ITEM {
    node: *mut NODE,
}

static mut fcall_list: *mut *mut NODE = ptr::null_mut();
static mut fcall_count: c_long = 0;
static mut currule: c_int = 0;
static mut curfile: *mut IOBUF = ptr::null_mut();
static mut exiting: bool = false;
static mut interpret: Option<fn(*mut INSTRUCTION) -> c_int> = None;
const MAX_EXEC_HOOKS: usize = 10;
static mut num_exec_hook: c_int = 0;
static mut pre_execute: [Option<Func_ptr>; MAX_EXEC_HOOKS] = [None; MAX_EXEC_HOOKS];
static mut post_execute: Option<Func_ptr> = None;
static mut OFSlen: c_int = 0;
static mut ORSlen: c_int = 0;
static mut OFMTidx: c_int = 0;
static mut CONVFMTidx: c_int = 0;
static mut node_Boolean: [*mut NODE; 2] = [ptr::null_mut(); 2];
static mut frame_ptr: *mut NODE = ptr::null_mut();
static mut stack_ptr: *mut STACK_ITEM = ptr::null_mut();
static mut stack_bottom: *mut STACK_ITEM = ptr::null_mut();
static mut stack_top: *mut STACK_ITEM = ptr::null_mut();
static mut STACK_SIZE: u32 = 256;
static mut max_args: c_int = 0;
static mut args_array: *mut *mut NODE = ptr::null_mut();
static mut fmt_list: *mut *mut NODE = ptr::null_mut();

#[derive(Debug)]
struct IOBUF;

#[derive(Debug)]
struct EXEC_STATE {
    next: *mut EXEC_STATE,
    cptr: *mut INSTRUCTION,
    rule: c_int,
    stack_size: c_long,
    source: *const c_char,
}

static mut exec_state_stack: EXEC_STATE = EXEC_STATE {
    next: ptr::null_mut(),
    cptr: ptr::null_mut(),
    rule: 0,
    stack_size: 0,
    source: ptr::null(),
};

#[inline]
unsafe fn DEREF(n: *mut NODE) {
    if !n.is_null() {
        (*n).valref -= 1;
        if (*n).valref == 0 {
            // Free node memory
        }
    }
}

#[inline]
unsafe fn PUSH(n: *mut NODE) {
    if stack_ptr >= stack_top {
        grow_stack();
    }
    stack_ptr = stack_ptr.add(1);
    (*stack_ptr).node = n;
}

#[inline]
unsafe fn POP() -> *mut NODE {
    if stack_ptr < stack_bottom {
        ptr::null_mut()
    } else {
        let n = (*stack_ptr).node;
        stack_ptr = stack_ptr.sub(1);
        n
    }
}

#[inline]
unsafe fn TOP() -> *mut NODE {
    if stack_ptr < stack_bottom {
        ptr::null_mut()
    } else {
        (*stack_ptr).node
    }
}

#[inline]
unsafe fn stack_empty() -> bool {
    stack_ptr < stack_bottom
}

#[inline]
unsafe fn decr_sp() {
    stack_ptr = stack_ptr.sub(1);
}

#[inline]
unsafe fn force_string(n: *mut NODE) -> *mut NODE {
    if !n.is_null() && ((*n).flags & STRING) == 0 {
        // Convert to string
    }
    n
}

#[inline]
unsafe fn force_number(n: *mut NODE) -> *mut NODE {
    if !n.is_null() && ((*n).flags & NUMBER) == 0 {
        // Convert to number
    }
    n
}

#[inline]
unsafe fn fixtype(n: *mut NODE) -> *mut NODE {
    if !n.is_null() {
        if ((*n).flags & (STRING|NUMBER)) == (STRING|NUMBER) {
            if (*n).numbr == (*n).numbr { // Check for NaN
                (*n).flags &= !STRING;
            } else if (*n).stlen == 0 {
                (*n).flags &= !NUMBER;
            }
        }
    }
    n
}

#[inline]
unsafe fn make_number(num: AWKNUM) -> *mut NODE {
    let n: *mut NODE = Box::into_raw(Box::new(NODE {
        type_: Node_val,
        flags: MALLOC | NUMCUR | NUMBER,
        valref: 1,
        stptr: ptr::null_mut(),
        stlen: 0,
        numbr: num,
        vname: ptr::null_mut(),
        var_value: ptr::null_mut(),
        param_cnt: 0,
        for_list: ptr::null_mut(),
        for_list_size: 0,
        func_node: ptr::null_mut(),
        code_ptr: ptr::null_mut(),
        reti: ptr::null_mut(),
        stack: ptr::null_mut(),
        prev_frame_size: 0,
        prev_array: ptr::null_mut(),
        orig_array: ptr::null_mut(),
    }));
    n
}

#[inline]
unsafe fn make_string(s: *const c_char, len: usize) -> *mut NODE {
    let n: *mut NODE = Box::into_raw(Box::new(NODE {
        type_: Node_val,
        flags: MALLOC | STRCUR | STRING,
        valref: 1,
        stptr: s as *mut c_char,
        stlen: len,
        numbr: 0.0,
        vname: ptr::null_mut(),
        var_value: ptr::null_mut(),
        param_cnt: 0,
        for_list: ptr::null_mut(),
        for_list_size: 0,
        func_node: ptr::null_mut(),
        code_ptr: ptr::null_mut(),
        reti: ptr::null_mut(),
        stack: ptr::null_mut(),
        prev_frame_size: 0,
        prev_array: ptr::null_mut(),
        orig_array: ptr::null_mut(),
    }));
    n
}

#[inline]
unsafe fn dupnode(n: *mut NODE) -> *mut NODE {
    if !n.is_null() {
        (*n).valref += 1;
    }
    n
}

#[inline]
unsafe fn unref(n: *mut NODE) {
    if !n.is_null() {
        (*n).valref -= 1;
        if (*n).valref == 0 {
            // Free node memory
        }
    }
}

#[inline]
unsafe fn get_number_si(n: *mut NODE) -> c_long {
    force_number(n);
    (*n).numbr as c_long
}

#[inline]
unsafe fn boolval(n: *mut NODE) -> bool {
    if n.is_null() {
        false
    } else {
        fixtype(n);
        if ((*n).flags & NUMBER) != 0 {
            (*n).numbr != 0.0
        } else {
            (*n).stlen > 0
        }
    }
}

const MALLOC: u32 = 0x0001;
const STRING: u32 = 0x0002;
const STRCUR: u32 = 0x0004;
const NUMCUR: u32 = 0x0008;
const NUMBER: u32 = 0x0010;
const USER_INPUT: u32 = 0x0020;
const BOOLVAL: u32 = 0x0040;
const INTLSTR: u32 = 0x0080;
const NUMINT: u32 = 0x0100;
const INTIND: u32 = 0x0200;
const WSTRCUR: u32 = 0x0400;
const MPFN: u32 = 0x0800;
const MPZN: u32 = 0x1000;
const NO_EXT_SET: u32 = 0x2000;
const NULL_FIELD: u32 = 0x4000;
const ARRAYMAXED: u32 = 0x8000;
const HALFHAT: u32 = 0x10000;
const XARRAY: u32 = 0x20000;
const NUMCONSTSTR: u32 = 0x40000;
const REGEX: u32 = 0x80000;

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

const Op_illegal: OPCODE = 0;
const Op_times: OPCODE = 1;
const Op_times_i: OPCODE = 2;
const Op_quotient: OPCODE = 3;
const Op_quotient_i: OPCODE = 4;
const Op_mod: OPCODE = 5;
const Op_mod_i: OPCODE = 6;
const Op_plus: OPCODE = 7;
const Op_plus_i: OPCODE = 8;
const Op_minus: OPCODE = 9;
const Op_minus_i: OPCODE = 10;
const Op_exp: OPCODE = 11;
const Op_exp_i: OPCODE = 12;
const Op_concat: OPCODE = 13;
const Op_line_range: OPCODE = 14;
const Op_cond_pair: OPCODE = 15;
const Op_subscript: OPCODE = 16;
const Op_sub_array: OPCODE = 17;
const Op_preincrement: OPCODE = 18;
const Op_predecrement: OPCODE = 19;
const Op_postincrement: OPCODE = 20;
const Op_postdecrement: OPCODE = 21;
const Op_unary_minus: OPCODE = 22;
const Op_unary_plus: OPCODE = 23;
const Op_field_spec: OPCODE = 24;
const Op_not: OPCODE = 25;
const Op_assign: OPCODE = 26;
const Op_store_var: OPCODE = 27;
const Op_store_sub: OPCODE = 28;
const Op_store_field: OPCODE = 29;
const Op_store_field_exp: OPCODE = 30;
const Op_assign_times: OPCODE = 31;
const Op_assign_quotient: OPCODE = 32;
const Op_assign_mod: OPCODE = 33;
const Op_assign_plus: OPCODE = 34;
const Op_assign_minus: OPCODE = 35;
const Op_assign_exp: OPCODE = 36;
const Op_assign_concat: OPCODE = 37;
const Op_and: OPCODE = 38;
const Op_and_final: OPCODE = 39;
const Op_or: OPCODE = 40;
const Op_or_final: OPCODE = 41;
const Op_equal: OPCODE = 42;
const Op_notequal: OPCODE = 43;
const Op_less: OPCODE = 44;
const Op_greater: OPCODE = 45;
const Op_leq: OPCODE = 46;
const Op_geq: OPCODE = 47;
const Op_match: OPCODE = 48;
const Op_match_rec: OPCODE = 49;
const Op_nomatch: OPCODE = 50;
const Op_rule: OPCODE = 51;
const Op_K_case: OPCODE = 52;
const Op_K_default: OPCODE = 53;
const Op_K_break: OPCODE = 54;
const Op_K_continue: OPCODE = 55;
const Op_K_print: OPCODE = 56;
const Op_K_print_rec: OPCODE = 57;
const Op_K_printf: OPCODE = 58;
const Op_K_next: OPCODE = 59;
const Op_K_exit: OPCODE = 60;
const Op_K_return: OPCODE = 61;
const Op_K_return_from_eval: OPCODE = 62;
const Op_K_delete: OPCODE = 63;
const Op_K_delete_loop: OPCODE = 64;
const Op_K_getline_redir: OPCODE = 65;
const Op_K_getline: OPCODE = 66;
const Op_K_nextfile: OPCODE = 67;
const Op_K_namespace: OPCODE = 68;
const Op_builtin: OPCODE = 69;
const Op_sub_builtin: OPCODE = 70;
const Op_ext_builtin: OPCODE = 71;
const Op_in_array: OPCODE = 72;
const Op_func_call: OPCODE = 73;
const Op_indirect_func_call: OPCODE = 74;
const Op_push: OPCODE = 75;
const Op_push_arg: OPCODE = 76;
const Op_push_arg_untyped: OPCODE = 77;
const Op_push_i: OPCODE = 78;
const Op_push_re: OPCODE = 79;
const Op_push_array: OPCODE = 80;
const Op_push_param: OPCODE = 81;
const Op_push_lhs: OPCODE = 82;
const Op_subscript_lhs: OPCODE = 83;
const Op_field_spec_lhs: OPCODE = 84;
const Op_no_op: OPCODE = 85;
const Op_pop: OPCODE = 86;
const Op_jmp: OPCODE = 87;
const Op_jmp_true: OPCODE = 88;
const Op_jmp_false: OPCODE = 89;
const Op_get_record: OPCODE = 90;
const Op_newfile: OPCODE = 91;
const Op_arrayfor_init: OPCODE = 92;
const Op_arrayfor_incr: OPCODE = 93;
const Op_arrayfor_final: OPCODE = 94;
const Op_var_update: OPCODE = 95;
const Op_var_assign: OPCODE = 96;
const Op_field_assign: OPCODE = 97;
const Op_subscript_assign: OPCODE = 98;
const Op_after_beginfile: OPCODE = 99;
const Op_after_endfile: OPCODE = 100;
const Op_func: OPCODE = 101;
const Op_comment: OPCODE = 102;
const Op_exec_count: OPCODE = 103;
const Op_breakpoint: OPCODE = 104;
const Op_lint: OPCODE = 105;
const Op_lint_plus: OPCODE = 106;
const Op_atexit: OPCODE = 107;
const Op_stop: OPCODE = 108;
const Op_token: OPCODE = 109;
const Op_symbol: OPCODE = 110;
const Op_list: OPCODE = 111;
const Op_K_do: OPCODE = 112;
const Op_K_for: OPCODE = 113;
const Op_K_arrayfor: OPCODE = 114;
const Op_K_while: OPCODE = 115;
const Op_K_switch: OPCODE = 116;
const Op_K_if: OPCODE = 117;
const Op_K_else: OPCODE = 118;
const Op_K_function: OPCODE = 119;
const Op_cond_exp: OPCODE = 120;
const Op_parens: OPCODE = 121;
const Op_final: OPCODE = 122;

#[derive(PartialEq)]
enum scalar_cmp_t {
    SCALAR_EQ,
    SCALAR_NEQ,
    SCALAR_LT,
    SCALAR_LE,
    SCALAR_GT,
    SCALAR_GE,
}

unsafe fn grow_stack() -> *mut STACK_ITEM {
    STACK_SIZE *= 2;
    let new_size = STACK_SIZE as usize * mem::size_of::<STACK_ITEM>();
    stack_bottom = libc::realloc(stack_bottom as *mut libc::c_void, new_size) as *mut STACK_ITEM;
    stack_top = stack_bottom.add(STACK_SIZE as usize - 1);
    stack_ptr = stack_bottom.add((STACK_SIZE / 2) as usize);
    stack_ptr
}

unsafe fn push_frame(f