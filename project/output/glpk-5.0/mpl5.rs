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
extern "C" {
    pub type AVL;
    pub type DMP;
    pub type glp_file;
    fn _glp_jday(d: i32, m: i32, y: i32) -> i32;
    fn _glp_jdate(j: i32, d: *mut i32, m: *mut i32, y: *mut i32) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn time(__timer: *mut time_t) -> time_t;
    fn _glp_xgmtime(_: *const time_t) -> *mut tm;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut i8, _: ...);
}
pub type __int32_t = i32;
pub type __time_t = i64;
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: i32,
    pub c: i32,
    pub token: i32,
    pub imlen: i32,
    pub image: *mut i8,
    pub value: libc::c_double,
    pub b_token: i32,
    pub b_imlen: i32,
    pub b_image: *mut i8,
    pub b_value: libc::c_double,
    pub f_dots: i32,
    pub f_scan: i32,
    pub f_token: i32,
    pub f_imlen: i32,
    pub f_image: *mut i8,
    pub f_value: libc::c_double,
    pub context: *mut i8,
    pub c_ptr: i32,
    pub flag_d: i32,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: i32,
    pub as_within: i32,
    pub as_in: i32,
    pub as_binary: i32,
    pub flag_s: i32,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut i8,
    pub tup_buf: *mut i8,
    pub rand: *mut RNG,
    pub flag_p: i32,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: i32,
    pub n: i32,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut i8,
    pub out_fp: *mut glp_file,
    pub out_file: *mut i8,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut i8,
    pub jump: jmp_buf,
    pub phase: i32,
    pub mod_file: *mut i8,
    pub mpl_buf: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: i32,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: i32,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: i32,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: i32,
    pub dim: i32,
    pub size: i32,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: i32,
    pub arg: OPERANDS,
    pub type_0: i32,
    pub dim: i32,
    pub up: *mut CODE,
    pub vflag: i32,
    pub valid: i32,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut i8,
    pub index: C2RustUnnamed_5,
    pub par: C2RustUnnamed_4,
    pub set: C2RustUnnamed_3,
    pub var: C2RustUnnamed_2,
    pub con: C2RustUnnamed_1,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_0,
    pub loop_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut i8,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub dimen: i32,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: i32,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [i32; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: i32,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: i32,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: i32,
    pub link: *mut libc::c_void,
    pub na: i32,
    pub arg: *mut *mut i8,
    pub nf: i32,
    pub name: *mut *mut i8,
    pub type_0: *mut i32,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: i32,
    pub type_0: i32,
    pub u: C2RustUnnamed_6,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: i32,
    pub u: C2RustUnnamed_7,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub type_0: i32,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: C2RustUnnamed_10,
    pub out: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut i8,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut i8,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut i8,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_fn_gmtime(mut mpl: *mut MPL) -> libc::c_double {
    let mut current_block: u64;
    let mut timer: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut j: i32 = 0;
    time(&mut timer);
    if timer == -(1 as i32) as time_t {
        current_block = 11346005226452049025;
    } else {
        current_block = 16658872821858055392;
    }
    loop {
        match current_block {
            11346005226452049025 => {
                _glp_mpl_error(
                    mpl,
                    b"gmtime(); unable to obtain current calendar time\0" as *const u8
                        as *const i8 as *mut i8,
                );
                current_block = 16658872821858055392;
            }
            _ => {
                tm = _glp_xgmtime(&mut timer);
                if tm.is_null() {
                    current_block = 11346005226452049025;
                    continue;
                }
                j = _glp_jday(
                    (*tm).tm_mday,
                    (*tm).tm_mon + 1 as i32,
                    1900 as i32 + (*tm).tm_year,
                );
                if j < 0 as i32 {
                    current_block = 11346005226452049025;
                } else {
                    break;
                }
            }
        }
    }
    return (((j - _glp_jday(1 as i32, 1 as i32, 1970 as i32)) as libc::c_double * 24.0f64
        + (*tm).tm_hour as libc::c_double) * 60.0f64 + (*tm).tm_min as libc::c_double)
        * 60.0f64 + (*tm).tm_sec as libc::c_double;
}
static mut week: [*mut i8; 7] = [
    b"Monday\0" as *const u8 as *const i8 as *mut i8,
    b"Tuesday\0" as *const u8 as *const i8 as *mut i8,
    b"Wednesday\0" as *const u8 as *const i8 as *mut i8,
    b"Thursday\0" as *const u8 as *const i8 as *mut i8,
    b"Friday\0" as *const u8 as *const i8 as *mut i8,
    b"Saturday\0" as *const u8 as *const i8 as *mut i8,
    b"Sunday\0" as *const u8 as *const i8 as *mut i8,
];
static mut moon: [*mut i8; 12] = [
    b"January\0" as *const u8 as *const i8 as *mut i8,
    b"February\0" as *const u8 as *const i8 as *mut i8,
    b"March\0" as *const u8 as *const i8 as *mut i8,
    b"April\0" as *const u8 as *const i8 as *mut i8,
    b"May\0" as *const u8 as *const i8 as *mut i8,
    b"June\0" as *const u8 as *const i8 as *mut i8,
    b"July\0" as *const u8 as *const i8 as *mut i8,
    b"August\0" as *const u8 as *const i8 as *mut i8,
    b"September\0" as *const u8 as *const i8 as *mut i8,
    b"October\0" as *const u8 as *const i8 as *mut i8,
    b"November\0" as *const u8 as *const i8 as *mut i8,
    b"December\0" as *const u8 as *const i8 as *mut i8,
];
unsafe extern "C" fn error1(
    mut mpl: *mut MPL,
    mut str: *const i8,
    mut s: *const i8,
    mut fmt: *const i8,
    mut f: *const i8,
    mut msg: *const i8,
) {
    glp_printf(b"Input string passed to str2time:\n\0" as *const u8 as *const i8);
    glp_printf(b"%s\n\0" as *const u8 as *const i8, str);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const i8,
        s.offset_from(str) as i64 + 1 as i32 as i64,
        b"^\0" as *const u8 as *const i8,
    );
    glp_printf(b"Format string passed to str2time:\n\0" as *const u8 as *const i8);
    glp_printf(b"%s\n\0" as *const u8 as *const i8, fmt);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const i8,
        f.offset_from(fmt) as i64 + 1 as i32 as i64,
        b"^\0" as *const u8 as *const i8,
    );
    _glp_mpl_error(mpl, b"%s\0" as *const u8 as *const i8 as *mut i8, msg);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_fn_str2time(
    mut mpl: *mut MPL,
    mut str: *const i8,
    mut fmt: *const i8,
) -> libc::c_double {
    let mut current_block: u64;
    let mut j: i32 = 0;
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hh: i32 = 0;
    let mut mm: i32 = 0;
    let mut ss: i32 = 0;
    let mut zone: i32 = 0;
    let mut s: *const i8 = 0 as *const i8;
    let mut f: *const i8 = 0 as *const i8;
    ss = -(1 as i32);
    mm = ss;
    hh = mm;
    day = hh;
    month = day;
    year = month;
    zone = 2147483647 as i32;
    s = str;
    f = fmt;
    while *f as i32 != '\0' as i32 {
        if *f as i32 == '%' as i32 {
            f = f.offset(1);
            f;
            if *f as i32 == 'b' as i32 || *f as i32 == 'h' as i32 {
                let mut k: i32 = 0;
                let mut name: *mut i8 = 0 as *mut i8;
                if month >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                let mut current_block_15: u64;
                month = 1 as i32;
                while month <= 12 as i32 {
                    name = moon[(month - 1 as i32) as usize];
                    k = 0 as i32;
                    loop {
                        if !(k <= 2 as i32) {
                            current_block_15 = 1054647088692577877;
                            break;
                        }
                        if ({
                            let mut __res: i32 = 0;
                            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                if 0 != 0 {
                                    let mut __c: i32 = *s.offset(k as isize) as u8 as i32;
                                    __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                        __c
                                    } else {
                                        *(*__ctype_toupper_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = toupper(*s.offset(k as isize) as u8 as i32);
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc())
                                    .offset(*s.offset(k as isize) as u8 as i32 as isize);
                            }
                            __res
                        })
                            != ({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = *name.offset(k as isize) as u8 as i32;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = toupper(*name.offset(k as isize) as u8 as i32);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(*name.offset(k as isize) as u8 as i32 as isize);
                                }
                                __res
                            })
                        {
                            current_block_15 = 11650488183268122163;
                            break;
                        }
                        k += 1;
                        k;
                    }
                    match current_block_15 {
                        11650488183268122163 => {
                            month += 1;
                            month;
                        }
                        _ => {
                            s = s.offset(3 as i32 as isize);
                            k = 3 as i32;
                            while *name.offset(k as isize) as i32 != '\0' as i32 {
                                if ({
                                    let mut __res: i32 = 0;
                                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                        if 0 != 0 {
                                            let mut __c: i32 = *s as u8 as i32;
                                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                                __c
                                            } else {
                                                *(*__ctype_toupper_loc()).offset(__c as isize)
                                            });
                                        } else {
                                            __res = toupper(*s as u8 as i32);
                                        }
                                    } else {
                                        __res = *(*__ctype_toupper_loc())
                                            .offset(*s as u8 as i32 as isize);
                                    }
                                    __res
                                })
                                    != ({
                                        let mut __res: i32 = 0;
                                        if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                            if 0 != 0 {
                                                let mut __c: i32 = *name.offset(k as isize) as u8 as i32;
                                                __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                                    __c
                                                } else {
                                                    *(*__ctype_toupper_loc()).offset(__c as isize)
                                                });
                                            } else {
                                                __res = toupper(*name.offset(k as isize) as u8 as i32);
                                            }
                                        } else {
                                            __res = *(*__ctype_toupper_loc())
                                                .offset(*name.offset(k as isize) as u8 as i32 as isize);
                                        }
                                        __res
                                    })
                                {
                                    break;
                                }
                                s = s.offset(1);
                                s;
                                k += 1;
                                k;
                            }
                            break;
                        }
                    }
                }
                if month > 12 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"abbreviated month name missing or invalid\0" as *const u8
                            as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'd' as i32 {
                if day >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh0 = s;
                s = s.offset(1);
                day = *fresh0 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh1 = s;
                    s = s.offset(1);
                    day = 10 as i32 * day + (*fresh1 as i32 - '0' as i32);
                }
                if !(1 as i32 <= day && day <= 31 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"day out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'H' as i32 {
                if hh >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh2 = s;
                s = s.offset(1);
                hh = *fresh2 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh3 = s;
                    s = s.offset(1);
                    hh = 10 as i32 * hh + (*fresh3 as i32 - '0' as i32);
                }
                if !(0 as i32 <= hh && hh <= 23 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"hour out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'm' as i32 {
                if month >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh4 = s;
                s = s.offset(1);
                month = *fresh4 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh5 = s;
                    s = s.offset(1);
                    month = 10 as i32 * month + (*fresh5 as i32 - '0' as i32);
                }
                if !(1 as i32 <= month && month <= 12 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"month out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'M' as i32 {
                if mm >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh6 = s;
                s = s.offset(1);
                mm = *fresh6 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh7 = s;
                    s = s.offset(1);
                    mm = 10 as i32 * mm + (*fresh7 as i32 - '0' as i32);
                }
                if !(0 as i32 <= mm && mm <= 59 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"minute out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'S' as i32 {
                if ss >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh8 = s;
                s = s.offset(1);
                ss = *fresh8 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh9 = s;
                    s = s.offset(1);
                    ss = 10 as i32 * ss + (*fresh9 as i32 - '0' as i32);
                }
                if !(0 as i32 <= ss && ss <= 60 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"second out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'y' as i32 {
                if year >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                let fresh10 = s;
                s = s.offset(1);
                year = *fresh10 as i32 - '0' as i32;
                if '0' as i32 <= *s as i32 && *s as i32 <= '9' as i32 {
                    let fresh11 = s;
                    s = s.offset(1);
                    year = 10 as i32 * year + (*fresh11 as i32 - '0' as i32);
                }
                year += if year >= 69 as i32 { 1900 as i32 } else { 2000 as i32 };
                current_block = 14916268686031723178;
            } else if *f as i32 == 'Y' as i32 {
                if year >= 0 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year multiply specified\0" as *const u8 as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year missing or invalid\0" as *const u8 as *const i8,
                    );
                }
                year = 0 as i32;
                j = 1 as i32;
                while j <= 4 as i32 {
                    if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                        break;
                    }
                    let fresh12 = s;
                    s = s.offset(1);
                    year = 10 as i32 * year + (*fresh12 as i32 - '0' as i32);
                    j += 1;
                    j;
                }
                if !(1 as i32 <= year && year <= 4000 as i32) {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"year out of range\0" as *const u8 as *const i8,
                    );
                }
                current_block = 14916268686031723178;
            } else if *f as i32 == 'z' as i32 {
                let mut z: i32 = 0;
                let mut hh_0: i32 = 0;
                let mut mm_0: i32 = 0;
                if zone != 2147483647 as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"time zone offset multiply specified\0" as *const u8
                            as *const i8,
                    );
                }
                while *s as i32 == ' ' as i32 {
                    s = s.offset(1);
                    s;
                }
                if *s as i32 == 'Z' as i32 {
                    mm_0 = 0 as i32;
                    hh_0 = mm_0;
                    z = hh_0;
                    s = s.offset(1);
                    s;
                } else {
                    if *s as i32 == '+' as i32 {
                        z = 1 as i32;
                        s = s.offset(1);
                        s;
                    } else if *s as i32 == '-' as i32 {
                        z = -(1 as i32);
                        s = s.offset(1);
                        s;
                    } else {
                        error1(
                            mpl,
                            str,
                            s,
                            fmt,
                            f,
                            b"time zone offset sign missing\0" as *const u8 as *const i8,
                        );
                    }
                    hh_0 = 0 as i32;
                    j = 1 as i32;
                    's_554: loop {
                        if j <= 2 as i32 {
                            if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                                current_block = 16146019769766336461;
                            } else {
                                current_block = 12079920068676227593;
                            }
                        } else {
                            if hh_0 > 23 as i32 {
                                current_block = 2258564000256424177;
                            } else {
                                current_block = 11718254377427810743;
                            }
                            '_err2: loop {
                                match current_block {
                                    2258564000256424177 => {
                                        error1(
                                            mpl,
                                            str,
                                            s,
                                            fmt,
                                            f,
                                            b"time zone offset value out of range\0" as *const u8
                                                as *const i8,
                                        );
                                        current_block = 11718254377427810743;
                                    }
                                    _ => {
                                        if *s as i32 == ':' as i32 {
                                            s = s.offset(1);
                                            s;
                                            if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                                                break;
                                            }
                                        }
                                        mm_0 = 0 as i32;
                                        if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                                            break 's_554;
                                        }
                                        j = 1 as i32;
                                        while j <= 2 as i32 {
                                            if !('0' as i32 <= *s as i32 && *s as i32 <= '9' as i32) {
                                                break '_err2;
                                            }
                                            let fresh14 = s;
                                            s = s.offset(1);
                                            mm_0 = 10 as i32 * mm_0 + (*fresh14 as i32 - '0' as i32);
                                            j += 1;
                                            j;
                                        }
                                        if mm_0 > 59 as i32 {
                                            current_block = 2258564000256424177;
                                        } else {
                                            break 's_554;
                                        }
                                    }
                                }
                            }
                            current_block = 16146019769766336461;
                        }
                        match current_block {
                            16146019769766336461 => {
                                error1(
                                    mpl,
                                    str,
                                    s,
                                    fmt,
                                    f,
                                    b"time zone offset value incomplete or invalid\0"
                                        as *const u8 as *const i8,
                                );
                            }
                            _ => {}
                        }
                        let fresh13 = s;
                        s = s.offset(1);
                        hh_0 = 10 as i32 * hh_0 + (*fresh13 as i32 - '0' as i32);
                        j += 1;
                        j;
                    }
                }
                zone = z * (60 as i32 * hh_0 + mm_0);
                current_block = 14916268686031723178;
            } else if *f as i32 == '%' as i32 {
                current_block = 16908504748315633968;
            } else {
                error1(
                    mpl,
                    str,
                    s,
                    fmt,
                    f,
                    b"invalid conversion specifier\0" as *const u8 as *const i8,
                );
                current_block = 14916268686031723178;
            }
        } else if *f as i32 == ' ' as i32 {
            current_block = 14916268686031723178;
        } else {
            current_block = 16908504748315633968;
        }
        match current_block {
            16908504748315633968 => {
                if *s as i32 != *f as i32 {
                    error1(
                        mpl,
                        str,
                        s,
                        fmt,
                        f,
                        b"character mismatch\0" as *const u8 as *const i8,
                    );
                }
                s = s.offset(1);
                s;
            }
            _ => {}
        }
        f = f.offset(1);
        f;
    }
    if year < 0 as i32 {
        year = 1970 as i32;
    }
    if month < 0 as i32 {
        month = 1 as i32;
    }
    if day < 0 as i32 {
        day = 1 as i32;
    }
    if hh < 0 as i32 {
        hh = 0 as i32;
    }
    if mm < 0 as i32 {
        mm = 0 as i32;
    }
    if ss < 0 as i32 {
        ss = 0 as i32;
    }
    if zone == 2147483647 as i32 {
        zone = 0 as i32;
    }
    j = _glp_jday(day, month, year);
    (j >= 0 as i32
        || {
            glp_assert_(
                b"j >= 0\0" as *const u8 as *const i8,
                b"mpl/mpl5.c\0" as *const u8 as *const i8,
                279 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return (((j - _glp_jday(1 as i32, 1 as i32, 1970 as i32)) as libc::c_double * 24.0f64
        + hh as libc::c_double) * 60.0f64 + mm as libc::c_double) * 60.0f64
        + ss as libc::c_double - 60.0f64 * zone as libc::c_double;
}
unsafe extern "C" fn error2(
    mut mpl: *mut MPL,
    mut fmt: *const i8,
    mut f: *const i8,
    mut msg: *const i8,
) {
    glp_printf(b"Format string passed to time2str:\n\0" as *const u8 as *const i8);
    glp_printf(b"%s\n\0" as *const u8 as *const i8, fmt);
    glp_printf(
        b"%*s\n\0" as *const u8 as *const i8,
        f.offset_from(fmt) as i64 + 1 as i32 as i64,
        b"^\0" as *const u8 as *const i8,
    );
    _glp_mpl_error(mpl, b"%s\0" as *const u8 as *const i8 as *mut i8, msg);
}
unsafe extern "C" fn weekday(mut j: i32) -> i32 {
    return (j + _glp_jday(1 as i32, 1 as i32, 1970 as i32)) % 7 as i32 + 1 as i32;
}
unsafe extern "C" fn firstday(mut year: i32) -> i32 {
    let mut j: i32 = 0;
    j = _glp_jday(1 as i32, 1 as i32, year) - _glp_jday(1 as i32, 1 as i32, 1970 as i32);
    match weekday(j) {
        1 => {
            j += 0 as i32;
        }
        2 => {
            j -= 1 as i32;
        }
        3 => {
            j -= 2 as i32;
        }
        4 => {
            j -= 3 as i32;
        }
        5 => {
            j += 3 as i32;
        }
        6 => {
            j += 2 as i32;
        }
        7 => {
            j += 1 as i32;
        }
        _ => {
            (j != j
                || {
                    glp_assert_(
                        b"j != j\0" as *const u8 as *const i8,
                        b"mpl/mpl5.c\0" as *const u8 as *const i8,
                        314 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    (weekday(j) == 1 as i32
        || {
            glp_assert_(
                b"weekday(j) == 1\0" as *const u8 as *const i8,
                b"mpl/mpl5.c\0" as *const u8 as *const i8,
                317 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_mpl_fn_time2str(
    mut mpl: *mut MPL,
    mut str: *mut i8,
    mut t: libc::c_double,
    mut fmt: *const i8,
) {
    let mut j: i32 = 0;
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hh: i32 = 0;
    let mut mm: i32 = 0;
    let mut ss: i32 = 0;
    let mut len: i32 = 0;
    let mut temp: libc::c_double = 0.;
    let mut f: *const i8 = 0 as *const i8;
    let mut buf: [i8; 101] = [0; 101];
    if !(-62135596800.0f64 <= t && t <= 64092211199.0f64) {
        _glp_mpl_error(
            mpl,
            b"time2str(%.*g,...); argument out of range\0" as *const u8 as *const i8
                as *mut i8,
            15 as i32,
            t,
        );
    }
    t = floor(t + 0.5f64);
    temp = fabs(t) / 86400.0f64;
    j = floor(temp) as i32;
    if t < 0.0f64 {
        if temp == floor(temp) {
            j = -j;
        } else {
            j = -(j + 1 as i32);
        }
    }
    (_glp_jdate(
        j + _glp_jday(1 as i32, 1 as i32, 1970 as i32),
        &mut day,
        &mut month,
        &mut year,
    ) == 0 as i32
        || {
            glp_assert_(
                b"jdate(j + jday(1, 1, 1970), &day, &month, &year) == 0\0" as *const u8
                    as *const i8,
                b"mpl/mpl5.c\0" as *const u8 as *const i8,
                339 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ss = (t - 86400.0f64 * j as libc::c_double) as i32;
    (0 as i32 <= ss && ss < 86400 as i32
        || {
            glp_assert_(
                b"0 <= ss && ss < 86400\0" as *const u8 as *const i8,
                b"mpl/mpl5.c\0" as *const u8 as *const i8,
                341 as i32,
            );
            1 as i32 != 0
        }) as i32;
    mm = ss / 60 as i32;
    ss %= 60 as i32;
    hh = mm / 60 as i32;
    mm %= 60 as i32;
    len = 0 as i32;
    f = fmt;
    while *f as i32 != '\0' as i32 {
        if *f as i32 == '%' as i32 {
            f = f.offset(1);
            f;
            if *f as i32 == 'a' as i32 {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    week[(weekday(j) - 1 as i32) as usize] as *const libc::c_void,
                    3 as i32 as u64,
                );
                buf[3 as i32 as usize] = '\0' as i32 as i8;
            } else if *f as i32 == 'A' as i32 {
                strcpy(buf.as_mut_ptr(), week[(weekday(j) - 1 as i32) as usize]);
            } else if *f as i32 == 'b' as i32 || *f as i32 == 'h' as i32 {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    moon[(month - 1 as i32) as usize] as *const libc::c_void,
                    3 as i32 as u64,
                );
                buf[3 as i32 as usize] = '\0' as i32 as i8;
            } else if *f as i32 == 'B' as i32 {
                strcpy(buf.as_mut_ptr(), moon[(month - 1 as i32) as usize]);
            } else if *f as i32 == 'C' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    year / 100 as i32,
                );
            } else if *f as i32 == 'd' as i32 {
                sprintf(buf.as_mut_ptr(), b"%02d\0" as *const u8 as *const i8, day);
            } else if *f as i32 == 'D' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d/%02d/%02d\0" as *const u8 as *const i8,
                    month,
                    day,
                    year % 100 as i32,
                );
            } else if *f as i32 == 'e' as i32 {
                sprintf(buf.as_mut_ptr(), b"%2d\0" as *const u8 as *const i8, day);
            } else if *f as i32 == 'F' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%04d-%02d-%02d\0" as *const u8 as *const i8,
                    year,
                    month,
                    day,
                );
            } else if *f as i32 == 'g' as i32 {
                let mut iso: i32 = 0;
                if j < firstday(year) {
                    iso = year - 1 as i32;
                } else if j < firstday(year + 1 as i32) {
                    iso = year;
                } else {
                    iso = year + 1 as i32;
                }
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    iso % 100 as i32,
                );
            } else if *f as i32 == 'G' as i32 {
                let mut iso_0: i32 = 0;
                if j < firstday(year) {
                    iso_0 = year - 1 as i32;
                } else if j < firstday(year + 1 as i32) {
                    iso_0 = year;
                } else {
                    iso_0 = year + 1 as i32;
                }
                sprintf(buf.as_mut_ptr(), b"%04d\0" as *const u8 as *const i8, iso_0);
            } else if *f as i32 == 'H' as i32 {
                sprintf(buf.as_mut_ptr(), b"%02d\0" as *const u8 as *const i8, hh);
            } else if *f as i32 == 'I' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    if hh == 0 as i32 {
                        12 as i32
                    } else if hh <= 12 as i32 {
                        hh
                    } else {
                        hh - 12 as i32
                    },
                );
            } else if *f as i32 == 'j' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%03d\0" as *const u8 as *const i8,
                    _glp_jday(day, month, year) - _glp_jday(1 as i32, 1 as i32, year)
                        + 1 as i32,
                );
            } else if *f as i32 == 'k' as i32 {
                sprintf(buf.as_mut_ptr(), b"%2d\0" as *const u8 as *const i8, hh);
            } else if *f as i32 == 'l' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%2d\0" as *const u8 as *const i8,
                    if hh == 0 as i32 {
                        12 as i32
                    } else if hh <= 12 as i32 {
                        hh
                    } else {
                        hh - 12 as i32
                    },
                );
            } else if *f as i32 == 'm' as i32 {
                sprintf(buf.as_mut_ptr(), b"%02d\0" as *const u8 as *const i8, month);
            } else if *f as i32 == 'M' as i32 {
                sprintf(buf.as_mut_ptr(), b"%02d\0" as *const u8 as *const i8, mm);
            } else if *f as i32 == 'p' as i32 {
                strcpy(
                    buf.as_mut_ptr(),
                    if hh <= 11 as i32 {
                        b"AM\0" as *const u8 as *const i8
                    } else {
                        b"PM\0" as *const u8 as *const i8
                    },
                );
            } else if *f as i32 == 'P' as i32 {
                strcpy(
                    buf.as_mut_ptr(),
                    if hh <= 11 as i32 {
                        b"am\0" as *const u8 as *const i8
                    } else {
                        b"pm\0" as *const u8 as *const i8
                    },
                );
            } else if *f as i32 == 'r' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d:%02d %s\0" as *const u8 as *const i8,
                    if hh == 0 as i32 {
                        12 as i32
                    } else if hh <= 12 as i32 {
                        hh
                    } else {
                        hh - 12 as i32
                    },
                    mm,
                    ss,
                    if hh <= 11 as i32 {
                        b"AM\0" as *const u8 as *const i8
                    } else {
                        b"PM\0" as *const u8 as *const i8
                    },
                );
            } else if *f as i32 == 'R' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d\0" as *const u8 as *const i8,
                    hh,
                    mm,
                );
            } else if *f as i32 == 'S' as i32 {
                sprintf(buf.as_mut_ptr(), b"%02d\0" as *const u8 as *const i8, ss);
            } else if *f as i32 == 'T' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d:%02d:%02d\0" as *const u8 as *const i8,
                    hh,
                    mm,
                    ss,
                );
            } else if *f as i32 == 'u' as i32 {
                sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, weekday(j));
            } else if *f as i32 == 'U' as i32 {
                let mut sun: i32 = 0;
                sun = _glp_jday(1 as i32, 1 as i32, year)
                    - _glp_jday(1 as i32, 1 as i32, 1970 as i32);
                sun += 7 as i32 - weekday(sun);
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    (j + 7 as i32 - sun) / 7 as i32,
                );
            } else if *f as i32 == 'V' as i32 {
                let mut iso_1: i32 = 0;
                if j < firstday(year) {
                    iso_1 = j - firstday(year - 1 as i32);
                } else if j < firstday(year + 1 as i32) {
                    iso_1 = j - firstday(year);
                } else {
                    iso_1 = j - firstday(year + 1 as i32);
                }
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    iso_1 / 7 as i32 + 1 as i32,
                );
            } else if *f as i32 == 'w' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const i8,
                    weekday(j) % 7 as i32,
                );
            } else if *f as i32 == 'W' as i32 {
                let mut mon: i32 = 0;
                mon = _glp_jday(1 as i32, 1 as i32, year)
                    - _glp_jday(1 as i32, 1 as i32, 1970 as i32);
                mon += (8 as i32 - weekday(mon)) % 7 as i32;
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    (j + 7 as i32 - mon) / 7 as i32,
                );
            } else if *f as i32 == 'y' as i32 {
                sprintf(
                    buf.as_mut_ptr(),
                    b"%02d\0" as *const u8 as *const i8,
                    year % 100 as i32,
                );
            } else if *f as i32 == 'Y' as i32 {
                sprintf(buf.as_mut_ptr(), b"%04d\0" as *const u8 as *const i8, year);
            } else if *f as i32 == '%' as i32 {
                buf[0 as i32 as usize] = '%' as i32 as i8;
                buf[1 as i32 as usize] = '\0' as i32 as i8;
            } else {
                error2(
                    mpl,
                    fmt,
                    f,
                    b"invalid conversion specifier\0" as *const u8 as *const i8,
                );
            }
        } else {
            buf[0 as i32 as usize] = *f;
            buf[1 as i32 as usize] = '\0' as i32 as i8;
        }
        if (len as u64).wrapping_add(strlen(buf.as_mut_ptr())) > 100 as i32 as u64 {
            _glp_mpl_error(
                mpl,
                b"time2str; output string length exceeds %d characters\0" as *const u8
                    as *const i8 as *mut i8,
                100 as i32,
            );
        }
        memcpy(
            str.offset(len as isize) as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            strlen(buf.as_mut_ptr()),
        );
        len = (len as u64).wrapping_add(strlen(buf.as_mut_ptr())) as i32 as i32;
        f = f.offset(1);
        f;
    }
    *str.offset(len as isize) = '\0' as i32 as i8;
}