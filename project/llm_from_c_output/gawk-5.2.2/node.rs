use std::ffi::{c_char, c_double, c_int, c_void};
use std::mem;
use std::ptr;
use std::cmp::Ordering;
use std::os::raw::c_long;
use std::cell::RefCell;
use std::rc::Rc;

type AWKNUM = f64;
type NODE = Node;
type wchar_t = u32; // Assuming 32-bit wide chars

struct Node {
    type_: NodeType,
    flags: u32,
    numbr: AWKNUM,
    stptr: *mut c_char,
    stlen: usize,
    stfmt: i32,
    wstptr: *mut wchar_t,
    wstlen: usize,
    valref: i32,
    // Other fields omitted for brevity
}

enum NodeType {
    NodeVal,
    NodeRegex,
    NodeElemNew,
    // Other variants omitted
}

static mut make_number: Option<fn(f64) -> *mut NODE> = None;
static mut str2number: Option<fn(*mut NODE) -> *mut NODE> = None;
static mut format_val: Option<fn(*const c_char, c_int, *mut NODE) -> *mut NODE> = None;
static mut cmp_numbers: Option<fn(*const NODE, *const NODE) -> c_int> = None;

static mut fmt_list: *mut *mut NODE = ptr::null_mut();

extern "C" {
    fn efree(ptr: *mut c_void);
    fn emalloc(size: usize, desc: *const c_char) -> *mut c_void;
    fn erealloc(ptr: *mut c_void, size: usize, desc: *const c_char) -> *mut c_void;
    fn freenode(node: *mut NODE);
    fn getnode() -> *mut NODE;
    fn make_number_node(flags: i32) -> *mut NODE;
    fn make_string(s: *const c_char, len: usize) -> *mut NODE;
    fn format_tree(fmt: *const c_char, len: usize, args: *mut *mut NODE, nargs: usize) -> *mut NODE;
    fn free_wstr(node: *mut NODE);
    fn mpfr_unset(node: *mut NODE);
    fn is_valid_character(c: c_char) -> bool;
    fn using_utf8() -> bool;
    fn do_lint() -> bool;
    fn do_posix() -> bool;
    fn do_non_decimal_data() -> bool;
    fn do_traditional() -> bool;
    fn do_mpfr() -> bool;
    fn out_of_range(node: *mut NODE) -> bool;
    fn format_nan_inf(node: *mut NODE, fmt: c_char) -> *const c_char;
    fn double_to_int(d: f64) -> f64;
    fn parse_escape(s: *mut *const c_char) -> c_int;
    fn mbrlen(s: *const c_char, n: usize, ps: *mut mbstate_t) -> usize;
    fn mbrtowc(pwc: *mut wchar_t, s: *const c_char, n: usize, ps: *mut mbstate_t) -> usize;
    fn wcrtomb(s: *mut c_char, wc: wchar_t, ps: *mut mbstate_t) -> usize;
    fn btowc(c: c_int) -> wint_t;
    fn towlower(wc: wint_t) -> wint_t;
    fn isspace(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn isxdigit(c: c_int) -> c_int;
    fn isalpha(c: c_int) -> c_int;
    fn isnan(n: f64) -> bool;
    fn signbit(n: f64) -> bool;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> f64;
    fn sqrt(n: f64) -> f64;
    fn log(n: f64) -> f64;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
}

const BUFSIZ: usize = 8192;
const LONG_MIN: c_long = c_long::MIN;
const LONG_MAX: c_long = c_long::MAX;
const STFMT_UNUSED: i32 = -1;
const MALLOC: u32 = 0x01;
const STRCUR: u32 = 0x02;
const NUMCUR: u32 = 0x04;
const STRING: u32 = 0x08;
const NUMBER: u32 = 0x10;
const USER_INPUT: u32 = 0x20;
const INTIND: u32 = 0x40;
const BOOLVAL: u32 = 0x80;
const WSTRCUR: u32 = 0x100;
const MPZN: u32 = 0x200;
const MPFN: u32 = 0x400;
const REGEX: u32 = 0x800;

static mut Nnull_string: *mut NODE = ptr::null_mut();
static mut Null_field: *mut NODE = ptr::null_mut();
static mut gawk_mb_cur_max: usize = 1;
static mut btowc_cache: [wint_t; 256] = [0; 256];
static mut do_lint_old: bool = false;

struct mbstate_t {
    // Implementation dependent
}

struct wint_t {
    // Implementation dependent
}

struct block_header {
    freep: *mut c_void,
    size: usize,
    name: *const c_char,
    active: usize,
    highwater: usize,
}

const BLOCK_MAX: usize = 2;
static mut nextfree: [block_header; BLOCK_MAX] = [
    block_header {
        freep: ptr::null_mut(),
        size: mem::size_of::<NODE>(),
        name: b"node\0".as_ptr() as *const c_char,
        active: 0,
        highwater: 0,
    },
    block_header {
        freep: ptr::null_mut(),
        size: mem::size_of::<BUCKET>(),
        name: b"bucket\0".as_ptr() as *const c_char,
        active: 0,
        highwater: 0,
    },
];

struct BUCKET {
    // Implementation dependent
}

const BLOCKCHUNK: usize = 100;

struct block_item {
    freep: *mut block_item,
}

#[no_mangle]
pub extern "C" fn r_make_number(x: c_double) -> *mut NODE {
    let r = unsafe { make_number_node(0) };
    unsafe {
        (*r).numbr = x;
    }
    r
}

#[no_mangle]
pub extern "C" fn r_force_number(n: *mut NODE) -> *mut NODE {
    unsafe {
        if (*n).type_ == NodeType::NodeElemNew {
            (*n).type_ = NodeType::NodeVal;
            (*n).flags &= !STRING;
            *(*n).stptr = b'0' as c_char;
            (*n).stlen = 1;
            return n;
        }

        if ((*n).flags & NUMCUR) != 0 {
            return n;
        }

        (*n).flags |= NUMCUR;
        (*n).numbr = 0.0;

        let mut cp = (*n).stptr;
        let mut cpend = cp.wrapping_add((*n).stlen);

        while cp < cpend && isspace(*cp as c_int) != 0 {
            cp = cp.wrapping_add(1);
        }

        if cp == cpend {
            goto_badnum(n);
            return n;
        }

        while isspace(*cpend.wrapping_sub(1) as c_int) != 0 {
            cpend = cpend.wrapping_sub(1);
        }

        if !do_posix() {
            if is_alpha(*cp as c_int) != 0 {
                goto_badnum(n);
                return n;
            } else if is_ieee_magic_val(cp) {
                if cpend == cp.wrapping_add(4) {
                    (*n).numbr = get_ieee_magic_val(cp);
                    goto_goodnum(n);
                    return n;
                } else {
                    goto_badnum(n);
                    return n;
                }
            }
        }

        if !do_posix() && (is_alpha(*cp as c_int) != 0 || (!do_non_decimal_data() && is_hex(cp, cpend)) {
            goto_badnum(n);
            return n;
        }

        if cpend.wrapping_offset_from(cp) == 1 {
            if isdigit(*cp as c_int) != 0 {
                (*n).numbr = (*cp - b'0' as c_char) as AWKNUM;
                if (*n).stlen == 1 {
                    (*n).flags |= INTIND;
                }
                goto_goodnum(n);
                return n;
            }
            goto_badnum(n);
            return n;
        }

        // Rest of the function implementation omitted for brevity
        n
    }
}

unsafe fn goto_badnum(n: *mut NODE) {
    (*n).flags &= !USER_INPUT;
}

unsafe fn goto_goodnum(n: *mut NODE) {
    if isnan((*n).numbr) && *(*n).stptr == b'-' as c_char && signbit((*n).numbr) == 0 {
        (*n).numbr = -(*n).numbr;
    }

    if ((*n).flags & USER_INPUT) != 0 {
        (*n).flags &= !STRING;
        (*n).flags |= NUMBER;
    }
}

#[no_mangle]
pub extern "C" fn is_hex(str: *const c_char, cpend: *const c_char) -> bool {
    unsafe {
        let mut s = str;
        if *s == b'-' as c_char || *s == b'+' as c_char {
            s = s.wrapping_add(1);
        }

        if s.wrapping_add(1) < cpend && *s == b'0' as c_char && (s.add(1) == b'x' as c_char || s.add(1) == b'X' as c_char) {
            return true;
        }

        false
    }
}

#[no_mangle]
pub extern "C" fn r_format_val(format: *const c_char, index: c_int, s: *mut NODE) -> *mut NODE {
    unsafe {
        let mut buf = [0 as c_char; BUFSIZ];
        let mut sp = buf.as_mut_ptr();
        let val: f64;

        if out_of_range(s) {
            let result = format_nan_inf(s, b'g' as c_char);
            return make_string(result, strlen(result));
        } else if {
            let d = double_to_int((*s).numbr);
            d != (*s).numbr || d <= LONG_MIN as f64 || d >= LONG_MAX as f64
        } {
            // Rest of the function implementation omitted for brevity
            s
        } else {
            // Rest of the function implementation omitted for brevity
            s
        }
    }
}

#[no_mangle]
pub extern "C" fn r_dupnode(n: *mut NODE) -> *mut NODE {
    unsafe {
        assert!((*n).type_ == NodeType::NodeVal);

        let r = getnode();
        *r = *n;

        r.flags |= MALLOC;
        r.valref = 1;
        r.wstptr = ptr::null_mut();
        r.wstlen = 0;

        if (n.flags & STRCUR) != 0 {
            r.stptr = emalloc(n.stlen + 1, b"r_dupnode\0".as_ptr() as *const c_char) as *mut c_char;
            memcpy(r.stptr as *mut c_void, n.stptr as *const c_void, n.stlen);
            r.stptr[n.stlen] = b'\0' as c_char;
            r.stlen = n.stlen;
            if (n.flags & WSTRCUR) != 0 {
                r.wstlen = n.wstlen;
                r.wstptr = emalloc(
                    mem::size_of::<wchar_t>() * (n.wstlen + 1),
                    b"r_dupnode\0".as_ptr() as *const c_char,
                ) as *mut wchar_t;
                memcpy(
                    r.wstptr as *mut c_void,
                    n.wstptr as *const c_void,
                    n.wstlen * mem::size_of::<wchar_t>(),
                );
                r.wstptr[n.wstlen] = 0;
                r.flags |= WSTRCUR;
            }
        }

        r
    }
}

#[no_mangle]
pub extern "C" fn cmp_awknums(t1: *const NODE, t2: *const NODE) -> c_int {
    unsafe {
        if isnan((*t1).numbr) {
            return !isnan((*t2).numbr) as c_int;
        }
        if isnan((*t2).numbr) {
            return -1;
        }
        if (*t1).numbr == (*t2).numbr {
            return 0;
        }
        if (*t1).numbr < (*t2).numbr {
            -1
        } else {
            1
        }
    }
}

// Additional function implementations omitted for brevity