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
    pub type glp_file;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_row_name(P: *mut glp_prob, i: i32) -> *const i8;
    fn glp_get_col_name(P: *mut glp_prob, j: i32) -> *const i8;
    fn glp_analyze_coef(
        P: *mut glp_prob,
        k: i32,
        coef1: *mut libc::c_double,
        var1: *mut i32,
        value1: *mut libc::c_double,
        coef2: *mut libc::c_double,
        var2: *mut i32,
        value2: *mut libc::c_double,
    );
    fn glp_analyze_bound(
        P: *mut glp_prob,
        k: i32,
        value1: *mut libc::c_double,
        var1: *mut i32,
        value2: *mut libc::c_double,
        var2: *mut i32,
    );
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_bf_exists(P: *mut glp_prob) -> i32;
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_lb(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_version() -> *const i8;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
unsafe extern "C" fn format(mut buf: *mut i8, mut x: libc::c_double) -> *mut i8 {
    if x == -1.7976931348623157e+308f64 {
        strcpy(buf, b"         -Inf\0" as *const u8 as *const i8);
    } else if x == 1.7976931348623157e+308f64 {
        strcpy(buf, b"         +Inf\0" as *const u8 as *const i8);
    } else if fabs(x) <= 999999.99998f64 {
        sprintf(buf, b"%13.5f\0" as *const u8 as *const i8, x);
        if strcmp(buf as *const i8, b"      0.00000\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(buf as *const i8, b"     -0.00000\0" as *const u8 as *const i8)
                == 0 as i32
        {
            strcpy(buf, b"       .     \0" as *const u8 as *const i8);
        } else if memcmp(
            buf as *const libc::c_void,
            b"      0.\0" as *const u8 as *const i8 as *const libc::c_void,
            8 as i32 as u64,
        ) == 0 as i32
        {
            memcpy(
                buf as *mut libc::c_void,
                b"       .\0" as *const u8 as *const i8 as *const libc::c_void,
                8 as i32 as u64,
            );
        } else if memcmp(
            buf as *const libc::c_void,
            b"     -0.\0" as *const u8 as *const i8 as *const libc::c_void,
            8 as i32 as u64,
        ) == 0 as i32
        {
            memcpy(
                buf as *mut libc::c_void,
                b"      -.\0" as *const u8 as *const i8 as *const libc::c_void,
                8 as i32 as u64,
            );
        }
    } else {
        sprintf(buf, b"%13.6g\0" as *const u8 as *const i8, x);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn glp_print_ranges(
    mut P: *mut glp_prob,
    mut len: i32,
    mut list: *const i32,
    mut flags: i32,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut pass: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut numb: i32 = 0;
    let mut type_0: i32 = 0;
    let mut stat: i32 = 0;
    let mut var1: i32 = 0;
    let mut var2: i32 = 0;
    let mut count: i32 = 0;
    let mut page: i32 = 0;
    let mut ret: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut slack: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    let mut prim: libc::c_double = 0.;
    let mut dual: libc::c_double = 0.;
    let mut value1: libc::c_double = 0.;
    let mut value2: libc::c_double = 0.;
    let mut coef1: libc::c_double = 0.;
    let mut coef2: libc::c_double = 0.;
    let mut obj1: libc::c_double = 0.;
    let mut obj2: libc::c_double = 0.;
    let mut name: *const i8 = 0 as *const i8;
    let mut limit: *const i8 = 0 as *const i8;
    let mut buf: [i8; 14] = [0; 14];
    m = (*P).m;
    n = (*P).n;
    if len < 0 as i32 {
        (glp_error_(b"api/prrngs.c\0" as *const u8 as *const i8, 70 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_print_ranges: len = %d; invalid list length\n\0" as *const u8
                as *const i8,
            len,
        );
    }
    if len > 0 as i32 {
        if list.is_null() {
            (glp_error_(b"api/prrngs.c\0" as *const u8 as *const i8, 74 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_print_ranges: list = %p: invalid parameter\n\0" as *const u8
                    as *const i8,
                list,
            );
        }
        t = 1 as i32;
        while t <= len {
            k = *list.offset(t as isize);
            if !(1 as i32 <= k && k <= m + n) {
                (glp_error_(b"api/prrngs.c\0" as *const u8 as *const i8, 79 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_print_ranges: list[%d] = %d; row/column number out of range\n\0"
                        as *const u8 as *const i8,
                    t,
                    k,
                );
            }
            t += 1;
            t;
        }
    }
    if flags != 0 as i32 {
        (glp_error_(b"api/prrngs.c\0" as *const u8 as *const i8, 84 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_print_ranges: flags = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(b"api/prrngs.c\0" as *const u8 as *const i8, 87 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_print_ranges: fname = %p; invalid parameter\n\0" as *const u8
                as *const i8,
            fname,
        );
    }
    if glp_get_status(P) != 5 as i32 {
        glp_printf(
            b"glp_print_ranges: optimal basic solution required\n\0" as *const u8
                as *const i8,
        );
        ret = 1 as i32;
    } else if glp_bf_exists(P) == 0 {
        glp_printf(
            b"glp_print_ranges: basis factorization required\n\0" as *const u8
                as *const i8,
        );
        ret = 2 as i32;
    } else {
        glp_printf(
            b"Write sensitivity analysis report to '%s'...\n\0" as *const u8
                as *const i8,
            fname,
        );
        fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
        if fp.is_null() {
            glp_printf(
                b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 3 as i32;
        } else {
            count = 0 as i32;
            page = count;
            pass = 1 as i32;
            while pass <= 2 as i32 {
                t = 1 as i32;
                while t <= (if len == 0 as i32 { m + n } else { len }) {
                    if t == 1 as i32 {
                        count = 0 as i32;
                    }
                    k = if len == 0 as i32 { t } else { *list.offset(t as isize) };
                    if !(pass == 1 as i32 && k > m || pass == 2 as i32 && k <= m) {
                        if count == 0 as i32 {
                            page += 1;
                            _glp_format(
                                fp,
                                b"GLPK %-4s - SENSITIVITY ANALYSIS REPORT%73sPage%4d\n\0"
                                    as *const u8 as *const i8,
                                glp_version(),
                                b"\0" as *const u8 as *const i8,
                                page,
                            );
                            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                            _glp_format(
                                fp,
                                b"%-12s%s\n\0" as *const u8 as *const i8,
                                b"Problem:\0" as *const u8 as *const i8,
                                if ((*P).name).is_null() {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    (*P).name
                                },
                            );
                            _glp_format(
                                fp,
                                b"%-12s%s%s%.10g (%s)\n\0" as *const u8 as *const i8,
                                b"Objective:\0" as *const u8 as *const i8,
                                if ((*P).obj).is_null() {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    (*P).obj
                                },
                                if ((*P).obj).is_null() {
                                    b"\0" as *const u8 as *const i8
                                } else {
                                    b" = \0" as *const u8 as *const i8
                                },
                                (*P).obj_val,
                                if (*P).dir == 1 as i32 {
                                    b"MINimum\0" as *const u8 as *const i8
                                } else if (*P).dir == 2 as i32 {
                                    b"MAXimum\0" as *const u8 as *const i8
                                } else {
                                    b"???\0" as *const u8 as *const i8
                                },
                            );
                            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                            _glp_format(
                                fp,
                                b"%6s %-12s %2s %13s %13s %13s  %13s %13s %13s %s\n\0"
                                    as *const u8 as *const i8,
                                b"No.\0" as *const u8 as *const i8,
                                if pass == 1 as i32 {
                                    b"Row name\0" as *const u8 as *const i8
                                } else {
                                    b"Column name\0" as *const u8 as *const i8
                                },
                                b"St\0" as *const u8 as *const i8,
                                b"Activity\0" as *const u8 as *const i8,
                                if pass == 1 as i32 {
                                    b"Slack\0" as *const u8 as *const i8
                                } else {
                                    b"Obj coef\0" as *const u8 as *const i8
                                },
                                b"Lower bound\0" as *const u8 as *const i8,
                                b"Activity\0" as *const u8 as *const i8,
                                b"Obj coef\0" as *const u8 as *const i8,
                                b"Obj value at\0" as *const u8 as *const i8,
                                b"Limiting\0" as *const u8 as *const i8,
                            );
                            _glp_format(
                                fp,
                                b"%6s %-12s %2s %13s %13s %13s  %13s %13s %13s %s\n\0"
                                    as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                                b"Marginal\0" as *const u8 as *const i8,
                                b"Upper bound\0" as *const u8 as *const i8,
                                b"range\0" as *const u8 as *const i8,
                                b"range\0" as *const u8 as *const i8,
                                b"break point\0" as *const u8 as *const i8,
                                b"variable\0" as *const u8 as *const i8,
                            );
                            _glp_format(
                                fp,
                                b"------ ------------ -- ------------- ------------- -------------  ------------- ------------- ------------- ------------\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        if pass == 1 as i32 {
                            numb = k;
                            (1 as i32 <= numb && numb <= m
                                || {
                                    glp_assert_(
                                        b"1 <= numb && numb <= m\0" as *const u8 as *const i8,
                                        b"api/prrngs.c\0" as *const u8 as *const i8,
                                        140 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            row = *((*P).row).offset(numb as isize);
                            name = (*row).name;
                            type_0 = (*row).type_0;
                            lb = glp_get_row_lb(P, numb);
                            ub = glp_get_row_ub(P, numb);
                            coef = 0.0f64;
                            stat = (*row).stat;
                            prim = (*row).prim;
                            if type_0 == 1 as i32 {
                                slack = -prim;
                            } else if type_0 == 2 as i32 {
                                slack = lb - prim;
                            } else if type_0 == 3 as i32 || type_0 == 4 as i32
                                || type_0 == 5 as i32
                            {
                                slack = ub - prim;
                            }
                            dual = (*row).dual;
                        } else {
                            numb = k - m;
                            (1 as i32 <= numb && numb <= n
                                || {
                                    glp_assert_(
                                        b"1 <= numb && numb <= n\0" as *const u8 as *const i8,
                                        b"api/prrngs.c\0" as *const u8 as *const i8,
                                        159 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            col = *((*P).col).offset(numb as isize);
                            name = (*col).name;
                            lb = glp_get_col_lb(P, numb);
                            ub = glp_get_col_ub(P, numb);
                            coef = (*col).coef;
                            stat = (*col).stat;
                            prim = (*col).prim;
                            slack = 0.0f64;
                            dual = (*col).dual;
                        }
                        if stat != 1 as i32 {
                            glp_analyze_bound(
                                P,
                                k,
                                &mut value1,
                                &mut var1,
                                &mut value2,
                                &mut var2,
                            );
                            if stat == 4 as i32 {
                                coef2 = coef;
                                coef1 = coef2;
                            } else if stat == 5 as i32 {
                                coef1 = -1.7976931348623157e+308f64;
                                coef2 = 1.7976931348623157e+308f64;
                            } else if stat == 2 as i32 && (*P).dir == 1 as i32
                                || stat == 3 as i32 && (*P).dir == 2 as i32
                            {
                                coef1 = coef - dual;
                                coef2 = 1.7976931348623157e+308f64;
                            } else {
                                coef1 = -1.7976931348623157e+308f64;
                                coef2 = coef - dual;
                            }
                            if value1 == -1.7976931348623157e+308f64 {
                                if dual < -1e-9f64 {
                                    obj1 = 1.7976931348623157e+308f64;
                                } else if dual > 1e-9f64 {
                                    obj1 = -1.7976931348623157e+308f64;
                                } else {
                                    obj1 = (*P).obj_val;
                                }
                            } else {
                                obj1 = (*P).obj_val + dual * (value1 - prim);
                            }
                            if value2 == 1.7976931348623157e+308f64 {
                                if dual < -1e-9f64 {
                                    obj2 = -1.7976931348623157e+308f64;
                                } else if dual > 1e-9f64 {
                                    obj2 = 1.7976931348623157e+308f64;
                                } else {
                                    obj2 = (*P).obj_val;
                                }
                            } else {
                                obj2 = (*P).obj_val + dual * (value2 - prim);
                            }
                        } else {
                            glp_analyze_coef(
                                P,
                                k,
                                &mut coef1,
                                &mut var1,
                                &mut value1,
                                &mut coef2,
                                &mut var2,
                                &mut value2,
                            );
                            if coef1 == -1.7976931348623157e+308f64 {
                                if prim < -1e-9f64 {
                                    obj1 = 1.7976931348623157e+308f64;
                                } else if prim > 1e-9f64 {
                                    obj1 = -1.7976931348623157e+308f64;
                                } else {
                                    obj1 = (*P).obj_val;
                                }
                            } else {
                                obj1 = (*P).obj_val + (coef1 - coef) * prim;
                            }
                            if coef2 == 1.7976931348623157e+308f64 {
                                if prim < -1e-9f64 {
                                    obj2 = -1.7976931348623157e+308f64;
                                } else if prim > 1e-9f64 {
                                    obj2 = 1.7976931348623157e+308f64;
                                } else {
                                    obj2 = (*P).obj_val;
                                }
                            } else {
                                obj2 = (*P).obj_val + (coef2 - coef) * prim;
                            }
                        }
                        _glp_format(fp, b"%6d\0" as *const u8 as *const i8, numb);
                        _glp_format(
                            fp,
                            b" %-12.12s\0" as *const u8 as *const i8,
                            if name.is_null() {
                                b"\0" as *const u8 as *const i8
                            } else {
                                name
                            },
                        );
                        if !name.is_null() && strlen(name) > 12 as i32 as u64 {
                            _glp_format(
                                fp,
                                b"%s\n%6s %12s\0" as *const u8 as *const i8,
                                name.offset(12 as i32 as isize),
                                b"\0" as *const u8 as *const i8,
                                b"\0" as *const u8 as *const i8,
                            );
                        }
                        _glp_format(
                            fp,
                            b" %2s\0" as *const u8 as *const i8,
                            if stat == 1 as i32 {
                                b"BS\0" as *const u8 as *const i8
                            } else if stat == 2 as i32 {
                                b"NL\0" as *const u8 as *const i8
                            } else if stat == 3 as i32 {
                                b"NU\0" as *const u8 as *const i8
                            } else if stat == 4 as i32 {
                                b"NF\0" as *const u8 as *const i8
                            } else if stat == 5 as i32 {
                                b"NS\0" as *const u8 as *const i8
                            } else {
                                b"??\0" as *const u8 as *const i8
                            },
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), prim),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), if k <= m { slack } else { coef }),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), lb),
                        );
                        _glp_format(
                            fp,
                            b"  %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), value1),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), coef1),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), obj1),
                        );
                        if var1 != 0 as i32 {
                            if var1 <= m {
                                limit = glp_get_row_name(P, var1);
                            } else {
                                limit = glp_get_col_name(P, var1 - m);
                            }
                            if !limit.is_null() {
                                _glp_format(fp, b" %s\0" as *const u8 as *const i8, limit);
                            }
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        _glp_format(
                            fp,
                            b"%6s %-12s %2s %13s\0" as *const u8 as *const i8,
                            b"\0" as *const u8 as *const i8,
                            b"\0" as *const u8 as *const i8,
                            b"\0" as *const u8 as *const i8,
                            b"\0" as *const u8 as *const i8,
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), dual),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), ub),
                        );
                        _glp_format(
                            fp,
                            b"  %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), value2),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), coef2),
                        );
                        _glp_format(
                            fp,
                            b" %s\0" as *const u8 as *const i8,
                            format(buf.as_mut_ptr(), obj2),
                        );
                        if var2 != 0 as i32 {
                            if var2 <= m {
                                limit = glp_get_row_name(P, var2);
                            } else {
                                limit = glp_get_col_name(P, var2 - m);
                            }
                            if !limit.is_null() {
                                _glp_format(fp, b" %s\0" as *const u8 as *const i8, limit);
                            }
                        }
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        count = (count + 1 as i32) % 10 as i32;
                    }
                    t += 1;
                    t;
                }
                pass += 1;
                pass;
            }
            _glp_format(fp, b"End of report\n\0" as *const u8 as *const i8);
            if _glp_ioerr(fp) != 0 {
                glp_printf(
                    b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                    fname,
                    _glp_get_err_msg(),
                );
                ret = 4 as i32;
            } else {
                ret = 0 as i32;
            }
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}