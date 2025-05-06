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
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_get_err_msg() -> *const i8;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn glp_get_status(P: *mut glp_prob) -> i32;
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
#[no_mangle]
pub unsafe extern "C" fn glp_write_sol(
    mut P: *mut glp_prob,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut ret: i32 = 1 as i32;
    let mut s: *mut i8 = 0 as *mut i8;
    if fname.is_null() {
        (glp_error_(b"api/wrsol.c\0" as *const u8 as *const i8, 55 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_sol: fname = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            fname,
        );
    }
    glp_printf(
        b"Writing basic solution to '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    count = 0 as i32;
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_get_err_msg(),
        );
    } else {
        _glp_format(
            fp,
            b"c %-12s%s\n\0" as *const u8 as *const i8,
            b"Problem:\0" as *const u8 as *const i8,
            (if ((*P).name).is_null() {
                b"\0" as *const u8 as *const i8
            } else {
                (*P).name
            }),
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const i8,
            b"Rows:\0" as *const u8 as *const i8,
            (*P).m,
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const i8,
            b"Columns:\0" as *const u8 as *const i8,
            (*P).n,
        );
        count += 1;
        count;
        _glp_format(
            fp,
            b"c %-12s%d\n\0" as *const u8 as *const i8,
            b"Non-zeros:\0" as *const u8 as *const i8,
            (*P).nnz,
        );
        count += 1;
        count;
        match glp_get_status(P) {
            5 => {
                s = b"OPTIMAL\0" as *const u8 as *const i8 as *mut i8;
            }
            2 => {
                s = b"FEASIBLE\0" as *const u8 as *const i8 as *mut i8;
            }
            3 => {
                s = b"INFEASIBLE (INTERMEDIATE)\0" as *const u8 as *const i8 as *mut i8;
            }
            4 => {
                s = b"INFEASIBLE (FINAL)\0" as *const u8 as *const i8 as *mut i8;
            }
            6 => {
                s = b"UNBOUNDED\0" as *const u8 as *const i8 as *mut i8;
            }
            1 => {
                s = b"UNDEFINED\0" as *const u8 as *const i8 as *mut i8;
            }
            _ => {
                s = b"???\0" as *const u8 as *const i8 as *mut i8;
            }
        }
        _glp_format(
            fp,
            b"c %-12s%s\n\0" as *const u8 as *const i8,
            b"Status:\0" as *const u8 as *const i8,
            s,
        );
        count += 1;
        count;
        match (*P).dir {
            1 => {
                s = b"MINimum\0" as *const u8 as *const i8 as *mut i8;
            }
            2 => {
                s = b"MAXimum\0" as *const u8 as *const i8 as *mut i8;
            }
            _ => {
                s = b"???\0" as *const u8 as *const i8 as *mut i8;
            }
        }
        _glp_format(
            fp,
            b"c %-12s%s%s%.10g (%s)\n\0" as *const u8 as *const i8,
            b"Objective:\0" as *const u8 as *const i8,
            (if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const i8
            } else {
                (*P).obj
            }),
            (if ((*P).obj).is_null() {
                b"\0" as *const u8 as *const i8
            } else {
                b" = \0" as *const u8 as *const i8
            }),
            (*P).obj_val,
            s,
        );
        count += 1;
        count;
        _glp_format(fp, b"c\n\0" as *const u8 as *const i8);
        count += 1;
        count;
        _glp_format(fp, b"s bas %d %d \0" as *const u8 as *const i8, (*P).m, (*P).n);
        count += 1;
        count;
        match (*P).pbs_stat {
            1 => {
                _glp_format(fp, b"u\0" as *const u8 as *const i8);
            }
            2 => {
                _glp_format(fp, b"f\0" as *const u8 as *const i8);
            }
            3 => {
                _glp_format(fp, b"i\0" as *const u8 as *const i8);
            }
            4 => {
                _glp_format(fp, b"n\0" as *const u8 as *const i8);
            }
            _ => {
                _glp_format(fp, b"?\0" as *const u8 as *const i8);
            }
        }
        _glp_format(fp, b" \0" as *const u8 as *const i8);
        match (*P).dbs_stat {
            1 => {
                _glp_format(fp, b"u\0" as *const u8 as *const i8);
            }
            2 => {
                _glp_format(fp, b"f\0" as *const u8 as *const i8);
            }
            3 => {
                _glp_format(fp, b"i\0" as *const u8 as *const i8);
            }
            4 => {
                _glp_format(fp, b"n\0" as *const u8 as *const i8);
            }
            _ => {
                _glp_format(fp, b"?\0" as *const u8 as *const i8);
            }
        }
        _glp_format(fp, b" %.*g\n\0" as *const u8 as *const i8, 15 as i32, (*P).obj_val);
        i = 1 as i32;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            _glp_format(fp, b"i %d \0" as *const u8 as *const i8, i);
            count += 1;
            count;
            match (*row).stat {
                1 => {
                    _glp_format(fp, b"b\0" as *const u8 as *const i8);
                }
                2 => {
                    _glp_format(fp, b"l\0" as *const u8 as *const i8);
                }
                3 => {
                    _glp_format(fp, b"u\0" as *const u8 as *const i8);
                }
                4 => {
                    _glp_format(fp, b"f\0" as *const u8 as *const i8);
                }
                5 => {
                    _glp_format(fp, b"s\0" as *const u8 as *const i8);
                }
                _ => {
                    (row != row
                        || {
                            glp_assert_(
                                b"row != row\0" as *const u8 as *const i8,
                                b"api/wrsol.c\0" as *const u8 as *const i8,
                                127 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            _glp_format(
                fp,
                b" %.*g %.*g\n\0" as *const u8 as *const i8,
                15 as i32,
                (*row).prim,
                15 as i32,
                (*row).dual,
            );
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            _glp_format(fp, b"j %d \0" as *const u8 as *const i8, j);
            count += 1;
            count;
            match (*col).stat {
                1 => {
                    _glp_format(fp, b"b\0" as *const u8 as *const i8);
                }
                2 => {
                    _glp_format(fp, b"l\0" as *const u8 as *const i8);
                }
                3 => {
                    _glp_format(fp, b"u\0" as *const u8 as *const i8);
                }
                4 => {
                    _glp_format(fp, b"f\0" as *const u8 as *const i8);
                }
                5 => {
                    _glp_format(fp, b"s\0" as *const u8 as *const i8);
                }
                _ => {
                    (col != col
                        || {
                            glp_assert_(
                                b"col != col\0" as *const u8 as *const i8,
                                b"api/wrsol.c\0" as *const u8 as *const i8,
                                153 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            _glp_format(
                fp,
                b" %.*g %.*g\n\0" as *const u8 as *const i8,
                15 as i32,
                (*col).prim,
                15 as i32,
                (*col).dual,
            );
            j += 1;
            j;
        }
        _glp_format(fp, b"e o f\n\0" as *const u8 as *const i8);
        count += 1;
        count;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
        } else {
            glp_printf(b"%d lines were written\n\0" as *const u8 as *const i8, count);
            ret = 0 as i32;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}