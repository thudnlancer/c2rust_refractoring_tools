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
    fn _glp_get_err_msg() -> *const i8;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_get_num_int(P: *mut glp_prob) -> i32;
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
pub unsafe extern "C" fn glp_write_prob(
    mut P: *mut glp_prob,
    mut flags: i32,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut mip: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut ret: i32 = 0;
    if flags != 0 as i32 {
        (glp_error_(b"api/wrprob.c\0" as *const u8 as *const i8, 56 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_prob: flags = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            flags,
        );
    }
    if fname.is_null() {
        (glp_error_(b"api/wrprob.c\0" as *const u8 as *const i8, 59 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_prob: fname = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            fname,
        );
    }
    glp_printf(b"Writing problem data to '%s'...\n\0" as *const u8 as *const i8, fname);
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    count = 0 as i32;
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as i32;
    } else {
        mip = (glp_get_num_int(P) > 0 as i32) as i32;
        _glp_format(
            fp,
            b"p %s %s %d %d %d\n\0" as *const u8 as *const i8,
            (if mip == 0 {
                b"lp\0" as *const u8 as *const i8
            } else {
                b"mip\0" as *const u8 as *const i8
            }),
            (if (*P).dir == 1 as i32 {
                b"min\0" as *const u8 as *const i8
            } else {
                (if (*P).dir == 2 as i32 {
                    b"max\0" as *const u8 as *const i8
                } else {
                    b"???\0" as *const u8 as *const i8
                })
            }),
            (*P).m,
            (*P).n,
            (*P).nnz,
        );
        count += 1;
        count;
        if !((*P).name).is_null() {
            _glp_format(fp, b"n p %s\n\0" as *const u8 as *const i8, (*P).name);
            count += 1;
            count;
        }
        if !((*P).obj).is_null() {
            _glp_format(fp, b"n z %s\n\0" as *const u8 as *const i8, (*P).obj);
            count += 1;
            count;
        }
        i = 1 as i32;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            if !((*row).type_0 == 5 as i32 && (*row).lb == 0.0f64) {
                _glp_format(fp, b"i %d \0" as *const u8 as *const i8, i);
                count += 1;
                count;
                if (*row).type_0 == 1 as i32 {
                    _glp_format(fp, b"f\n\0" as *const u8 as *const i8);
                } else if (*row).type_0 == 2 as i32 {
                    _glp_format(
                        fp,
                        b"l %.*g\n\0" as *const u8 as *const i8,
                        15 as i32,
                        (*row).lb,
                    );
                } else if (*row).type_0 == 3 as i32 {
                    _glp_format(
                        fp,
                        b"u %.*g\n\0" as *const u8 as *const i8,
                        15 as i32,
                        (*row).ub,
                    );
                } else if (*row).type_0 == 4 as i32 {
                    _glp_format(
                        fp,
                        b"d %.*g %.*g\n\0" as *const u8 as *const i8,
                        15 as i32,
                        (*row).lb,
                        15 as i32,
                        (*row).ub,
                    );
                } else if (*row).type_0 == 5 as i32 {
                    _glp_format(
                        fp,
                        b"s %.*g\n\0" as *const u8 as *const i8,
                        15 as i32,
                        (*row).lb,
                    );
                } else {
                    (row != row
                        || {
                            glp_assert_(
                                b"row != row\0" as *const u8 as *const i8,
                                b"api/wrprob.c\0" as *const u8 as *const i8,
                                95 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            if !((*row).name).is_null() {
                _glp_format(
                    fp,
                    b"n i %d %s\n\0" as *const u8 as *const i8,
                    i,
                    (*row).name,
                );
                count += 1;
                count;
            }
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            if !(mip == 0 && (*col).type_0 == 2 as i32 && (*col).lb == 0.0f64) {
                if !(mip != 0 && (*col).kind == 2 as i32 && (*col).type_0 == 4 as i32
                    && (*col).lb == 0.0f64 && (*col).ub == 1.0f64)
                {
                    _glp_format(fp, b"j %d \0" as *const u8 as *const i8, j);
                    count += 1;
                    count;
                    if mip != 0 {
                        if (*col).kind == 1 as i32 {
                            _glp_format(fp, b"c \0" as *const u8 as *const i8);
                        } else if (*col).kind == 2 as i32 {
                            _glp_format(fp, b"i \0" as *const u8 as *const i8);
                        } else {
                            (col != col
                                || {
                                    glp_assert_(
                                        b"col != col\0" as *const u8 as *const i8,
                                        b"api/wrprob.c\0" as *const u8 as *const i8,
                                        114 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                    }
                    if (*col).type_0 == 1 as i32 {
                        _glp_format(fp, b"f\n\0" as *const u8 as *const i8);
                    } else if (*col).type_0 == 2 as i32 {
                        _glp_format(
                            fp,
                            b"l %.*g\n\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).lb,
                        );
                    } else if (*col).type_0 == 3 as i32 {
                        _glp_format(
                            fp,
                            b"u %.*g\n\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).ub,
                        );
                    } else if (*col).type_0 == 4 as i32 {
                        _glp_format(
                            fp,
                            b"d %.*g %.*g\n\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).lb,
                            15 as i32,
                            (*col).ub,
                        );
                    } else if (*col).type_0 == 5 as i32 {
                        _glp_format(
                            fp,
                            b"s %.*g\n\0" as *const u8 as *const i8,
                            15 as i32,
                            (*col).lb,
                        );
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const i8,
                                    b"api/wrprob.c\0" as *const u8 as *const i8,
                                    128 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
            }
            if !((*col).name).is_null() {
                _glp_format(
                    fp,
                    b"n j %d %s\n\0" as *const u8 as *const i8,
                    j,
                    (*col).name,
                );
                count += 1;
                count;
            }
            j += 1;
            j;
        }
        if (*P).c0 != 0.0f64 {
            _glp_format(
                fp,
                b"a 0 0 %.*g\n\0" as *const u8 as *const i8,
                15 as i32,
                (*P).c0,
            );
            count += 1;
            count;
        }
        j = 1 as i32;
        while j <= (*P).n {
            col = *((*P).col).offset(j as isize);
            if (*col).coef != 0.0f64 {
                _glp_format(
                    fp,
                    b"a 0 %d %.*g\n\0" as *const u8 as *const i8,
                    j,
                    15 as i32,
                    (*col).coef,
                );
                count += 1;
                count;
            }
            j += 1;
            j;
        }
        i = 1 as i32;
        while i <= (*P).m {
            row = *((*P).row).offset(i as isize);
            aij = (*row).ptr;
            while !aij.is_null() {
                _glp_format(
                    fp,
                    b"a %d %d %.*g\n\0" as *const u8 as *const i8,
                    i,
                    (*(*aij).col).j,
                    15 as i32,
                    (*aij).val,
                );
                count += 1;
                count;
                aij = (*aij).r_next;
            }
            i += 1;
            i;
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
            ret = 1 as i32;
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