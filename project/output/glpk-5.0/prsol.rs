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
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strlen(_: *const i8) -> u64;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_get_err_msg() -> *const i8;
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_check_kkt(
        P: *mut glp_prob,
        sol: i32,
        cond: i32,
        ae_max: *mut libc::c_double,
        ae_ind: *mut i32,
        re_max: *mut libc::c_double,
        re_ind: *mut i32,
    );
}
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
#[no_mangle]
pub unsafe extern "C" fn glp_print_sol(
    mut P: *mut glp_prob,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut ae_ind: i32 = 0;
    let mut re_ind: i32 = 0;
    let mut ret: i32 = 0;
    let mut ae_max: libc::c_double = 0.;
    let mut re_max: libc::c_double = 0.;
    glp_printf(
        b"Writing basic solution to '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as i32;
    } else {
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
            b"%-12s%d\n\0" as *const u8 as *const i8,
            b"Rows:\0" as *const u8 as *const i8,
            (*P).m,
        );
        _glp_format(
            fp,
            b"%-12s%d\n\0" as *const u8 as *const i8,
            b"Columns:\0" as *const u8 as *const i8,
            (*P).n,
        );
        _glp_format(
            fp,
            b"%-12s%d\n\0" as *const u8 as *const i8,
            b"Non-zeros:\0" as *const u8 as *const i8,
            (*P).nnz,
        );
        t = glp_get_status(P);
        _glp_format(
            fp,
            b"%-12s%s\n\0" as *const u8 as *const i8,
            b"Status:\0" as *const u8 as *const i8,
            if t == 5 as i32 {
                b"OPTIMAL\0" as *const u8 as *const i8
            } else if t == 2 as i32 {
                b"FEASIBLE\0" as *const u8 as *const i8
            } else if t == 3 as i32 {
                b"INFEASIBLE (INTERMEDIATE)\0" as *const u8 as *const i8
            } else if t == 4 as i32 {
                b"INFEASIBLE (FINAL)\0" as *const u8 as *const i8
            } else if t == 6 as i32 {
                b"UNBOUNDED\0" as *const u8 as *const i8
            } else if t == 1 as i32 {
                b"UNDEFINED\0" as *const u8 as *const i8
            } else {
                b"???\0" as *const u8 as *const i8
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
            b"   No.   Row name   St   Activity     Lower bound   Upper bound    Marginal\n\0"
                as *const u8 as *const i8,
        );
        _glp_format(
            fp,
            b"------ ------------ -- ------------- ------------- ------------- -------------\n\0"
                as *const u8 as *const i8,
        );
        i = 1 as i32;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            _glp_format(fp, b"%6d \0" as *const u8 as *const i8, i);
            if ((*row).name).is_null() || strlen((*row).name) <= 12 as i32 as u64 {
                _glp_format(
                    fp,
                    b"%-12s \0" as *const u8 as *const i8,
                    if ((*row).name).is_null() {
                        b"\0" as *const u8 as *const i8
                    } else {
                        (*row).name
                    },
                );
            } else {
                _glp_format(
                    fp,
                    b"%s\n%20s\0" as *const u8 as *const i8,
                    (*row).name,
                    b"\0" as *const u8 as *const i8,
                );
            }
            _glp_format(
                fp,
                b"%s \0" as *const u8 as *const i8,
                if (*row).stat == 1 as i32 {
                    b"B \0" as *const u8 as *const i8
                } else if (*row).stat == 2 as i32 {
                    b"NL\0" as *const u8 as *const i8
                } else if (*row).stat == 3 as i32 {
                    b"NU\0" as *const u8 as *const i8
                } else if (*row).stat == 4 as i32 {
                    b"NF\0" as *const u8 as *const i8
                } else if (*row).stat == 5 as i32 {
                    b"NS\0" as *const u8 as *const i8
                } else {
                    b"??\0" as *const u8 as *const i8
                },
            );
            _glp_format(
                fp,
                b"%13.6g \0" as *const u8 as *const i8,
                if fabs((*row).prim) <= 1e-9f64 { 0.0f64 } else { (*row).prim },
            );
            if (*row).type_0 == 2 as i32 || (*row).type_0 == 4 as i32
                || (*row).type_0 == 5 as i32
            {
                _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*row).lb);
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const i8,
                    b"\0" as *const u8 as *const i8,
                );
            }
            if (*row).type_0 == 3 as i32 || (*row).type_0 == 4 as i32 {
                _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*row).ub);
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const i8,
                    if (*row).type_0 == 5 as i32 {
                        b"=\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                );
            }
            if (*row).stat != 1 as i32 {
                if fabs((*row).dual) <= 1e-9f64 {
                    _glp_format(
                        fp,
                        b"%13s\0" as *const u8 as *const i8,
                        b"< eps\0" as *const u8 as *const i8,
                    );
                } else {
                    _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*row).dual);
                }
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            i += 1;
            i;
        }
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        _glp_format(
            fp,
            b"   No. Column name  St   Activity     Lower bound   Upper bound    Marginal\n\0"
                as *const u8 as *const i8,
        );
        _glp_format(
            fp,
            b"------ ------------ -- ------------- ------------- ------------- -------------\n\0"
                as *const u8 as *const i8,
        );
        j = 1 as i32;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            _glp_format(fp, b"%6d \0" as *const u8 as *const i8, j);
            if ((*col).name).is_null() || strlen((*col).name) <= 12 as i32 as u64 {
                _glp_format(
                    fp,
                    b"%-12s \0" as *const u8 as *const i8,
                    if ((*col).name).is_null() {
                        b"\0" as *const u8 as *const i8
                    } else {
                        (*col).name
                    },
                );
            } else {
                _glp_format(
                    fp,
                    b"%s\n%20s\0" as *const u8 as *const i8,
                    (*col).name,
                    b"\0" as *const u8 as *const i8,
                );
            }
            _glp_format(
                fp,
                b"%s \0" as *const u8 as *const i8,
                if (*col).stat == 1 as i32 {
                    b"B \0" as *const u8 as *const i8
                } else if (*col).stat == 2 as i32 {
                    b"NL\0" as *const u8 as *const i8
                } else if (*col).stat == 3 as i32 {
                    b"NU\0" as *const u8 as *const i8
                } else if (*col).stat == 4 as i32 {
                    b"NF\0" as *const u8 as *const i8
                } else if (*col).stat == 5 as i32 {
                    b"NS\0" as *const u8 as *const i8
                } else {
                    b"??\0" as *const u8 as *const i8
                },
            );
            _glp_format(
                fp,
                b"%13.6g \0" as *const u8 as *const i8,
                if fabs((*col).prim) <= 1e-9f64 { 0.0f64 } else { (*col).prim },
            );
            if (*col).type_0 == 2 as i32 || (*col).type_0 == 4 as i32
                || (*col).type_0 == 5 as i32
            {
                _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*col).lb);
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const i8,
                    b"\0" as *const u8 as *const i8,
                );
            }
            if (*col).type_0 == 3 as i32 || (*col).type_0 == 4 as i32 {
                _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*col).ub);
            } else {
                _glp_format(
                    fp,
                    b"%13s \0" as *const u8 as *const i8,
                    if (*col).type_0 == 5 as i32 {
                        b"=\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                );
            }
            if (*col).stat != 1 as i32 {
                if fabs((*col).dual) <= 1e-9f64 {
                    _glp_format(
                        fp,
                        b"%13s\0" as *const u8 as *const i8,
                        b"< eps\0" as *const u8 as *const i8,
                    );
                } else {
                    _glp_format(fp, b"%13.6g \0" as *const u8 as *const i8, (*col).dual);
                }
            }
            _glp_format(fp, b"\n\0" as *const u8 as *const i8);
            j += 1;
            j;
        }
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        _glp_format(
            fp,
            b"Karush-Kuhn-Tucker optimality conditions:\n\0" as *const u8 as *const i8,
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        glp_check_kkt(
            P,
            1 as i32,
            1 as i32,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.PE: max.abs.err = %.2e on row %d\n\0" as *const u8 as *const i8,
            ae_max,
            ae_ind,
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on row %d\n\0" as *const u8 as *const i8,
            re_max,
            re_ind,
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const i8
            } else {
                b"PRIMAL SOLUTION IS WRONG\0" as *const u8 as *const i8
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        glp_check_kkt(
            P,
            1 as i32,
            2 as i32,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.PB: max.abs.err = %.2e on %s %d\n\0" as *const u8 as *const i8,
            ae_max,
            if ae_ind <= (*P).m {
                b"row\0" as *const u8 as *const i8
            } else {
                b"column\0" as *const u8 as *const i8
            },
            if ae_ind <= (*P).m { ae_ind } else { ae_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on %s %d\n\0" as *const u8 as *const i8,
            re_max,
            if re_ind <= (*P).m {
                b"row\0" as *const u8 as *const i8
            } else {
                b"column\0" as *const u8 as *const i8
            },
            if re_ind <= (*P).m { re_ind } else { re_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const i8
            } else {
                b"PRIMAL SOLUTION IS INFEASIBLE\0" as *const u8 as *const i8
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        glp_check_kkt(
            P,
            1 as i32,
            3 as i32,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.DE: max.abs.err = %.2e on column %d\n\0" as *const u8 as *const i8,
            ae_max,
            if ae_ind == 0 as i32 { 0 as i32 } else { ae_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on column %d\n\0" as *const u8 as *const i8,
            re_max,
            if re_ind == 0 as i32 { 0 as i32 } else { re_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const i8
            } else {
                b"DUAL SOLUTION IS WRONG\0" as *const u8 as *const i8
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        glp_check_kkt(
            P,
            1 as i32,
            4 as i32,
            &mut ae_max,
            &mut ae_ind,
            &mut re_max,
            &mut re_ind,
        );
        _glp_format(
            fp,
            b"KKT.DB: max.abs.err = %.2e on %s %d\n\0" as *const u8 as *const i8,
            ae_max,
            if ae_ind <= (*P).m {
                b"row\0" as *const u8 as *const i8
            } else {
                b"column\0" as *const u8 as *const i8
            },
            if ae_ind <= (*P).m { ae_ind } else { ae_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"        max.rel.err = %.2e on %s %d\n\0" as *const u8 as *const i8,
            re_max,
            if re_ind <= (*P).m {
                b"row\0" as *const u8 as *const i8
            } else {
                b"column\0" as *const u8 as *const i8
            },
            if re_ind <= (*P).m { re_ind } else { re_ind - (*P).m },
        );
        _glp_format(
            fp,
            b"%8s%s\n\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
            if re_max <= 1e-9f64 {
                b"High quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-6f64 {
                b"Medium quality\0" as *const u8 as *const i8
            } else if re_max <= 1e-3f64 {
                b"Low quality\0" as *const u8 as *const i8
            } else {
                b"DUAL SOLUTION IS INFEASIBLE\0" as *const u8 as *const i8
            },
        );
        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
        _glp_format(fp, b"End of output\n\0" as *const u8 as *const i8);
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as i32;
        } else {
            ret = 0 as i32;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}