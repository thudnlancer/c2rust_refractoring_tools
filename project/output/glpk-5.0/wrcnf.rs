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
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_get_err_msg() -> *const i8;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn glp_check_cnfsat(P: *mut glp_prob) -> i32;
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
pub unsafe extern "C" fn glp_write_cnfsat(
    mut P: *mut glp_prob,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut ret: i32 = 0;
    let mut s: [i8; 50] = [0; 50];
    if glp_check_cnfsat(P) != 0 as i32 {
        glp_printf(
            b"glp_write_cnfsat: problem object does not encode CNF-SAT instance\n\0"
                as *const u8 as *const i8,
        );
        ret = 1 as i32;
    } else {
        glp_printf(
            b"Writing CNF-SAT problem data to '%s'...\n\0" as *const u8 as *const i8,
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
                b"c %s\n\0" as *const u8 as *const i8,
                (if ((*P).name).is_null() {
                    b"unknown\0" as *const u8 as *const i8
                } else {
                    (*P).name
                }),
            );
            count += 1;
            count;
            _glp_format(
                fp,
                b"p cnf %d %d\n\0" as *const u8 as *const i8,
                (*P).n,
                (*P).m,
            );
            count += 1;
            count;
            i = 1 as i32;
            while i <= (*P).m {
                len = 0 as i32;
                aij = (**((*P).row).offset(i as isize)).ptr;
                while !aij.is_null() {
                    j = (*(*aij).col).j;
                    if (*aij).val < 0.0f64 {
                        j = -j;
                    }
                    sprintf(s.as_mut_ptr(), b"%d\0" as *const u8 as *const i8, j);
                    if len > 0 as i32
                        && ((len + 1 as i32) as u64).wrapping_add(strlen(s.as_mut_ptr()))
                            > 72 as i32 as u64
                    {
                        _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                        count += 1;
                        count;
                        len = 0 as i32;
                    }
                    _glp_format(
                        fp,
                        b"%s%s\0" as *const u8 as *const i8,
                        if len == 0 as i32 {
                            b"\0" as *const u8 as *const i8
                        } else {
                            b" \0" as *const u8 as *const i8
                        },
                        s.as_mut_ptr(),
                    );
                    if len > 0 as i32 {
                        len += 1;
                        len;
                    }
                    len = (len as u64).wrapping_add(strlen(s.as_mut_ptr())) as i32
                        as i32;
                    aij = (*aij).r_next;
                }
                if len > 0 as i32 && len + 1 as i32 + 1 as i32 > 72 as i32 {
                    _glp_format(fp, b"\n\0" as *const u8 as *const i8);
                    count += 1;
                    count;
                    len = 0 as i32;
                }
                _glp_format(
                    fp,
                    b"%s0\n\0" as *const u8 as *const i8,
                    (if len == 0 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b" \0" as *const u8 as *const i8
                    }),
                );
                count += 1;
                count;
                i += 1;
                i;
            }
            _glp_format(fp, b"c eof\n\0" as *const u8 as *const i8);
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
                glp_printf(
                    b"%d lines were written\n\0" as *const u8 as *const i8,
                    count,
                );
                ret = 0 as i32;
            }
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}